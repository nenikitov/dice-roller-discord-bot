use std::{
    num::{NonZeroI16, NonZeroU8},
    str::FromStr,
};

use regex::Regex;

use super::*;

impl FromStr for Modifier {
    type Err = ParseModifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = Regex::new(r"^adv(\d*)$").unwrap().captures(s) {
            let count = captures
                .get(1)
                .expect("This group is always present in the capture")
                .as_str();
            let count = if count.is_empty() { "1" } else { count }
                .parse::<NonZeroU8>()
                .map_err(|e| {
                    Self::Err::new(
                        s.to_string(),
                        ModifierErrorKind::Advantage(ModifierCountError::Parse(
                            ParseNumberError::new(count.to_string(), e),
                        )),
                    )
                })?;
            Ok(Self::Advantage(count))
        } else if let Some(captures) = Regex::new(r"^dis(\d*)$").unwrap().captures(s) {
            let count = captures
                .get(1)
                .expect("This group is always present in the capture")
                .as_str();
            let count = if count.is_empty() { "1" } else { count }
                .parse::<NonZeroU8>()
                .map_err(|e| {
                    Self::Err::new(
                        s.to_string(),
                        ModifierErrorKind::Disadvantage(ModifierCountError::Parse(
                            ParseNumberError::new(count.to_string(), e),
                        )),
                    )
                })?;
            Ok(Self::Disadvantage(count))
        } else {
            Err(Self::Err::new(s.to_string(), ModifierErrorKind::Invalid))
        }
    }
}

impl FromStr for Token {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = Regex::new(r"^([+-]?\d+)$").unwrap().captures(s) {
            let number = captures
                .get(1)
                .expect("This group is always present in the capture")
                .as_str();

            match number.parse::<NonZeroI16>() {
                Ok(number) => Ok(Self::Constant(number)),
                Err(e) => Err(Self::Err::new(
                    s.to_string(),
                    TokenErrorKind::Constant(ParseConstantError::new(number.to_string(), e)),
                )),
            }
        } else if let Some(captures) = Regex::new(r"^(\d*)d(\d+)(:(.+))?$").unwrap().captures(s) {
            let count = captures
                .get(1)
                .expect("This group is always present in the capture")
                .as_str();
            let count = if count.is_empty() { "1" } else { count }
                .parse::<NonZeroU8>()
                .map_err(|e| {
                    Self::Err::new(
                        s.to_string(),
                        TokenErrorKind::Die(ParseDieError::new(
                            count.to_string(),
                            DieErrorKind::Count(e),
                        )),
                    )
                })?;

            let sides = captures
                .get(2)
                .expect("This group is always present in the capture")
                .as_str();
            let sides = sides.parse::<NonZeroU8>().map_err(|e| {
                Self::Err::new(
                    s.to_string(),
                    TokenErrorKind::Die(ParseDieError::new(
                        sides.to_string(),
                        DieErrorKind::Sides(e),
                    )),
                )
            })?;

            let modifier = captures.get(3).map(|_| {
                captures
                    .get(4)
                    .expect("This group is present in the capture if `3` is present too")
                    .as_str()
            });
            let modifier = modifier
                .map(|m| {
                    let modifier = match m.parse::<Modifier>() {
                        Err(e) => {
                            return Err(Self::Err::new(
                                s.to_string(),
                                TokenErrorKind::Die(ParseDieError::new(
                                    m.to_string(),
                                    DieErrorKind::Modifier(e),
                                )),
                            ))
                        }
                        Ok(m) => m,
                    };
                    match modifier {
                        Modifier::Advantage(take) => {
                            if take > count {
                                return Err(Self::Err::new(
                                    s.to_string(),
                                    TokenErrorKind::Die(ParseDieError::new(
                                        m.to_string(),
                                        DieErrorKind::Modifier(ParseModifierError::new(
                                            m.to_string(),
                                            ModifierErrorKind::Advantage(
                                                ModifierCountError::MoreThanDice { take, count },
                                            ),
                                        )),
                                    )),
                                ));
                            }
                        }
                        Modifier::Disadvantage(take) => {
                            if take > count {
                                return Err(Self::Err::new(
                                    s.to_string(),
                                    TokenErrorKind::Die(ParseDieError::new(
                                        m.to_string(),
                                        DieErrorKind::Modifier(ParseModifierError::new(
                                            m.to_string(),
                                            ModifierErrorKind::Disadvantage(
                                                ModifierCountError::MoreThanDice { take, count },
                                            ),
                                        )),
                                    )),
                                ));
                            }
                        }
                    }

                    Ok(modifier)
                })
                .transpose()?;

            Ok(Self::Die(
                (0..u8::from(count)).map(|_| Die::new(sides)).collect(),
                modifier,
            ))
        } else {
            Err(Self::Err::new(s.to_string(), TokenErrorKind::Invalid))
        }
    }
}

impl FromStr for Tokens {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .split_whitespace()
            .map(|token| token.parse::<Token>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(tokens))
    }
}

#[cfg(test)]
mod test {
    use std::num::{IntErrorKind, ParseIntError};

    fn parse_int_error(kind: IntErrorKind) -> ParseIntError {
        match kind {
            IntErrorKind::Empty => "".parse::<u8>().unwrap_err(),
            IntErrorKind::InvalidDigit => "a".parse::<u8>().unwrap_err(),
            IntErrorKind::PosOverflow => "1000".parse::<u8>().unwrap_err(),
            IntErrorKind::NegOverflow => "-1".parse::<u8>().unwrap_err(),
            IntErrorKind::Zero => "0".parse::<NonZeroU8>().unwrap_err(),
            _ => unreachable!(),
        }
    }

    use super::*;

    #[test]
    fn parse_5_works() {
        assert_eq!(
            "5".parse(),
            Ok(Token::Constant(NonZeroI16::new(5).unwrap()))
        );
    }

    #[test]
    fn parse_neg5_works() {
        assert_eq!(
            "-2".parse(),
            Ok(Token::Constant(NonZeroI16::new(-2).unwrap()))
        );
    }

    #[test]
    fn parse_other_fails() {
        assert_eq!(
            "other".parse::<Token>(),
            Err(ParseTokenError::new(
                "other".to_string(),
                TokenErrorKind::Invalid
            ))
        );
    }

    #[test]
    fn parse_d2_works() {
        assert_eq!(
            "d2".parse(),
            Ok(Token::Die(vec![Die::new(NonZeroU8::new(2).unwrap())], None))
        );
    }

    #[test]
    fn parse_d0_fails() {
        assert_eq!(
            "d0".parse::<Token>(),
            Err(ParseTokenError::new(
                "d0".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "0".to_string(),
                    DieErrorKind::Sides(parse_int_error(IntErrorKind::Zero))
                ))
            ))
        );
    }

    #[test]
    fn parse_3d2_works() {
        assert_eq!(
            "3d4".parse(),
            Ok(Token::Die(
                vec![
                    Die::new(NonZeroU8::new(4).unwrap()),
                    Die::new(NonZeroU8::new(4).unwrap()),
                    Die::new(NonZeroU8::new(4).unwrap())
                ],
                None
            ))
        );
    }

    #[test]
    fn parse_0d2_fails() {
        assert_eq!(
            "0d2".parse::<Token>(),
            Err(ParseTokenError::new(
                "0d2".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "0".to_string(),
                    DieErrorKind::Count(parse_int_error(IntErrorKind::Zero))
                ))
            ))
        );
    }

    #[test]
    fn parse_2d20adv_works() {
        assert_eq!(
            "2d20:adv".parse(),
            Ok(Token::Die(
                vec![
                    Die::new(NonZeroU8::new(20).unwrap()),
                    Die::new(NonZeroU8::new(20).unwrap()),
                ],
                Some(Modifier::Advantage(NonZeroU8::new(1).unwrap()))
            ))
        );
    }

    #[test]
    fn parse_4d6adv3_works() {
        assert_eq!(
            "4d6:adv3".parse(),
            Ok(Token::Die(
                vec![
                    Die::new(NonZeroU8::new(6).unwrap()),
                    Die::new(NonZeroU8::new(6).unwrap()),
                    Die::new(NonZeroU8::new(6).unwrap()),
                    Die::new(NonZeroU8::new(6).unwrap()),
                ],
                Some(Modifier::Advantage(NonZeroU8::new(3).unwrap()))
            ))
        );
    }

    #[test]
    fn parse_4d6adv0_fails() {
        assert_eq!(
            "4d6:adv0".parse::<Token>(),
            Err(ParseTokenError::new(
                "4d6:adv0".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "adv0".to_string(),
                    DieErrorKind::Modifier(ParseModifierError::new(
                        "adv0".to_string(),
                        ModifierErrorKind::Advantage(ModifierCountError::Parse(
                            ParseNumberError::new(
                                "0".to_string(),
                                parse_int_error(IntErrorKind::Zero)
                            )
                        ))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn parse_4d6adv20_fails() {
        assert_eq!(
            "4d6:adv20".parse::<Token>(),
            Err(ParseTokenError::new(
                "4d6:adv20".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "adv20".to_string(),
                    DieErrorKind::Modifier(ParseModifierError::new(
                        "adv20".to_string(),
                        ModifierErrorKind::Advantage(ModifierCountError::MoreThanDice {
                            take: NonZeroU8::new(20).unwrap(),
                            count: NonZeroU8::new(4).unwrap()
                        })
                    ))
                ))
            ))
        );
    }

    #[test]
    fn parse_2d20dis_works() {
        assert_eq!(
            "2d20:dis".parse(),
            Ok(Token::Die(
                vec![
                    Die::new(NonZeroU8::new(20).unwrap()),
                    Die::new(NonZeroU8::new(20).unwrap()),
                ],
                Some(Modifier::Disadvantage(NonZeroU8::new(1).unwrap()))
            ))
        );
    }

    #[test]
    fn parse_4d6dis3_works() {
        assert_eq!(
            "4d6:dis3".parse(),
            Ok(Token::Die(
                vec![
                    Die::new(NonZeroU8::new(6).unwrap()),
                    Die::new(NonZeroU8::new(6).unwrap()),
                    Die::new(NonZeroU8::new(6).unwrap()),
                    Die::new(NonZeroU8::new(6).unwrap()),
                ],
                Some(Modifier::Disadvantage(NonZeroU8::new(3).unwrap()))
            ))
        );
    }

    #[test]
    fn parse_4d6dis0_fails() {
        assert_eq!(
            "4d6:dis0".parse::<Token>(),
            Err(ParseTokenError::new(
                "4d6:dis0".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "dis0".to_string(),
                    DieErrorKind::Modifier(ParseModifierError::new(
                        "dis0".to_string(),
                        ModifierErrorKind::Disadvantage(ModifierCountError::Parse(
                            ParseNumberError::new(
                                "0".to_string(),
                                parse_int_error(IntErrorKind::Zero)
                            )
                        ))
                    ))
                ))
            ))
        );
    }

    #[test]
    fn parse_4d6dis20_fails() {
        assert_eq!(
            "4d6:dis20".parse::<Token>(),
            Err(ParseTokenError::new(
                "4d6:dis20".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "dis20".to_string(),
                    DieErrorKind::Modifier(ParseModifierError::new(
                        "dis20".to_string(),
                        ModifierErrorKind::Disadvantage(ModifierCountError::MoreThanDice {
                            take: NonZeroU8::new(20).unwrap(),
                            count: NonZeroU8::new(4).unwrap()
                        })
                    ))
                ))
            ))
        );
    }

    #[test]
    fn parse_4d6other_fails() {
        assert_eq!(
            "4d6:other".parse::<Token>(),
            Err(ParseTokenError::new(
                "4d6:other".to_string(),
                TokenErrorKind::Die(ParseDieError::new(
                    "other".to_string(),
                    DieErrorKind::Modifier(ParseModifierError::new(
                        "other".to_string(),
                        ModifierErrorKind::Invalid
                    ))
                ))
            ))
        );
    }
}

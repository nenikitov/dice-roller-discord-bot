use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct ParseError<Kind>
where
    Kind: Debug + Display,
{
    token: String,
    kind: Kind,
}

impl<Kind> ParseError<Kind>
where
    Kind: Debug + Display,
{
    pub fn new(token: String, kind: Kind) -> Self {
        Self { token, kind }
    }
}

impl<Kind> Display for ParseError<Kind>
where
    Kind: Debug + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing `{}`.\n{}", self.token, self.kind)
    }
}

impl<Kind> Error for ParseError<Kind> where Kind: Display + Debug {}

impl<Kind> PartialEq for ParseError<Kind>
where
    Kind: Display + Debug + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token && self.kind == other.kind
    }
}

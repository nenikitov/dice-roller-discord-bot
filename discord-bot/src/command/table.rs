use std::{fmt::Display, iter};

pub enum Span {
    Full,
    Columns(u8),
}

#[derive(Clone)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Alignment {
    fn align(&self, s: &str, width: usize) -> String {
        match self {
            Alignment::Left => format!("{s:<width$}", width = width),
            Alignment::Center => format!("{s:^width$}", width = width),
            Alignment::Right => format!("{s:>width$}", width = width),
        }
    }
}

#[derive(Clone)]
pub enum TableRow {
    Columns(Vec<(String, Alignment)>),
    FullWidth(String, Alignment),
    Separator(char),
}

pub struct Table {
    rows: Vec<TableRow>,
}

impl Table {
    pub fn new(rows: Vec<TableRow>) -> Self {
        Self { rows }
    }

    pub fn append_row(&mut self, row: &TableRow) {
        self.rows.push(row.clone());
    }

    pub fn append_rows(&mut self, rows: &[TableRow]) {
        let mut rows = rows.to_vec();
        self.rows.append(&mut rows);
    }

    pub fn append_table(&mut self, other: &Table) {
        self.append_rows(&other.rows);
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut column_widths = vec![];
        for row in &self.rows {
            if let TableRow::Columns(row) = row {
                for (c, (column, _)) in row.iter().enumerate() {
                    if column_widths.len() <= c {
                        column_widths.push(column.len());
                    } else {
                        column_widths[c] = usize::max(column_widths[c], column.len());
                    }
                }
            }
        }

        let total_width = self
            .rows
            .iter()
            .filter_map(|row| {
                if let TableRow::FullWidth(row, _) = row {
                    Some(row)
                } else {
                    None
                }
            })
            .map(|row| row.len())
            .max()
            .unwrap_or_default();

        let total_width = usize::max(
            column_widths.iter().sum::<usize>() + column_widths.len().saturating_sub(1),
            total_width,
        );

        for row in &self.rows {
            match row {
                TableRow::Columns(row) => {
                    for (c, (column, alignment)) in row.iter().enumerate() {
                        if c > 0 {
                            write!(f, " ")?;
                        }
                        write!(f, "{}", alignment.align(column, column_widths[c]))?;
                    }
                }
                TableRow::FullWidth(row, alignment) => {
                    write!(f, "{}", alignment.align(row, total_width))?;
                }
                TableRow::Separator(row) => {
                    write!(
                        f,
                        "{}",
                        iter::repeat(row).take(total_width).collect::<String>()
                    )?;
                }
            }
            writeln!(f);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use itertools::Itertools;

    use super::*;

    fn trim(s: String) -> String {
        s.lines().map(|s| s.trim_end()).join("\n")
    }

    #[test]
    fn table_works() {
        assert_eq!(
            trim(
                indoc! {r"
                         PEOPLE
                 #######################
                 Name    Age City
                 -----------------------
                 Alice    30 New York
                 Bob      20 Los Angeles
                 Charlie   5 Chicago
                 =======================
                 TOTAL    55"}
                .to_string()
            ),
            trim(
                Table::new(vec![
                    TableRow::FullWidth("PEOPLE".to_string(), Alignment::Center),
                    TableRow::Separator('#'),
                    TableRow::Columns(vec![
                        ("Name".to_string(), Alignment::Left),
                        ("Age".to_string(), Alignment::Right),
                        ("City".to_string(), Alignment::Left),
                    ]),
                    TableRow::Separator('-'),
                    TableRow::Columns(vec![
                        ("Alice".to_string(), Alignment::Left),
                        ("30".to_string(), Alignment::Right),
                        ("New York".to_string(), Alignment::Left),
                    ]),
                    TableRow::Columns(vec![
                        ("Bob".to_string(), Alignment::Left),
                        ("20".to_string(), Alignment::Right),
                        ("Los Angeles".to_string(), Alignment::Left),
                    ]),
                    TableRow::Columns(vec![
                        ("Charlie".to_string(), Alignment::Left),
                        ("5".to_string(), Alignment::Right),
                        ("Chicago".to_string(), Alignment::Left),
                    ]),
                    TableRow::Separator('='),
                    TableRow::Columns(vec![
                        ("TOTAL".to_string(), Alignment::Left),
                        ("55".to_string(), Alignment::Right),
                    ]),
                ])
                .to_string()
            )
        );
    }
}

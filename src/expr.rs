use failure::Error;
use std::str::FromStr;

pub enum Expr {
    Def(String),
    Ident(String),
    Push(f64),
    Drop,
    Clear,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl FromStr for Expr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        Ok(match s {
            | def if def.starts_with("def:") => {
                let ident = &def["def:".len()..];
                ensure!(!ident.is_empty(), "ident must not be empty");
                ensure!(!starts_with_digit(ident), "ident must not start with a digit");
                Expr::Def(ident.into())
            },
            | ident if !starts_with_digit(ident) => Expr::Ident(ident.into()),
            | n => Expr::Push(n.parse()?),
        })
    }
}

fn starts_with_digit(s: &str) -> bool {
    s.chars().next().map(|c| c.is_digit(10)).unwrap_or(false)
}

use failure::Error;
use std::str::FromStr;

pub enum Expr {
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
            | "drop"
            | "," => Expr::Drop,
            "clear" => Expr::Clear,
            "swap" => Expr::Swap,
            | "add"
            | "+" => Expr::Add,
            | "sub"
            | "-" => Expr::Sub,
            | "mul"
            | "*" => Expr::Mul,
            | "div"
            | "/" => Expr::Div,
            | "pow"
            | "^" => Expr::Pow,
            n => Expr::Push(n.parse()?),
        })
    }
}

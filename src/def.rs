use failure::Error;

use {Calc, Expr};

#[derive(Clone)]
pub enum Def {
    Value(f64),
    Fn(fn(&mut Calc) -> Result<(), Error>),
}

impl Def {
    pub fn eval(&self, calc: &mut Calc) -> Result<(), Error> {
        Ok(match *self {
            Def::Value(n) => calc.eval_expr(Expr::Push(n))?,
            Def::Fn(f) => f(calc)?,
        })
    }
}

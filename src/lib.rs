#[macro_use]
extern crate failure;

use failure::Error;

mod expr;
use self::expr::Expr;

pub struct Calc {
    stack: Vec<f64>,
}

impl Calc {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<(), Error> {
        match expr {
            Expr::Push(n) => self.stack.push(n),
            Expr::Drop => drop(self.stack.pop()),
            Expr::Clear => self.stack.clear(),
            Expr::Swap => {
                let (a, b) = self.pop2()?;
                self.stack.push(b);
                self.stack.push(a);
            },
            Expr::Add => {
                let (a, b) = self.pop2()?;
                let res = a + b;
                self.stack.push(res);
            },
            Expr::Sub => {
                let (a, b) = self.pop2()?;
                let res = a - b;
                self.stack.push(res);
            },
            Expr::Mul => {
                let (a, b) = self.pop2()?;
                let res = a * b;
                self.stack.push(res);
            },
            Expr::Div => {
                let (a, b) = self.pop2()?;
                let res = a / b;
                self.stack.push(res);
            },
            Expr::Pow => {
                let (a, b) = self.pop2()?;
                let res = a.powf(b);
                self.stack.push(res);
            },
        };

        Ok(())
    }

    fn pop2(&mut self) -> Result<(f64, f64), Error> {
        ensure!(self.stack.len() >= 2, "Need at least 2 elements");
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();

        Ok((b, a))
    }

    pub fn stack(&self) -> &[f64] {
        &self.stack
    }
}

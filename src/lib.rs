#[macro_use]
extern crate failure;

use failure::Error;
use std::collections::HashMap;

mod expr;
use self::expr::Expr;

mod def;
use self::def::Def;

pub struct Calc {
    stack: Vec<f64>,
    defs: HashMap<String, Def>,
}

impl Calc {
    pub fn new() -> Self {
        let mut calc = Self {
            stack: Vec::new(),
            defs: HashMap::new(),
        };

        calc.def_fn("drop", |calc| Ok(drop(calc.pop())));
        calc.def_fn("clear", |calc| Ok(calc.stack.clear()));
        calc.def_fn("swap", |calc| {
            let (a, b) = calc.pop2()?;
            calc.stack.push(b);
            calc.stack.push(a);
            Ok(())
        });
        calc.def_fn("+", |calc| {
            let (a, b) = calc.pop2()?;
            let res = a + b;
            calc.stack.push(res);
            Ok(())
        });
        calc.def_fn("-", |calc| {
            let (a, b) = calc.pop2()?;
            let res = a - b;
            calc.stack.push(res);
            Ok(())
        });
        calc.def_fn("*", |calc| {
            let (a, b) = calc.pop2()?;
            let res = a * b;
            calc.stack.push(res);
            Ok(())
        });
        calc.def_fn("/", |calc| {
            let (a, b) = calc.pop2()?;
            let res = a / b;
            calc.stack.push(res);
            Ok(())
        });
        calc.def_fn("^", |calc| {
            let (a, b) = calc.pop2()?;
            let res = a.powf(b);
            calc.stack.push(res);
            Ok(())
        });

        calc
    }

    pub fn eval_expr(&mut self, expr: Expr) -> Result<(), Error> {
        match expr {
            Expr::Def(ident) => {
                let value = self.pop()?;
                self.defs.insert(ident, Def::Value(value));
            },
            Expr::Ident(ident) => {
                match self.defs.get(&ident).cloned() {
                    None => bail!("No definition found for `{}`", ident),
                    Some(def) => def.eval(self)?,
                }
            },
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

    pub fn def_fn(&mut self, ident: impl Into<String>, f: fn(&mut Calc) -> Result<(), Error>) {
        self.def(ident, Def::Fn(f));
    }

    pub fn def(&mut self, ident: impl Into<String>, def: Def) {
        self.defs.insert(ident.into(), def);
    }

    pub fn pop(&mut self) -> Result<f64, Error> {
        ensure!(self.stack.len() >= 1, "Need at least 1 element");

        let x = self.stack.pop().unwrap();

        Ok(x)
    }

    pub fn pop2(&mut self) -> Result<(f64, f64), Error> {
        ensure!(self.stack.len() >= 2, "Need at least 2 elements");
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();

        Ok((b, a))
    }

    pub fn stack(&self) -> &[f64] {
        &self.stack
    }
}

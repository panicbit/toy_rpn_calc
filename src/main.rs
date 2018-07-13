extern crate stackulon;
extern crate failure;

use stackulon::Calc;
use failure::Error;
use std::io::{stdin,stdout,Write};

fn main() {
    let mut calc = Calc::new();
    let stdout = stdout();
    let stdin = stdin();

    loop {
        let mut line = String::new();

        print!("> ");
        stdout.lock().flush().unwrap();
        stdin.read_line(&mut line).unwrap();

        if let Err(e) = eval_line(&mut calc, &line) {
            println!("{}", e);
        }

        println!();
        print_stack(&calc);
        println!();
    }
}

fn eval_line(calc: &mut Calc, line: &str) -> Result<(), Error> {
    for word in line.split_whitespace() {
        let expr = word.parse()?;

        calc.eval_expr(expr)?;
    }

    Ok(())
}

fn print_stack(calc: &Calc) {
    for (i, value) in calc.stack().iter().enumerate() {
        let i = i + 1;
        println!("{} = {}", i, value);
    }
}

use std::io;
use crate::calculation::{Command, Operator};

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_command() -> Result<Command, &'static str> {
    let input = read_line();
    Command::try_from(input.as_str())
}

pub fn get_num(msg: &str) -> f64 {
    loop {
        print!("{}", msg);
        match read_line().parse::<f64>() {
            Ok(num) => break num,
            Err(_) => println!("❌ Error: insert a valid number."),
        }
    }
}

pub fn handle_zero_div(msg: &str, op: Operator) -> f64 {
    loop {
        let num = get_num(msg);
        match (op, num == 0.0) {
            (Operator::Divide, true) => {
                println!("❌ Error: can't divide by zero. Try again.");
            }           
            (_, _) => break num,
        }
    }
}

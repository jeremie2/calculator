
// main.rs

mod calculation;
mod input;

use calculation::{Calculation, Operator};

fn main() {
    println!("\n:::: CALCULATOR ::::");
    let mut history: Vec<(Calculation, f64)> = Vec::new();
    
    loop {
        print!("Select the operation (+ - * /) or a command (h = history, q = exit): ");
        let op: char = match text_io::try_read!() {
            Ok(s) => s,
            Err(_) => continue,
        };

        match op {
            'q' => {
                println!("👋 Goodbye!");
                break;
            }
            'h' => {
                println!("\n📜 ::: OPERATION HISTORY :::");
                match history.is_empty() {
                    true => println!("Hystory is empty."),
                    false => {
                        for (index, (calc, res)) in history.iter().enumerate() {
                            println!("[{}] {} = {}", index + 1, calc, res);
                        }                        
                    }
                }
                println!("---------------------------------------\n");
                continue;
            }
            '+' | '-' | '*' | '/' => {
                let n1 = input::get_num("Insert the first number: ");
                let n2 = input::handle_zero_div("Insert the second number: ", op);
                let op = Operator::try_from(op).unwrap();
                let calc = Calculation::new(n1, op, n2);
                let res = calc.execute();

                println!("\n👉 {calc} = {res}\n");
                println!("---------------------------------------");

                history.push((calc, res));
            }
            _ => {
                println!("❌ Error: invalid command or operator. Try again.");
                println!("---------------------------------------");
            }               
        }
    }
}


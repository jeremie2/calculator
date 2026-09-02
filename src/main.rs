mod calculation;
mod input;
mod history;

use calculation::{Calculation, Command};
use history::History;

fn main() {
    println!("\n:::: CALCULATOR ::::");
    let mut history = History::new();
    
    loop {
        println!("Select operation (+ - * /), history (h) or exit (q):");
        
        let command = match input::get_command() {
            Ok(cmd) => cmd,
            Err(_) => {
                println!("❌ Error: invalid command or operator. Try again.\n");
                continue;
            }
        };

        match command {
            Command::Exit => {
                println!("👋 Goodbye!");
                break;
            }
            Command::ShowHistory => history.display(),
            Command::Operation(op) => {
                let n1 = input::get_num("Insert the first number: ");
                let n2 = input::handle_zero_div("Insert the second number: ", op);
                
                let calc = Calculation::new(n1, op, n2);
                let res = calc.execute();

                println!("\n👉 {calc} = {res}\n");
                println!("---------------------------------------");

                history.add(calc, res);
            }
        }
    }
}



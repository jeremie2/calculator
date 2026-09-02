mod calculation;
mod input;
mod history;

use calculation::{Calculation, Command};
use history::History;

fn main() {
    println!("\n:::: CALCULATOR ::::");
    let mut history = History::new();
    
    loop {
        let raw_input = input::get_input(
            "Enter a calculation (e.g., 2+4), 'h' for history, or 'q' to exit:"
        );
        match Command::try_from(raw_input.as_str()) {
            Ok(Command::Exit) => {
                println!("👋 Goodbye!");
                break;
            }
            Ok(Command::ShowHistory) => {
                history.display();
                continue;
            }
            _ => {}
        }

        match raw_input.parse::<Calculation>() {
            Ok(calc) => {
                let res = calc.execute();
                println!("\n👉 {calc} = {res}\n");
                println!("---------------------------------------");
                history.add(calc, res);
            }
            Err(err_msg) => {
                println!("❌ Error: {err_msg}. Try again.");
                println!("---------------------------------------");
            }
        }
    }
}

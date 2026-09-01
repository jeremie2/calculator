
// input.rs

use text_io::try_read;

pub fn get_num(msg: &str) -> f64 {
    loop {
        println!("{msg}");
        match try_read!() {
            Ok(num) => break num,
            Err(_) => println!("❌ Error: insert a valid number."),
        }
    }
}

pub fn handle_zero_div(msg: &str, op: char) -> f64 {
    loop {
        let num = get_num(msg);
        match (op, num) {
            ('/', 0.0) => {
                println!("❌ Error: can't divide by zero. Try again.");
                continue;
            }           
            (_, _) => break num,
        }
    }
}

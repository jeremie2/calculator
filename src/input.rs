use std::io;

// Helper function
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_input(msg: &str) -> String {
    println!("{}", msg);
    read_line()
}


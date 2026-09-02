use crate::calculation::Calculation;

pub struct History {
    records: Vec<(Calculation, f64)>
}

impl History {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn add(&mut self, calc: Calculation, result: f64) {
        self.records.push((calc, result))
    }

    pub fn display(&self) {
        println!("\n📜 ::: OPERATION HISTORY :::");
        if self.records.is_empty() {
            println!("History is empty.");
        } else {
            for (index, (calc, res)) in self.records.iter().enumerate() {
                println!("[{}] {} = {}", index + 1, calc, res);
            }                        
        }
        println!("---------------------------------------\n");
    }
}

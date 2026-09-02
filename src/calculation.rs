use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum Command {
    Operation(Operator),
    ShowHistory,
    Exit,
}

impl TryFrom<&str> for Command {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "q" => Ok(Command::Exit),
            "h" => Ok(Command::ShowHistory),
            "+" => Ok(Command::Operation(Operator::Add)),
            "-" => Ok(Command::Operation(Operator::Subtract)),
            "*" => Ok(Command::Operation(Operator::Multiply)),
            "/" => Ok(Command::Operation(Operator::Divide)),
            _ => Err("❌ Unknown command")
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
        };
        write!(f, "{symbol}")
    }
}

#[derive(Clone, Copy)]
pub struct Calculation {
    pub num1: f64,
    pub operator: Operator,
    pub num2: f64,
}

impl Calculation {
    pub fn new(num1: f64, operator: Operator, num2: f64) -> Self {
        Self {num1, operator, num2}
    }

    pub fn execute(&self) -> f64 {
        match self.operator {
            Operator::Add => self.num1 + self.num2,
            Operator::Subtract => self.num1 - self.num2,
            Operator::Multiply => self.num1 * self.num2,
            Operator::Divide => self.num1 / self.num2,            
        }
    }
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.num1, self.operator, self.num2)
    }
}



// --------------------------------------------
//             UNIT TEST SECTION
// --------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_by_zero_produces_infinity() {
        let calc = Calculation::new(5.0, Operator::Divide, 0.0);
        assert_eq!(calc.execute(), std::f64::INFINITY);
    }

    #[test]
    fn test_divide_by_zero_negative_numerator() {
        let calc = Calculation::new(-5.0, Operator::Divide, 0.0);
        assert_eq!(calc.execute(), std::f64::NEG_INFINITY);
    }

    #[test]
    fn test_normal_division() {
        let calc = Calculation::new(10.0, Operator::Divide, 2.0);
        assert_eq!(calc.execute(), 5.0);
    }

    #[test]
    fn test_add() {
        let calc = Calculation::new(10.0, Operator::Add, 2.0);
        assert_eq!(calc.execute(), 12.0);
    }

    #[test]
    fn test_multiply() {
        let calc = Calculation::new(10.0, Operator::Multiply, 2.0);
        assert_eq!(calc.execute(), 20.0);
    }

    #[test]
    fn test_subtract() {
        let calc = Calculation::new(10.0, Operator::Subtract, 20.0);
        assert_eq!(calc.execute(), -10.0);
    }
    
    #[test]
    fn test_float_precisione() {
        let calc = Calculation::new(0.1, Operator::Add, 0.05);
        let tolleranza = 1e-10;
        let differenza = (calc.execute() - 0.15).abs();
        assert!(differenza < tolleranza);
    }
}

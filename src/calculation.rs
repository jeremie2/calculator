use std::fmt::{self, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum Command {
    ShowHistory,
    Exit,
}

impl TryFrom<char> for Operator {
    type Error = &'static str;
    
    // fn try_from(op: char) -> Result<Self, &'static str>
    fn try_from(op: char) -> Result<Self, Self::Error> {
        match op {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Subtract),
            '*' => Ok(Operator::Multiply),
            '/' => Ok(Operator::Divide),
            _   => Err("Unknown operator"),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = &'static str;
    
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "q" => Ok(Command::Exit),
            "h" => Ok(Command::ShowHistory),
            _   => Err("Not a system command"),
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

impl FromStr for Calculation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A string with no whitespace.
        let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
        
        // Which operator (+ - * /) inside the string
        let (split_index, op_char) = cleaned
            .chars()
            .enumerate()
            .find(|&(_, c)| c == '+' || c == '-' || c == '*' || c == '/')
            .ok_or("No valid operator found")?;
        
        // Dividing the string into:
        // - left part before the operator (num1)
        // - right part after the operator (num2)
        let num1 = &cleaned[..split_index];
        let num2 = &cleaned[split_index + 1..];

        // Convertions.
        let num1 = num1.parse::<f64>().map_err(|_| "Invalid first number")?;
        let num2 = num2.parse::<f64>().map_err(|_| "Invalid second number")?;
        let operator = Operator::try_from(op_char).map_err(|_| "Unknown operator")?;

        if operator == Operator::Divide && num2 == 0.0 {
            return Err("Division by zero is not allowed");
        }

        Ok(Calculation::new(num1, operator, num2))       
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

use colored::*;
use regex::Regex;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::f64::consts::{E, PI};
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
    Power(f64, f64),
    SquareRoot(f64),
    Sine(f64),
    Cosine(f64),
    Tangent(f64),
    Logarithm(f64),
    NaturalLog(f64),
}

fn calculate(op: Operation) -> Result<f64, String> {
    match op {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0.0 {
                Err("Division by zero!".to_string())
            } else {
                Ok(a / b)
            }
        }
        Operation::Power(a, b) => Ok(a.powf(b)),
        Operation::SquareRoot(a) => {
            if a < 0.0 {
                Err("Cannot calculate square root of negative number!".to_string())
            } else {
                Ok(a.sqrt())
            }
        }
        Operation::Sine(a) => Ok(a.to_radians().sin()),
        Operation::Cosine(a) => Ok(a.to_radians().cos()),
        Operation::Tangent(a) => Ok(a.to_radians().tan()),
        Operation::Logarithm(a) => {
            if a <= 0.0 {
                Err("Cannot calculate logarithm of non-positive number!".to_string())
            } else {
                Ok(a.log10())
            }
        }
        Operation::NaturalLog(a) => {
            if a <= 0.0 {
                Err("Cannot calculate natural logarithm of non-positive number!".to_string())
            } else {
                Ok(a.ln())
            }
        }
    }
}

fn parse_expression(input: &str) -> Result<Operation, String> {
    let input = input.to_lowercase();
    
    let input = input.replace("pi", &PI.to_string());
    let input = input.replace("e", &E.to_string());

    let basic_op_regex = Regex::new(r"^(-?\d*\.?\d+)\s*([\+\-\*/\^])\s*(-?\d*\.?\d+)$").unwrap();
    
    let func_regex = Regex::new(r"^(sqrt|sin|cos|tan|log|ln)\s*\(?(-?\d*\.?\d+)\)?$").unwrap();

    if let Some(caps) = basic_op_regex.captures(&input) {
        let a = f64::from_str(&caps[1]).map_err(|_| "Invalid first number")?;
        let b = f64::from_str(&caps[3]).map_err(|_| "Invalid second number")?;
        
        match &caps[2] {
            "+" => Ok(Operation::Add(a, b)),
            "-" => Ok(Operation::Subtract(a, b)),
            "*" => Ok(Operation::Multiply(a, b)),
            "/" => Ok(Operation::Divide(a, b)),
            "^" => Ok(Operation::Power(a, b)),
            _ => Err("Unknown operator".to_string()),
        }
    } else if let Some(caps) = func_regex.captures(&input) {
        let num = f64::from_str(&caps[2]).map_err(|_| "Invalid number")?;
        
        match &caps[1] {
            "sqrt" => Ok(Operation::SquareRoot(num)),
            "sin" => Ok(Operation::Sine(num)),
            "cos" => Ok(Operation::Cosine(num)),
            "tan" => Ok(Operation::Tangent(num)),
            "log" => Ok(Operation::Logarithm(num)),
            "ln" => Ok(Operation::NaturalLog(num)),
            _ => Err("Unknown function".to_string()),
        }
    } else {
        Err("Invalid expression format".to_string())
    }
}

fn print_help() {
    println!("{}", "\nAvailable operations:".bright_green());
    println!("  • Basic: + - * / ^");
    println!("  • Functions: sqrt, sin, cos, tan, log, ln");
    println!("  • Constants: pi, e");
    println!("\n{}", "Examples:".bright_green());
    println!("  • 2 + 2");
    println!("  • 3 * pi");
    println!("  • sin 45");
    println!("  • sqrt 16");
    println!("  • 2 ^ 3");
    println!("\n{}", "Commands:".bright_green());
    println!("  • help - Show this help message");
    println!("  • exit - Exit the calculator");
    println!();
}

fn main() {
    println!("{}", "\n=== Scientific Calculator ===".bright_blue());
    print_help();

    let mut rl = Editor::<()>::new().unwrap();

    loop {
        match rl.readline("calc> ".bright_yellow().to_string().as_str()) {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let input = line.trim();

                if input.eq_ignore_ascii_case("exit") {
                    println!("{}", "Goodbye!".bright_blue());
                    break;
                }

                if input.eq_ignore_ascii_case("help") {
                    print_help();
                    continue;
                }

                match parse_expression(input) {
                    Ok(operation) => match calculate(operation) {
                        Ok(result) => println!("{} {}", "=".bright_green(), result),
                        Err(e) => println!("{} {}", "Error:".bright_red(), e),
                    },
                    Err(e) => println!("{} {}", "Error:".bright_red(), e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
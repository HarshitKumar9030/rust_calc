use colored::*;
use regex::Regex;
use rustyline::{DefaultEditor, Editor};
use std::f64::consts::{E, PI};
use std::str::FromStr;

struct Calculator {
    memory: f64,
    history: Vec<String>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            memory: 0.0,
            history: Vec::new(),
        }
    }

    fn store_in_memory(&mut self, value: f64) {
        self.memory = value;
        println!("{}", "Value stored in memory.".bright_green());
    }

    fn add_to_memory(&mut self, value: f64) {
        self.memory += value;
        println!("{}", "Value added to memory.".bright_green());
    }

    fn recall_memory(&self) -> f64 {
        self.memory
    }

    fn clear_memory(&mut self) {
        self.memory = 0.0;
        println!("{}", "Memory cleared.".bright_green());
    }

    fn add_to_history(&mut self, expression: &str, result: f64) {
        self.history.push(format!("{} = {}", expression, result));
    }

    fn show_history(&self) {
        println!("\n{}", "Calculation History:".bright_blue());
        if self.history.is_empty() {
            println!("No calculations yet.");
        } else {
            for (i, entry) in self.history.iter().enumerate() {
                println!("{}. {}", i + 1, entry);
            }
        }
    }
}

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
    Factorial(f64),
    Absolute(f64),
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
        Operation::Factorial(a) => {
            if a < 0.0 || a.fract() != 0.0 {
                Err("Factorial only defined for non-negative integers!".to_string())
            } else {
                let n = a as u64;
                Ok((1..=n).fold(1.0, |acc, x| acc * x as f64))
            }
        }
        Operation::Absolute(a) => Ok(a.abs()),
    }
}

fn parse_expression(input: &str) -> Result<Operation, String> {
    let input = input.to_lowercase();
    
    // Handle special constants
    let input = input.replace("pi", &PI.to_string());
    let input = input.replace("e", &E.to_string());

    // Basic operations regex
    let basic_op_regex = Regex::new(r"^(-?\d*\.?\d+)\s*([\+\-\*/\^])\s*(-?\d*\.?\d+)$").unwrap();
    
    // Function regex
    let func_regex = Regex::new(r"^(sqrt|sin|cos|tan|log|ln|abs|fact)\s*\(?(-?\d*\.?\d+)\)?$").unwrap();

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
            "abs" => Ok(Operation::Absolute(num)),
            "fact" => Ok(Operation::Factorial(num)),
            _ => Err("Unknown function".to_string()),
        }
    } else {
        Err("Invalid expression format".to_string())
    }
}

fn print_help() {
    println!("{}", "\nAvailable Operations:".bright_green());
    println!("  • Basic: + - * / ^");
    println!("  • Functions: sqrt, sin, cos, tan, log, ln, abs, fact");
    println!("  • Constants: pi, e");
    
    println!("\n{}", "Memory Commands:".bright_green());
    println!("  • ms <number> - Store in memory");
    println!("  • m+ <number> - Add to memory");
    println!("  • mr - Recall from memory");
    println!("  • mc - Clear memory");
    
    println!("\n{}", "Other Commands:".bright_green());
    println!("  • help - Show this help message");
    println!("  • history - Show calculation history");
    println!("  • clear - Clear screen");
    println!("  • exit - Exit calculator");
    
    println!("\n{}", "Examples:".bright_green());
    println!("  • 2 + 2");
    println!("  • sin 45");
    println!("  • 3 * pi");
    println!("  • sqrt 16");
    println!("  • 2 ^ 3");
    println!("  • fact 5");
    println!("  • abs -4.2");
    println!();
}

fn main() {
    println!("{}", "\n=== Enhanced Scientific Calculator ===".bright_blue());
    print_help();

    let mut calc = Calculator::new();
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        match rl.readline("calc> ".bright_yellow().to_string().as_str()) {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let input = line.trim();

                match input {
                    "exit" => {
                        println!("{}", "Goodbye!".bright_blue());
                        break;
                    }
                    "help" => print_help(),
                    "clear" => print!("\x1B[2J\x1B[1;1H"),
                    "history" => calc.show_history(),
                    "mr" => println!("Memory: {}", calc.recall_memory()),
                    "mc" => calc.clear_memory(),
                    input => {
                        if input.starts_with("ms ") {
                            if let Ok(value) = input[3..].trim().parse::<f64>() {
                                calc.store_in_memory(value);
                            } else {
                                println!("{} Invalid number format", "Error:".bright_red());
                            }
                        } else if input.starts_with("m+ ") {
                            if let Ok(value) = input[3..].trim().parse::<f64>() {
                                calc.add_to_memory(value);
                            } else {
                                println!("{} Invalid number format", "Error:".bright_red());
                            }
                        } else {
                            match parse_expression(input) {
                                Ok(operation) => match calculate(operation) {
                                    Ok(result) => {
                                        println!("{} {}", "=".bright_green(), result);
                                        calc.add_to_history(input, result);
                                    }
                                    Err(e) => println!("{} {}", "Error:".bright_red(), e),
                                },
                                Err(e) => println!("{} {}", "Error:".bright_red(), e),
                            }
                        }
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
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
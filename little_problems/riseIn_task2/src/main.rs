use std::io;
enum Operation {
    Add(f64, f64),
    Substract(f64, f64),
    Division(f64, f64),
    Multiplication(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Substract(x, y) => x - y,
        Operation::Division(x, y) => x / y,
        Operation::Multiplication(x, y) => x * y,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut op = String::new();
    io::stdin().read_line(&mut op)?;
    let op: Vec<&str> = op
        .split_whitespace()
        .map(|x| x)
        .collect();
    if op.len() != 3 {
        return Err("Invalid input".into());
    }
    
    if let Some(str) = Some(op[1]) {
        let num1 = op[0].parse::<f64>()?;
        let num2 = op[2].parse::<f64>()?;
        match str.to_lowercase().as_str() {
            "add" | "+" => {
                println!("Additon = {}", calculate(Operation::Add(num1, num2)));
            }
            "substract" | "sub" | "-" => {
                println!("Substraction = {}", calculate(Operation::Substract(num1, num2)));
            }
            "multiplay" | "mul" | "*" => {
                println!("Multiplication = {}", calculate(Operation::Multiplication(num1, num2)));
            }
            "division" | "div" | "/" => {
                if num2 == 0.0 {
                    return Err("Division by zero error".into());
                }
                println!("Division = {}", calculate(Operation::Division(num1, num2)));
            },
            _ => {()}
        }
    }
    Ok(())
}

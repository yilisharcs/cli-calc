use regex::Regex;
use std::io::Write; // What are Traits?

fn parse_input(input: &str) -> (f64, char, f64) {
    // TODO: better error handling
    let re = Regex::new(r"(\d+)\s*([\+\-\*/])\s*(\d+)").unwrap();
    if let Some(captures) = re.captures(input) {
        let num1: f64 = captures[1].parse().unwrap();
        let operator: char = captures[2].chars().next().unwrap();
        let num2: f64 = captures[3].parse().unwrap();
        (num1, operator, num2)
    } else {
        panic!("Invalid input format");
    }
}

fn calculate(num1: f64, operator: char, num2: f64) -> f64 {
    match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Unsupported operator"),
    }
}

fn main() -> std::io::Result<()> {
    println!("Cli_Calc v0.1.0");

    loop {
        print!(">> ");
        // Utility of `?` vs `.unwrap()`
        std::io::stdout().flush()?; // How does this work?

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        // This can't process more elaborate user input
        // TODO: do something more efficient than regex
        let (num1, op, num2) = parse_input(input);
        let result = calculate(num1, op, num2);

        println!("{result}");
    }

    Ok(())
}

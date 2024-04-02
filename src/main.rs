use std::io;

// Define token types
#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(char),
    LeftParenthesis,
    RightParenthesis,
}

fn main() {
    println!("Welcome to the Rust Calculator CLI with BODMAS support!");

    loop {
        println!("Enter an expression (e.g., 2 + 2) or type 'quit' to exit:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim leading and trailing whitespaces
        let input = input.trim();

        if input.to_lowercase() == "quit" {
            println!("Goodbye!");
            break;
        }

        let result = evaluate_expression(input);
        match result {
            Ok(value) => println!("Result: {}", value),
            Err(error) => println!("Error: {}", error),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // Tokenize the expression
    let tokens = tokenize(expression)?;

    // Parse the tokens into a syntax tree
    let tree = parse_tokens(tokens)?;

    // Evaluate the syntax tree recursively
    let result = evaluate_tree(&tree)?;

    Ok(result)
}

// Tokenize the input expression
fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();

    for c in expression.chars() {
        match c {
            '+' | '-' | '*' | '/' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Number(buffer.parse().unwrap()));
                    buffer.clear();
                }
                tokens.push(Token::Operator(c));
            }
            '(' => {
                if !buffer.is_empty() {
                    return Err("Invalid expression format".to_string());
                }
                tokens.push(Token::LeftParenthesis);
            }
            ')' => {
                if !buffer.is_empty() {
                    tokens.push(Token::Number(buffer.parse().unwrap()));
                    buffer.clear();
                }
                tokens.push(Token::RightParenthesis);
            }
            '0'..='9' | '.' => buffer.push(c),
            ' ' => continue,
            _ => return Err("Invalid character in expression".to_string()),
        }
    }

    if !buffer.is_empty() {
        tokens.push(Token::Number(buffer.parse().unwrap()));
    }

    Ok(tokens)
}

// Parse tokens into a syntax tree
fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(op) => {
                while let Some(top_op) = operators.last() {
                    if let Token::Operator(top_char) = *top_op {
                        if precedence(op) <= precedence(top_char) {
                            output.push(operators.pop().unwrap());
                            continue;
                        }
                    }
                    break;
                }
                operators.push(Token::Operator(op));
            }
            Token::LeftParenthesis => operators.push(token),
            Token::RightParenthesis => {
                while let Some(top) = operators.pop() {
                    if let Token::LeftParenthesis = top {
                        break;
                    }
                    output.push(top);
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    Ok(output)
}

// Evaluate the syntax tree
fn evaluate_tree(tokens: &[Token]) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => stack.push(*num),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err("Invalid expression format".to_string());
                }
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                let result = match op {
                    '+' => operand1 + operand2,
                    '-' => operand1 - operand2,
                    '*' => operand1 * operand2,
                    '/' => {
                        if operand2 == 0.0 {
                            return Err("Division by zero".to_string());
                        }
                        operand1 / operand2
                    }
                    _ => return Err("Invalid operator".to_string()),
                };
                stack.push(result);
            }
            _ => return Err("Invalid token in expression".to_string()),
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression format".to_string());
    }

    Ok(stack[0])
}

// Define operator precedence
fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

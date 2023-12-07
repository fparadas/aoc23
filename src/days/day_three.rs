use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Symbol(char),
    Dot,
    NumberDigit(u8),
}

type Line = Vec<Token>;
type Matrix = Vec<Line>;

fn is_ascii_symbol(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '=' | '>' | '<' | '!' | '&' | '|' | '^' | '~' | '%' | '@' | '$'
        | '#' | '?' | ':' | ';' | ',' | '[' | ']' | '{' | '}' | '(' | ')' | '`' | '\'' | '"'
        | '\\' => true,
        _ => false,
    }
}

fn cathegorize(c: char) -> Token {
    if is_ascii_symbol(c) {
        Token::Symbol(c)
    } else if c.is_ascii_digit() {
        Token::NumberDigit(c.to_digit(10).unwrap() as u8)
    } else {
        Token::Dot
    }
}

fn parse(line: &str) -> Result<Line, &str> {
    let mut result: Line = Vec::new();

    for c in line.chars() {
        result.push(cathegorize(c));
    }

    Ok(result)
}

fn shift_sum(a: u32, b: u8) -> u32 {
    a*10 + b as u32
}

fn has_symbol(line: &Line) -> bool {
    line.iter().filter(|el| {
        match el {
            Token::Symbol(_) => true,
            _ => false,
        }
    }).count() > 0
}

fn is_near_symbol(first: Option<&Line>, second: &Line, third: Option<&Line>, index: usize) -> bool {
    let mut near_symbol: bool = false;
    let start = if index == 0 {
        0
    } else {
        index - 1
    };
    let end = if index + 1 == second.len() {
        index
    } else if index + 2 == second.len() {
        index + 1
    } else {
        index + 2
    };
    if let Some(first_line) = first {
        if has_symbol(&first_line[start..end].to_vec()) {
            near_symbol = true;
            return near_symbol;
        }
    }
    if has_symbol(&second[start..end].to_vec()) {
        near_symbol = true;
        return near_symbol;
    }
    if let Some(third_line) = third {
         if has_symbol(&third_line[start..end].to_vec()) {
            near_symbol = true;
            return near_symbol;
        }
    }
    near_symbol
}

fn get_marked_numbers(first: Option<&Line>, second: &Line, third: Option<&Line>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number: u32 = 0;
    let mut near_symbol: bool = false;

    for (i, el) in second.iter().enumerate() {
        match el {
            Token::NumberDigit(d) => {
                current_number = shift_sum(current_number, *d);
                near_symbol = near_symbol || is_near_symbol(first, second, third, i);
            }
            _ => {
                if current_number != 0 {
                    if near_symbol {
                        numbers.push(current_number);
                    }
                    current_number = 0;
                    near_symbol = false;
                }
            },
        }
        if i == second.len() - 1 {
            if current_number != 0 {
                if near_symbol {
                    numbers.push(current_number);
                }
                current_number = 0;
                near_symbol = false;
            }
        }
    }

    numbers
}

fn process_marked_numbers(matrix: &Matrix) -> Vec<u32> { 
    let mut marked_numbers: Vec<Vec<u32>> = Vec::new();

    for (i, _) in matrix.iter().enumerate() {
        let first = if i == 0 {
            None
        } else {
            Some(&matrix[i - 1])
        };
        let third = if i == matrix.len() - 1 {
            None
        } else {
            Some(&matrix[i + 1])
        };

        marked_numbers.push(get_marked_numbers(first, &matrix[i], third));
    }

    marked_numbers.iter().flatten().map(|el| *el).collect()
}

pub fn run(part: u8) -> io::Result<()> {
    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    let source: Matrix = input
        .iter()
        .enumerate()
        .map(|(i, line)| match parse(line) {
            Ok(matrix) => matrix,
            Err(msg) => {
                panic!("PARSE ERROR - Line {}: {}", i, msg);
            }
        })
        .collect();


    let marked_numbers = process_marked_numbers(&source);

    match part {
        1 => {
            println!("Part 1: {}", marked_numbers.iter().sum::<u32>());
        }
        _ => {
            println!("Part not implemented")
        }
    };

    Ok(())
}
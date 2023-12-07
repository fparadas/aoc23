use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Token {
    Symbol(char),
    Dot,
    NumberDigit(u8),
}

type Line = Vec<Token>;
type Matrix = Vec<Line>;

fn cathegorize(c: char) -> Token {
    if c.is_ascii_alphabetic() {
        Token::Symbol(c)
    } else if c.is_ascii_digit() {
        Token::NumberDigit(c.to_digit(10).unwrap() as u8)
    } else {
        Token::Dot
    }
}

fn parse(line: &str) -> Result<Line, &str> {
    Err("Not implemented")
}

fn shift_sum(a: u8, b: u8) -> u32 {
    (a*10 + b) as u32
}

fn get_marked_numbers(first: &Line, second: &Line) -> Result<Vec<u32>, &str> {

    let mut numbers: Vec<u32> = Vec::new();

    Ok(numbers)
}

fn process_marked_numbers(matrix: &Matrix) -> Result<u32, &str> { 
    let mut marked_numbers: Vec<Vec<u32>> = Vec::new();

    for (i, _) in matrix[1..].iter().enumerate() {
        marked_numbers.push(get_marked_numbers(&matrix[i - 1], &matrix[i]))
    }

    Err("Not implemented")
}

fn run(part: u8) -> Result<(), String> {
    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    let source: Matrix = input
        .iter()
        .map(|line| match parse(line) {
            Ok(matrix) => matrix,
            Err(msg) => {
                panic!("PARSE ERROR: {}", msg);
            }
        })
        .collect();

    let puzze_answer: Result<u32, &str> = match part {
        1 => {
            sum_adjecent_to_symbol(&source)
        }
        _ => {
            Err("Part not implemented")
        }
    }
    println!("{:?}", source);

    Ok(())
}

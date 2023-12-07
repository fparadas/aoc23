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
    matches!(
        c,
        '+' | '-'
            | '*'
            | '/'
            | '='
            | '>'
            | '<'
            | '!'
            | '&'
            | '|'
            | '^'
            | '~'
            | '%'
            | '@'
            | '$'
            | '#'
            | '?'
            | ':'
            | ';'
            | ','
            | '['
            | ']'
            | '{'
            | '}'
            | '('
            | ')'
            | '`'
            | '\''
            | '"'
            | '\\'
    )
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
    a * 10 + b as u32
}

fn has_symbol(line: &Line, symbol: Option<char>) -> bool {
    line.iter()
        .filter(|el| match el {
            Token::Symbol(c) if symbol.is_some() => *c == symbol.unwrap(),
            Token::Symbol(_) => true,
            _ => false,
        })
        .count()
        > 0
}

fn is_near_symbol(
    first: Option<&Line>,
    second: &Line,
    third: Option<&Line>,
    index: usize,
    symbol: Option<char>,
) -> bool {
    let mut near_symbol: bool = false;
    let start = if index == 0 { 0 } else { index - 1 };
    let end = if index + 1 == second.len() {
        index
    } else if index + 2 == second.len() {
        index + 1
    } else {
        index + 2
    };
    if let Some(first_line) = first {
        if has_symbol(&first_line[start..end].to_vec(), symbol) {
            near_symbol = true;
            return near_symbol;
        }
    }
    if has_symbol(&second[start..end].to_vec(), symbol) {
        near_symbol = true;
        return near_symbol;
    }
    if let Some(third_line) = third {
        if has_symbol(&third_line[start..end].to_vec(), symbol) {
            near_symbol = true;
            return near_symbol;
        }
    }
    near_symbol
}

fn get_marked_numbers(
    first: Option<&Line>,
    second: &Line,
    third: Option<&Line>,
    symbol: Option<char>,
) -> Vec<(u32, Vec<usize>)> {
    let mut numbers: Vec<(u32, Vec<usize>)> = Vec::new();
    let mut current_number: u32 = 0;
    let mut near_symbol: bool = false;
    let mut range: Vec<usize> = Vec::new();

    for (i, el) in second.iter().enumerate() {
        match el {
            Token::NumberDigit(d) => {
                current_number = shift_sum(current_number, *d);
                range.push(i);
                near_symbol = near_symbol || is_near_symbol(first, second, third, i, symbol);
            }
            _ => {
                if current_number != 0 {
                    if near_symbol {
                        numbers.push((current_number, range.clone()));
                    }
                    current_number = 0;
                    near_symbol = false;
                    range = Vec::new();
                }
            }
        }
        if i == second.len() - 1 && current_number != 0 {
            if near_symbol {
                numbers.push((current_number, range.clone()));
            }
            current_number = 0;
            near_symbol = false;
            range = Vec::new();
        }
    }

    numbers
}

fn process_marked_numbers(matrix: &Matrix) -> Vec<u32> {
    let mut marked_numbers: Vec<Vec<u32>> = Vec::new();

    for (i, _) in matrix.iter().enumerate() {
        let first = if i == 0 { None } else { Some(&matrix[i - 1]) };
        let third = if i == matrix.len() - 1 {
            None
        } else {
            Some(&matrix[i + 1])
        };
        let numbers = get_marked_numbers(first, &matrix[i], third, None)
            .iter()
            .map(|el| el.0)
            .collect();
        marked_numbers.push(numbers);
    }

    marked_numbers.iter().flatten().copied().collect()
}

fn get_gears(line: &Line, numbers: Vec<(u32, Vec<usize>)>) -> Vec<u32> {
    let mut gears: Vec<u32> = Vec::new();
    for (i, el) in line.iter().enumerate() {
        match el {
            Token::Symbol(c) => {
                let numbers_near_symbol: Vec<(u32, Vec<usize>)> = numbers
                    .iter()
                    .filter(|el| {
                        el.1.contains(&i) || el.1.contains(&(i - 1)) || el.1.contains(&(i + 1))
                    })
                    .cloned()
                    .collect();
                if *c == '*' && numbers_near_symbol.len() > 1 {
                    gears.push(numbers_near_symbol.iter().map(|el| el.0).product());
                }
            }
            _ => continue,
        }
    }

    gears
}

fn process_symbols(matrix: &Matrix) -> Vec<u32> {
    let mut gears: Vec<u32> = Vec::new();

    for (i, _) in matrix.iter().enumerate() {
        let first = if i == 0 { None } else { Some(&matrix[i - 1]) };
        let third = if i == matrix.len() - 1 {
            None
        } else {
            Some(&matrix[i + 1])
        };
        let mut numbers = get_marked_numbers(first, &matrix[i], third, Some('*'));
        if let Some(first_line) = first {
            numbers.extend(get_marked_numbers(
                None,
                first_line,
                Some(&matrix[i]),
                Some('*'),
            ))
        }
        if let Some(third_line) = third {
            numbers.extend(get_marked_numbers(
                Some(&matrix[i]),
                third_line,
                None,
                Some('*'),
            ))
        }
        gears.append(get_gears(&matrix[i], numbers).as_mut());
    }

    gears
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
    let gears = process_symbols(&source);
    match part {
        1 => {
            println!("Part 1: {}", marked_numbers.iter().sum::<u32>());
        }
        2 => {
            println!("Part 2: {}", gears.iter().sum::<u32>());
        }
        _ => {
            println!("Part not implemented")
        }
    };

    Ok(())
}

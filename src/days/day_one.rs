use std::io::{self, BufRead};

pub fn run(part: u8) -> io::Result<()> {
    let stdin = io::stdin();
    let mut total: u32 = 0;

    match part {
        1 => {
            for line in stdin.lock().lines() {
                let numbers: Vec<u32> = line?
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| (c as u8 - '0' as u8) as u32)
                    .collect();

                total += match (numbers.first(), numbers.last()) {
                    (Some(first), Some(last)) => first * 10 + last,
                    _ => {
                        0;
                        break;
                    }
                }
            }
        }
        2 => {
            for line in stdin.lock().lines() {
                let numbers: Vec<u32> = parse(&line?).unwrap_or(vec![]);
                total += match (numbers.first(), numbers.last()) {
                    (Some(first), Some(last)) => first * 10 + last,
                    _ => {
                        0;
                        break;
                    }
                }
            }
        }
        _ => {
            println!("Part not implemented");
            return Ok(());
        }
    }

    println!("The answer you wish is {}", total);
    Ok(())
}

fn parse(input: &str) -> Result<Vec<u32>, ()> {
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut numbers: Vec<u32> = Vec::new();
    let mut current_word = String::new();

    for c in input.chars() {
        if c.is_ascii_digit() {
            numbers.push((c as u8 - '0' as u8) as u32);
            if !current_word.is_empty() {
                if let Some(&(_, number)) = number_words
                    .iter()
                    .find(|&&(word, _)| current_word.contains(word))
                {
                    numbers.push(number);
                }
                current_word = current_word.chars().last().unwrap().to_string();
            }
        } else if c.is_ascii_alphabetic() {
            current_word.push(c);
            if let Some(&(_, number)) = number_words
                .iter()
                .find(|&&(word, _)| current_word.contains(word))
            {
                numbers.push(number);
                current_word = current_word.chars().last().unwrap().to_string();
            }
        }
    }

    if numbers.is_empty() {
        Err(())
    } else {
        Ok(numbers)
    }
}

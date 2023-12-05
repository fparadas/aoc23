use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    id: u32,
    colors: Vec<HashMap<Color, u32>>,
}

fn parse(line: &str) -> Result<Game, &str> {
    if line.is_empty() {
        return Err(line);
    }

    let mut splitted = line.split(':');
    let id = splitted
        .nth(0)
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let colors = splitted
        .nth(0)
        .unwrap()
        .split(';')
        .map(|input| {
            let mut map = HashMap::new();

            for color in input.split(',') {
                let mut color: std::str::SplitWhitespace<'_> = color.split_whitespace();
                let count = color.next().unwrap().parse::<u32>().unwrap();
                let color = match color.next().unwrap() {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => panic!("Invalid color"),
                };

                map.insert(color, count);
            }

            map
        })
        .collect();

    Ok(Game { id, colors })
}

fn is_possible(game: &Game, nreds: u32, ngreens: u32, nblues: u32) -> bool {
    let color_counts = [
        (Color::Red, nreds),
        (Color::Green, ngreens),
        (Color::Blue, nblues),
    ];

    for color in &game.colors {
        for &(color_variant, count) in &color_counts {
            if *color.get(&color_variant).unwrap_or(&0) > count {
                return false;
            }
        }
    }

    true
}

pub fn run(part: u8) -> io::Result<()> {
    let stdin = io::stdin();
    let mut total: u32 = 0;
    let mut possible: Vec<u32> = vec![];

    match part {
        1 => {
            for line in stdin.lock().lines() {
                let input = line?;
                let game = parse(&input);
                match game {
                    Ok(game) => {
                        if is_possible(&game, 12, 13, 14) {
                            possible.push(game.id);
                            total += game.id;
                        }
                    }
                    Err(msg) => {
                        println!("Stopped {msg}");
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

    println!("Possible: {} {:?}", possible.len(), possible);
    println!("The answer you wish is {}", total);
    Ok(())
}

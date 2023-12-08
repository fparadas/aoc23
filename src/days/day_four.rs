use std::{io::{self, BufRead}, iter::repeat};

fn parse_card(card: &str) -> (Vec<u32>, Vec<u32>) {
    let hand = card
        .split(':')
        .nth(1)
        .unwrap()
        .split('|')
        .map(|x| {
            x.trim()
                .split(' ')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    (hand[0].clone(), hand[1].clone())
}

fn calculate(winning_list: Vec<u32>) -> u64 {
    f64::floor(2.0_f64.powf(winning_list.len() as f64 - 1.0)) as u64
}

fn intersection(vec1: &[u32], vec2: &[u32]) -> Vec<u32> {
    vec1.iter().filter(|x| vec2.contains(x)).cloned().collect()
}

fn part_one(game: Vec<(Vec<u32>, Vec<u32>)>) -> u64 {
    game.iter()
        .map(|x| calculate(intersection(&x.0, &x.1)))
        .sum()
}

fn draw_cards(game: Vec<(Vec<u32>, Vec<u32>)>) -> Vec<u64> {
    let mut copies = repeat(1).take(game.len()).collect::<Vec<u64>>();

    game.iter()
        .enumerate()
        .for_each(|(i, x)| {
            if i == game.len() - 1 {
                return;
            }
            let intersection = intersection(&x.0, &x.1);
            for j in 0..intersection.len() {
                copies[i + j + 1] += copies[i];
            }
        });
    
    copies
}

fn part_two(game: Vec<(Vec<u32>, Vec<u32>)>) -> u64 {
    draw_cards(game).iter().sum()
}

pub fn run(part: u8) -> io::Result<()> {
    let game: Vec<(Vec<u32>, Vec<u32>)> = io::stdin()
        .lock()
        .lines()
        .map(|x| parse_card(&x.unwrap()))
        .collect();

    let result = match part {
        1 => part_one(game),
        2 => part_two(game),
        _ => panic!("Part not implemented"),
    };

    println!("{}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_calculates_correctly() {
        assert_eq!(super::calculate(vec![1, 2, 3]), 4);
        assert_eq!(super::calculate(vec![]), 0);
    }

    #[test]
    fn it_parses_the_card() {
        assert_eq!(
            super::parse_card("Card 1: 1 2 3 | 4 5 6"),
            (vec![1, 2, 3], vec![4, 5, 6])
        );
    }

    #[test]
    fn it_calculates_intersection() {
        assert_eq!(super::intersection(&[1, 2, 3], &[1, 2, 3]), vec![1, 2, 3]);
        assert_eq!(super::intersection(&[1, 2, 3], &[1, 2, 4]), vec![1, 2]);
        assert_eq!(super::intersection(&[1, 2, 3], &[4, 5, 6]), vec![]);
    }

    #[test]
    fn it_solves_part_one() {
        assert_eq!(
            super::part_one(vec![
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                (vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
                (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                (vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]),
                (vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
                (vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11])
            ]),
            13
        );
    }

    #[test]
    fn it_draws_cards() {
       assert_eq!(
            super::draw_cards(vec![
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                (vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
                (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                (vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]),
                (vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
                (vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11])
            ]),
            vec![1, 2, 4, 8, 14, 1]
       ) 
    }

    #[test]
    fn it_solves_part_two() {
        assert_eq!(
            super::part_two(vec![
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                (vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
                (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                (vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]),
                (vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
                (vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11])
            ]),
                30
        )  
    }
}

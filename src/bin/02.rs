#[derive(Debug)]
pub enum Item {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Winner {
    P1,
    P2,
    Draw,
}

use {Item::Paper, Item::Rock, Item::Scissors, Winner::Draw, Winner::P1, Winner::P2};

fn item_score(p1: Item) -> u32 {
    match p1 {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn char_to_item(c: char) -> Item {
    match c {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
        _ => panic!(),
    }
}

fn winner(p1: &Item, p2: &Item) -> Winner {
    match (p1, p2) {
        (Rock, Rock) => Draw,
        (Rock, Paper) => P2,
        (Rock, Scissors) => P1,
        (Paper, Rock) => P1,
        (Paper, Paper) => Draw,
        (Paper, Scissors) => P2,
        (Scissors, Rock) => P2,
        (Scissors, Paper) => P1,
        (Scissors, Scissors) => Draw,
    }
}

fn line_score(line: &str) -> u32 {
    let p1 = dbg!(char_to_item(line.chars().nth(0).unwrap()));
    let p2 = dbg!(char_to_item(line.chars().nth(2).unwrap()));

    let winner_score = match winner(&p1, &p2) {
        P2 => 6,
        Draw => 3,
        P1 => 0,
    };
    dbg!(winner_score) + dbg!(item_score(p2))
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(line_score)
        .inspect(|v| println!("{}", v))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}

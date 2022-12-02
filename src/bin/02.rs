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

fn item_score(p: Item) -> u32 {
    match p {
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

fn char_to_result(c: char) -> Winner {
    match c {
        'X' => P1,
        'Y' => Draw,
        'Z' => P2,
        _ => panic!(),
    }
}

fn item_for_desired_result(p1: &Item, winner: &Winner) -> Item {
    match (p1, winner) {
        (Rock, P1) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, P2) => Paper,
        (Paper, P1) => Rock,
        (Paper, Draw) => Paper,
        (Paper, P2) => Scissors,
        (Scissors, P1) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, P2) => Rock,
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

fn winner_score(winner: Winner) -> u32 {
    match winner {
        P2 => 6,
        Draw => 3,
        P1 => 0,
    }
}

fn part1_line_score(line: &str) -> u32 {
    let p1 = char_to_item(line.chars().nth(0).unwrap());
    let p2 = char_to_item(line.chars().nth(2).unwrap());

    winner_score(winner(&p1, &p2)) + item_score(p2)
}

fn part2_line_score(line: &str) -> u32 {
    let p1 = char_to_item(line.chars().nth(0).unwrap());
    let winner = char_to_result(line.chars().nth(2).unwrap());
    let p2 = item_for_desired_result(&p1, &winner);

    winner_score(winner) + item_score(p2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(part1_line_score)
        // .inspect(|v| println!("{}", v))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(part2_line_score)
        // .inspect(|v| println!("{}", v))
        .sum();

    Some(result)
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
        assert_eq!(part_two(&input), Some(12));
    }
}

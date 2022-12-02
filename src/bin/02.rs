#[derive(Debug, Clone)]
pub enum Item {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone)]
pub enum Winner {
    P1,
    P2,
    Draw,
}

struct Game {
    p1: Item,
    p2: Item,
}

impl Game {
    fn winner(&self) -> Winner {
        match (&self.p1, &self.p2) {
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
}
use {Item::*, Winner::*};

trait Score {
    fn score(&self) -> u32;
}

impl Score for Item {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

impl Score for Winner {
    fn score(&self) -> u32 {
        match self {
            P2 => 6,
            Draw => 3,
            P1 => 0,
        }
    }
}

trait CharExt {
    fn to_item(&self) -> Item;
    fn to_result(&self) -> Winner;
}

impl CharExt for char {
    fn to_item(&self) -> Item {
        match self {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!(),
        }
    }

    fn to_result(&self) -> Winner {
        match self {
            'X' => P1,
            'Y' => Draw,
            'Z' => P2,
            _ => panic!(),
        }
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

fn part1_line_score(line: &str) -> u32 {
    let p1 = line.chars().nth(0).unwrap().to_item();
    let p2 = line.chars().nth(2).unwrap().to_item();
    let game = Game {
        p1: p1,
        p2: p2.clone(),
    };

    game.winner().score() + p2.score()
}

fn part2_line_score(line: &str) -> u32 {
    let p1 = line.chars().nth(0).unwrap().to_item();
    let winner = line.chars().nth(2).unwrap().to_result();
    let p2 = item_for_desired_result(&p1, &winner);

    winner.score() + p2.score()
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

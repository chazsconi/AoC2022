use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;

fn to_set(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn char_to_value(c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        *c as u32 - 'a' as u32 + 1
    } else {
        *c as u32 - 'A' as u32 + 27
    }
}

fn part1_line_score(line: &str) -> u32 {
    let len = line.chars().count();
    let (left, right) = line.split_at(len / 2);

    let left_set = to_set(left);
    let right_set = to_set(right);
    let diff = left_set.intersection(&right_set);

    diff.map(char_to_value).sum()
}

// fn part2_line_score(lines: impl Iterator<Item = str>) -> u32 {
//     // let foo = lines.map(|s: &str| s);

//     // .reduce(|acc, other| acc.intersection(&other));
//     // let len = line.chars().count();
//     // let (left, right) = line.split_at(len / 2);

//     // let left_set = to_set(left);
//     // let right_set = to_set(right);
//     // let diff = left_set.intersection(&right_set);

//     // diff.map(char_to_value).sum()
//     10
// }

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(part1_line_score)
        // .inspect(|v| println!("{}", v))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let chunks = input.lines().chunks(3);
    let result = chunks.into_iter().map(|lines| {
        let mut line_sets = lines.map(|line| to_set(line));
        let mut set = line_sets.next().unwrap();

        for line in line_sets {
            set = set.intersection(&line).copied().collect();
        }

        let v: u32 = set.iter().map(char_to_value).sum();
        v
    });

    let total: u32 = result.sum();

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

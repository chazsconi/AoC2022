use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<char>>;

fn execute_moves(stacks: Stacks, moves: Vec<Move>) -> Stacks {
    moves.iter().fold(stacks, |acc: Stacks, mov| {
        (1..mov.count + 1).fold(acc, |mut acc2: Stacks, _n| {
            let item = acc2[mov.from - 1].pop().unwrap();
            acc2[mov.to - 1].push(item);
            acc2
        })
    })
}

fn execute_moves_part2(stacks: Stacks, moves: Vec<Move>) -> Stacks {
    moves.iter().fold(stacks, |mut acc: Stacks, mov| {
        let removed: Vec<char> = (1..mov.count + 1).fold(Vec::new(), |mut acc2, _n| {
            let item = acc[mov.from - 1].pop().unwrap();
            acc2.push(item);
            acc2
        });
        acc[mov.to - 1].append(&mut removed.into_iter().rev().collect());
        acc
    })
}

fn top(stacks: Stacks) -> String {
    let chars = stacks
        .iter()
        .map(|stack| stack.clone().pop().unwrap())
        .collect();
    chars
}

fn load_stacks(stacks_input: &str) -> Stacks {
    let mut stack_lines: Vec<&str> = stacks_input.lines().rev().collect_vec();
    let stack_count = (stack_lines.first().unwrap().chars().count() + 2) / 4;
    let mut stacks: Stacks = Vec::new();
    let mut i = 0;
    while i < stack_count {
        stacks.push(Vec::new());
        i += 1
    }

    stack_lines.remove(0);

    for line in stack_lines {
        let chars: Vec<char> = line
            .chars()
            .chunks(4)
            .into_iter()
            .map(|char_iter| {
                let chars_vec: Vec<char> = char_iter.collect();
                chars_vec.get(1).unwrap().clone()
            })
            .collect();

        let mut stack_no = 0;
        while stack_no < stack_count {
            match chars.get(stack_no) {
                Some(v) => match v {
                    ' ' => (),
                    c => stacks[stack_no].push(c.clone()),
                },
                None => (),
            }
            stack_no += 1;
        }
    }

    stacks
}

fn load_moves(moves_input: &str) -> Vec<Move> {
    let regex = Regex::new(r"move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();
    moves_input
        .lines()
        .map(|line| {
            let caps = regex.captures(line).unwrap();
            let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

            Move {
                count: count,
                from: from,
                to: to,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let stacks_and_moves: Vec<&str> = input.split("\n\n").collect();
    let stacks = load_stacks(stacks_and_moves.get(0).unwrap());
    let moves = load_moves(stacks_and_moves.get(1).unwrap());
    let top = top(execute_moves(stacks, moves));
    Some(top)
}

pub fn part_two(input: &str) -> Option<String> {
    let stacks_and_moves: Vec<&str> = input.split("\n\n").collect();
    let stacks = load_stacks(stacks_and_moves.get(0).unwrap());
    let moves = load_moves(stacks_and_moves.get(1).unwrap());
    let top = top(execute_moves_part2(stacks, moves));
    Some(top)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}

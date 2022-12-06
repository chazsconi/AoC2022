use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut chars: Vec<char> = input.chars().collect();
    chars.resize(10_000, ' ');
    let char_array: [char; 10_000] = chars.try_into().unwrap();
    let pos = char_array
        .windows(4)
        .position(|char4| {
            let unique: Vec<&char> = char4.iter().unique().collect();
            unique.len() == 4
        })
        .unwrap();
    Some(pos + 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut chars: Vec<char> = input.chars().collect();
    chars.resize(10_000, ' ');
    let char_array: [char; 10_000] = chars.try_into().unwrap();
    let pos = char_array
        .windows(14)
        .position(|char14| {
            let unique: Vec<&char> = char14.iter().unique().collect();
            unique.len() == 14
        })
        .unwrap();
    Some(pos + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}

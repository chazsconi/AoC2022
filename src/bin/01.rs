use itertools::Itertools;

fn to_u32(x: &str) -> u32 {
    x.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves_supplies = input.split("\n\n");

    let summed_supplies_it = elves_supplies.map(|elf_supplies| {
        let it = elf_supplies.lines().map(to_u32);
        let sum: u32 = it.sum();
        sum
    });

    summed_supplies_it.max()
}

pub fn part_two(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf_supplies| elf_supplies.lines().map(to_u32).sum())
        .sorted_by(|a: &u32, b: &u32| b.cmp(a))
        .take(3)
        .sum()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), 45000);
    }
}

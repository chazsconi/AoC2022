use std::collections::HashSet;

fn to_u32(x: &str) -> u32 {
    x.parse::<u32>().unwrap()
}

#[derive(Debug, Clone)]
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        (other.start >= self.start && other.end <= self.end)
            || (self.start >= other.start && self.end <= other.end)
    }

    fn overlaps(&self, other: &Section) -> bool {
        let set1: HashSet<u32> = (self.start..self.end + 1).collect();
        let set2: HashSet<u32> = (other.start..other.end + 1).collect();

        set1.intersection(&set2).count() > 0
    }
}
fn string_to_section(s: &str) -> Section {
    let section_s = s.split("-");
    let collected_section: Vec<&str> = section_s.collect();
    let start = to_u32(collected_section.get(0).unwrap());
    let end = to_u32(collected_section.get(1).unwrap());

    let section = Section {
        start: start,
        end: end,
    };
    section
}

fn part1_line_score(line: &str) -> u32 {
    let sections: Vec<&str> = line.split(",").collect();

    let section1 = string_to_section(sections.get(0).unwrap());
    let section2 = string_to_section(sections.get(1).unwrap());

    if section1.contains(&section2) {
        1
    } else {
        0
    }
}

fn part2_line_score(line: &str) -> u32 {
    let sections: Vec<&str> = line.split(",").collect();

    let section1 = string_to_section(sections.get(0).unwrap());
    let section2 = string_to_section(sections.get(1).unwrap());

    if section1.overlaps(&section2) {
        1
    } else {
        0
    }
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
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

advent_of_code::solution!(3);

use regex::Regex;
use std::sync::LazyLock;

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

fn find_total(input: &str) -> u32 {
    let captures = RE.captures_iter(input);
    let total = captures
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * c.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum();

    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = find_total(input);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .split("do()")
        .map(|d| d.split("don't()").next().map_or(0, find_total))
        .sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let example = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_two(example);
        assert_eq!(result, Some(48));
    }
}

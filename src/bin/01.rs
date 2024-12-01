advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let ns = l
                .split_whitespace()
                .map(|w| w.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (ns[0], ns[1])
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2) = parse_lists(input);

    l1.sort_unstable();
    l2.sort_unstable();

    let distances = l1
        .iter()
        .zip(l2.iter())
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum();

    Some(distances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l1, l2) = parse_lists(input);
    let mut score = 0;
    for x in &l1 {
        score += x * l2.iter().filter(|i| *i == x).count() as u32;
    }
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

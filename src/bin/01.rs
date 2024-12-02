advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let mut ns = l.split_whitespace().map(|w| w.parse::<u32>().unwrap());
            (ns.next().unwrap(), ns.next().unwrap())
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
    let l2_index: std::collections::HashMap<u32, u32> =
        l2.iter()
            .fold(std::collections::HashMap::new(), |mut acc, x| {
                acc.entry(*x).and_modify(|e| *e += 1).or_insert(1);
                acc
            });

    let score = l1.iter().map(|x| x * l2_index.get(x).unwrap_or(&0)).sum();
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
        assert_eq!(result, Some(31));
    }
}

advent_of_code::solution!(7);

type Equation = (usize, Vec<usize>);

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let (test, rest) = l.split_once(':').unwrap();
            (
                test.parse().unwrap(),
                rest.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn find_operators(target: usize, current: usize, rest: &[usize]) -> bool {
    if rest.is_empty() {
        return target == current;
    }

    if current > target {
        return false;
    }

    find_operators(target, current + rest[0], &rest[1..])
        || find_operators(target, current * rest[0], &rest[1..])
}

pub fn part_one(input: &str) -> Option<usize> {
    let equations = parse_input(input);
    let valid_equations = equations
        .iter()
        .filter(|(target, values)| find_operators(*target, values[0], &values[1..]));
    let sum: usize = valid_equations.map(|(target, _)| *target).sum();
    Some(sum)
}

fn find_operators_concat(target: usize, current: usize, rest: &[usize]) -> bool {
    if rest.is_empty() {
        return target == current;
    }

    if current > target {
        return false;
    }

    find_operators_concat(target, current + rest[0], &rest[1..])
        || find_operators_concat(target, current * rest[0], &rest[1..])
        || find_operators_concat(
            target,
            current * (10_usize.pow(rest[0].checked_ilog10().unwrap_or(0) + 1)) + rest[0],
            &rest[1..],
        )
}

pub fn part_two(input: &str) -> Option<usize> {
    let equations = parse_input(input);
    let valid_equations = equations
        .iter()
        .filter(|(target, values)| find_operators_concat(*target, values[0], &values[1..]));

    let sum: usize = valid_equations.map(|(target, _)| *target).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

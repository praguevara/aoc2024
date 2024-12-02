advent_of_code::solution!(2);

type Level = u32;

type Report = Vec<Level>;

fn parse_input(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|w| w.parse::<u32>().unwrap())
                .collect::<Report>()
        })
        .collect()
}

fn check_safe(report: &Report) -> bool {
    let increasing = report[0] < report[1];
    for w in report.windows(2) {
        let a = w[0];
        let b = w[1];

        if (a < b) != increasing {
            return false;
        }

        let diff = a.max(b) - a.min(b);
        if !(1..=3).contains(&diff) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let safe_count = reports.iter().map(check_safe).filter(|x| *x).count() as u32;
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    let safe_count = reports
        .iter()
        .map(|r| {
            if check_safe(r) {
                return true;
            };
            for i in 0..r.len() {
                let mut x = r.to_owned();
                x.remove(i);
                if check_safe(&x) {
                    return true;
                };
            }
            false
        })
        .filter(|x| *x)
        .count() as u32;

    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

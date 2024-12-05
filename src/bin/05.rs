advent_of_code::solution!(5);

type Rules = std::collections::HashMap<u32, Vec<u32>>;

type Update = Vec<u32>;

struct Input {
    rules: Rules,
    updates: Vec<Update>,
}

fn parse_input(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    Input {
        rules: rules
            .lines()
            .map(|l| {
                let s = l.split_once('|').unwrap();
                (s.0.parse::<u32>().unwrap(), s.1.parse::<u32>().unwrap())
            })
            .fold(
                std::collections::HashMap::new(),
                |mut acc, (before, after)| {
                    acc.entry(before)
                        .and_modify(|v| v.push(after))
                        .or_insert(vec![after]);
                    acc
                },
            ),
        updates: updates
            .lines()
            .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
            .collect(),
    }
}

fn violates_rule(rules: &Rules, update: &Update) -> Option<(usize, usize)> {
    for (i, page) in update.iter().enumerate() {
        for after in rules.get(page).unwrap_or(&vec![]) {
            if let Some(after_pos) = update[0..i].iter().position(|x| x == after) {
                return Some((i, after_pos));
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let Input { rules, updates } = parse_input(input);

    let valid_updates = updates
        .iter()
        .filter(|update| violates_rule(&rules, update).is_none());
    let middle_pages = valid_updates.map(|u| u[u.len() / 2]);
    let sum = middle_pages.sum::<u32>();

    Some(sum)
}

fn fix_update(rules: &Rules, mut update: Update) -> Update {
    while let Some((before, after)) = violates_rule(rules, &update) {
        update.swap(before, after);
    }
    update
}

pub fn part_two(input: &str) -> Option<u32> {
    let Input { rules, updates } = parse_input(input);

    let invalid_updates = updates
        .into_iter()
        .filter(|update| violates_rule(&rules, update).is_some());

    let fixed_updates = invalid_updates.map(|update| fix_update(&rules, update));

    let middle_pages = fixed_updates.map(|u| u[u.len() / 2]);
    let sum = middle_pages.sum::<u32>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}

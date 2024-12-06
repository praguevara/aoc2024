advent_of_code::solution!(6);

type Position = (i32, i32);

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn rotate_clockwise(&self) -> Orientation {
        use Orientation::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug)]
struct Map {
    obstructions: Vec<Vec<bool>>,
}

impl Map {
    fn rows(&self) -> usize {
        self.obstructions.len()
    }

    fn cols(&self) -> usize {
        self.obstructions.first().map(|r| r.len()).unwrap_or(0)
    }

    fn is_obstructed(&self, (r, c): Position) -> bool {
        self.obstructions
            .get(r as usize)
            .map(|row| row.get(c as usize).copied().unwrap_or(false))
            .unwrap_or(false)
    }
}

fn parse_input(input: &str) -> (Map, Position, Orientation) {
    let (cells, position) =
        input
            .lines()
            .enumerate()
            .fold((vec![], None), |(mut lines, p), (row, line)| {
                let bytes = line.as_bytes();
                lines.push(bytes.iter().map(|c| matches!(c, b'#')).collect::<Vec<_>>());

                if p.is_none() {
                    if let Some(col) = bytes.iter().position(|x| *x == b'^') {
                        return (lines, Some((row as i32, col as i32)));
                    }
                }

                (lines, p)
            });

    (
        Map {
            obstructions: cells,
        },
        position.unwrap(),
        Orientation::Up,
    )
}

fn is_in_map(map: &Map, position: Position) -> bool {
    position.0 >= 0
        && position.1 >= 0
        && position.0 < map.rows() as i32
        && position.1 < map.cols() as i32
}

fn next_position(position: &Position, orientation: Orientation) -> Position {
    (
        position.0
            + match orientation {
                Orientation::Up => -1,
                Orientation::Down => 1,
                _ => 0,
            },
        position.1
            + match orientation {
                Orientation::Right => 1,
                Orientation::Left => -1,
                _ => 0,
            },
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, mut position, mut orientation) = parse_input(input);

    let mut visited = std::collections::HashSet::new();
    while is_in_map(&map, position) {
        visited.insert(position);
        let next_position = next_position(&position, orientation);

        if map.is_obstructed(next_position) {
            orientation = orientation.rotate_clockwise();
        } else {
            position = next_position;
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, starting_position, starting_orientation) = parse_input(input);

    let mut position = starting_position;
    let mut orientation = starting_orientation;
    let mut visited = std::collections::HashSet::new();
    while is_in_map(&map, position) {
        visited.insert(position);
        let next_position = next_position(&position, orientation);

        if map.is_obstructed(next_position) {
            orientation = orientation.rotate_clockwise();
        } else {
            position = next_position;
        }
    }

    let mut loops_found = 0;
    let mut visited_ori = std::collections::HashSet::new();
    for (i, j) in visited {
        let mut position = starting_position;
        let mut orientation = starting_orientation;
        visited_ori.clear();

        if map.is_obstructed((i, j)) {
            continue;
        }

        map.obstructions[i as usize][j as usize] = true;

        while is_in_map(&map, position) {
            if visited_ori.contains(&(position, orientation)) {
                loops_found += 1;
                break;
            } else {
                visited_ori.insert((position, orientation));
            }

            let next_position = next_position(&position, orientation);

            if map.is_obstructed(next_position) {
                orientation = orientation.rotate_clockwise();
            } else {
                position = next_position;
            }
        }

        map.obstructions[i as usize][j as usize] = false;
    }

    Some(loops_found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

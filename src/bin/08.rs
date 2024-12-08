advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Coord {
    fn times(self, n: i32) -> Self {
        Self(n * self.0, n * self.1)
    }
}

type Freq = u8;

type Antennas = HashMap<Freq, Vec<Coord>>;

fn parse_input(input: &str) -> (Antennas, Coord) {
    let (antennas, Coord(height, width)) = input.lines().enumerate().fold(
        (HashMap::new(), Coord(0, 0)),
        |(mut acc, Coord(h, w)), (r, line)| {
            let width = if r == 0 { line.len() as i32 } else { w };

            line.bytes().enumerate().for_each(|(c, char)| {
                if char != b'.' {
                    acc.entry(char)
                        .and_modify(|coords: &mut Vec<_>| coords.push(Coord(r as i32, c as i32)))
                        .or_insert_with(|| vec![Coord(r as i32, c as i32)]);
                }
            });

            (acc, Coord(h + 1, width))
        },
    );

    (antennas, Coord(height, width))
}

fn is_coord_bounded(Coord(a, b): Coord, Coord(rows, cols): Coord) -> bool {
    a >= 0 && b >= 0 && a < rows && b < cols
}

fn compute_antinodes(a: Coord, b: Coord) -> [Coord; 2] {
    let ab = b - a;
    [a - ab, b + ab]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas, bounds) = parse_input(input);
    let bounded_antinodes = antennas
        .values()
        .flat_map(|freq_antennas| {
            (0..freq_antennas.len()).flat_map(move |i| {
                ((i + 1)..freq_antennas.len()).flat_map(move |j| {
                    compute_antinodes(freq_antennas[i], freq_antennas[j])
                        .into_iter()
                        .filter(move |c| is_coord_bounded(*c, bounds))
                })
            })
        })
        .collect::<HashSet<_>>();

    Some(bounded_antinodes.len() as u32)
}

fn compute_multiple_antinodes(a: Coord, b: Coord, bounds: Coord) -> impl Iterator<Item = Coord> {
    let ab = b - a;

    let backward_iter = (0..)
        .map(move |i| a - ab.times(i))
        .take_while(move |antinode| is_coord_bounded(*antinode, bounds));

    let forward_iter = (0..)
        .map(move |i| b + ab.times(i))
        .take_while(move |antinode| is_coord_bounded(*antinode, bounds));

    backward_iter.chain(forward_iter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennas, bounds) = parse_input(input);
    let bounded_antinodes = antennas
        .values()
        .flat_map(|freq_antennas| {
            (0..freq_antennas.len()).flat_map(move |i| {
                ((i + 1)..freq_antennas.len()).flat_map(move |j| {
                    compute_multiple_antinodes(freq_antennas[i], freq_antennas[j], bounds)
                        .filter(move |c| is_coord_bounded(*c, bounds))
                })
            })
        })
        .collect::<HashSet<_>>();

    Some(bounded_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}

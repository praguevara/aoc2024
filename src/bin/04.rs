advent_of_code::solution!(4);

struct Board {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(data: Vec<Vec<u8>>) -> Self {
        Self {
            rows: data.len(),
            cols: data.first().map(|r| r.len()).unwrap_or(0),
            data,
        }
    }

    fn iter_rows(&self) -> impl Iterator<Item = BoardIter> {
        (0..self.rows).map(move |r| BoardIter {
            board: self,
            i: 0,
            j: r as i32,
            vi: 1,
            vj: 0,
        })
    }

    fn iter_rows_rev(&self) -> impl Iterator<Item = BoardIter> {
        (0..self.rows).map(move |r| BoardIter {
            board: self,
            i: (self.cols - 1) as i32,
            j: r as i32,
            vi: -1,
            vj: 0,
        })
    }
    fn iter_cols(&self) -> impl Iterator<Item = BoardIter> {
        (0..self.cols).map(move |c| BoardIter {
            board: self,
            i: c as i32,
            j: 0,
            vi: 0,
            vj: 1,
        })
    }

    fn iter_cols_rev(&self) -> impl Iterator<Item = BoardIter> {
        (0..self.cols).map(move |c| BoardIter {
            board: self,
            i: c as i32,
            j: (self.rows - 1) as i32,
            vi: 0,
            vj: -1,
        })
    }

    fn iter_diag_dr(&self) -> impl Iterator<Item = BoardIter> {
        Iterator::chain(
            (0..self.rows).rev().map(move |r| BoardIter {
                board: self,
                i: 0,
                j: r as i32,
                vi: 1,
                vj: 1,
            }),
            (1..self.cols).map(move |c| BoardIter {
                board: self,
                i: c as i32,
                j: 0,
                vi: 1,
                vj: 1,
            }),
        )
    }

    fn iter_diag_ul(&self) -> impl Iterator<Item = BoardIter> {
        Iterator::chain(
            (0..self.cols).map(move |c| BoardIter {
                board: self,
                i: c as i32,
                j: (self.rows - 1) as i32,
                vi: -1,
                vj: -1,
            }),
            (0..(self.rows - 1)).rev().map(move |r| BoardIter {
                board: self,
                i: (self.cols - 1) as i32,
                j: r as i32,
                vi: -1,
                vj: -1,
            }),
        )
    }

    fn iter_diag_dl(&self) -> impl Iterator<Item = BoardIter> {
        Iterator::chain(
            (0..self.cols).map(move |c| BoardIter {
                board: self,
                i: c as i32,
                j: 0,
                vi: -1,
                vj: 1,
            }),
            (1..self.rows).map(move |r| BoardIter {
                board: self,
                i: (self.cols - 1) as i32,
                j: r as i32,
                vi: -1,
                vj: 1,
            }),
        )
    }

    fn iter_diag_ur(&self) -> impl Iterator<Item = BoardIter> {
        Iterator::chain(
            (0..self.rows).map(move |r| BoardIter {
                board: self,
                i: 0,
                j: r as i32,
                vi: 1,
                vj: -1,
            }),
            (1..(self.cols)).map(move |c| BoardIter {
                board: self,
                i: c as i32,
                j: (self.rows - 1) as i32,
                vi: 1,
                vj: -1,
            }),
        )
    }

    fn iter_convolutions(&self) -> impl Iterator<Item = [[u8; 3]; 3]> + '_ {
        (0..(self.rows - 2)).flat_map(move |r| {
            (0..(self.cols - 2)).map(move |c| {
                [
                    [self.data[r][c], self.data[r][c + 1], self.data[r][c + 2]],
                    [
                        self.data[r + 1][c],
                        self.data[r + 1][c + 1],
                        self.data[r + 1][c + 2],
                    ],
                    [
                        self.data[r + 2][c],
                        self.data[r + 2][c + 1],
                        self.data[r + 2][c + 2],
                    ],
                ]
            })
        })
    }
}

struct BoardIter<'a> {
    board: &'a Board,
    i: i32,
    j: i32,
    vi: i32,
    vj: i32,
}

impl Iterator for BoardIter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 0 || self.j < 0 {
            return None;
        }

        if let Some(cell) = self
            .board
            .data
            .get(self.j as usize)
            .and_then(|r| r.get(self.i as usize))
        {
            self.i += self.vi;
            self.j += self.vj;
            Some(*cell)
        } else {
            None
        }
    }
}

fn parse_board(input: &str) -> Board {
    Board::new(input.lines().map(|line| line.as_bytes().to_vec()).collect())
}

fn count_occurrences<T: PartialEq>(data: impl Iterator<Item = T>, pat: &[T]) -> usize {
    let mut occurrences = 0;
    let mut pat_i = 0;
    for x in data {
        if x == pat[pat_i] {
            pat_i += 1;
            if pat_i >= pat.len() {
                occurrences += 1;
                pat_i = 0;
            }
        } else {
            pat_i = if x == pat[0] { 1 } else { 0 }
        }
    }
    occurrences
}

fn count_xmas(gen: impl Iterator<Item = impl Iterator<Item = u8>>) -> usize {
    gen.map(|x| count_occurrences(x, b"XMAS")).sum()
}

#[test]
fn count_xmas_test() {
    assert_eq!(count_xmas(std::iter::once(b"XMASXMAS".iter().copied())), 2);
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = parse_board(input);
    let r = dbg!(count_xmas(board.iter_rows()));
    let rr = dbg!(count_xmas(board.iter_rows_rev()));
    let c = dbg!(count_xmas(board.iter_cols()));
    let cr = dbg!(count_xmas(board.iter_cols_rev()));
    let dr = dbg!(count_xmas(board.iter_diag_dr()));
    let dl = dbg!(count_xmas(board.iter_diag_dl()));
    let ur = dbg!(count_xmas(board.iter_diag_ur()));
    let ul = dbg!(count_xmas(board.iter_diag_ul()));
    Some((r + rr + c + cr + dr + dl + ur + ul) as u32)
}

fn is_xmas_convolution(conv: &[[u8; 3]; 3]) -> bool {
    if conv[1][1] != b'A' {
        return false;
    }

    if !matches!(&[conv[0][0], conv[2][2]], b"MS" | b"SM") {
        return false;
    }

    if !matches!(&[conv[2][0], conv[0][2]], b"MS" | b"SM") {
        return false;
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = parse_board(input);
    let convolutions = board.iter_convolutions();
    Some(convolutions.filter(is_xmas_convolution).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_line(line: impl Iterator<Item = u8>) {
        println!("{}", String::from_utf8(line.collect::<Vec<_>>()).unwrap())
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}

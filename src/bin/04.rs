use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(4);

static ADJACENT_COORDS: [(i16, i16); 8] = [
    (1, 1),
    (-1, 1),
    (1, -1),
    (-1, -1),
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
];

trait Accessible {
    fn accessible(&self, coords: (usize, usize), line_length: usize, line_count: usize) -> bool;
}

impl Accessible for &str {
    fn accessible(&self, coords: (usize, usize), line_length: usize, line_count: usize) -> bool {
        let adjacency_count = ADJACENT_COORDS
            .map(|a| {
                let adj_coords = (coords.0 as i16 + a.0, coords.1 as i16 + a.1);
                if ((adj_coords.0 >= 0) && ((adj_coords.0 as usize) < line_length))
                    && (adj_coords.1 >= 0 && adj_coords.1 < line_count as i16)
                {
                    let idx = adj_coords.1 as usize * line_length + adj_coords.0 as usize;
                    let adj = self.as_bytes()[idx];
                    if adj == "@".as_bytes()[0] { 1 } else { 0 }
                } else {
                    0
                }
            })
            .iter()
            .sum::<u8>();

        adjacency_count < 4
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    /*let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";*/

    let line_length = input.lines().next().unwrap().len();
    let line_count = input.lines().collect::<Vec<&str>>().len();

    let stripped = input.replace("\n", "");
    let input = stripped.as_str();

    // y axis
    Some(
        (0..line_count)
            .into_par_iter()
            .map(|y| {
                // x axis
                (0..line_length)
                    .map(|x| {
                        let current_symbol = input.as_bytes()[y * line_length + x];
                        if current_symbol != "@".as_bytes()[0] {
                            return 0;
                        } else {
                            if input.accessible((x, y), line_length, line_count) {
                                1
                            } else {
                                0
                            }
                        }
                    })
                    .sum::<u64>()
            })
            .sum(),
    )
}

struct Input {
    pub data: String,
    pub line_count: usize,
    pub line_length: usize,
}

impl From<&str> for Input {
    fn from(input: &str) -> Self {
        let line_length = input.lines().next().unwrap().len();
        let line_count = input.lines().collect::<Vec<&str>>().len();

        let stripped = input.replace("\n", "");
        let input = stripped.as_str();
        Self {
            data: input.to_string(),
            line_count,
            line_length,
        }
    }
}

impl IntoIterator for Input {
    type Item = u64;

    type IntoIter = InputIterator;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { input: self }
    }
}

struct InputIterator {
    input: Input,
}

impl Iterator for InputIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        //let input_copy = self.input.data.clone();
        //let mut new_data = String::new();
        let out = (0..self.input.line_count)
            .into_iter()
            .map(|y| {
                (0..self.input.line_length)
                    .map(|x| {
                        let idx = y * self.input.line_length + x;
                        let current_symbol = self.input.data.as_bytes()[idx];
                        if current_symbol != "@".as_bytes()[0] {
                            0
                        } else {
                            if self.input.data.as_str().accessible(
                                (x, y),
                                self.input.line_length,
                                self.input.line_count,
                            ) {
                                self.input.data.replace_range(idx..idx + 1, ".");
                                1
                            } else {
                                0
                            }
                        }
                    })
                    .sum::<u64>()
            })
            .sum::<u64>();

        //self.input.data = new_data;
        //println!("out: {out}");

        if out == 0 { None } else { Some(out) }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    /*let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";*/

    let input = Input::from(input);

    Some(input.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

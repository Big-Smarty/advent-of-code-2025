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

pub fn accessible(
    coords: (usize, usize),
    input: Vec<Vec<char>>,
    line_length: usize,
    line_count: usize,
) -> bool {
    let adjacencies = ADJACENT_COORDS
        .map(|a| {
            let temp_coords = (coords.0 as i16 + a.0, coords.1 as i16 + a.1);
            if (temp_coords.0 >= 0 && temp_coords.0 < line_length as i16)
                && (temp_coords.1 >= 0 && temp_coords.1 < line_count as i16)
            {
                let temp = input[temp_coords.0 as usize][temp_coords.1 as usize];
                if temp == '@' { 1 } else { 0 }
            } else {
                0
            }
        })
        .iter()
        .sum::<u8>();
    if adjacencies < 4 {
        return true;
    }
    return false;
}
pub fn accessible_2(
    coords: (usize, usize),
    input: &str,
    line_length: usize,
    line_count: usize,
) -> bool {
    let adjacencies = ADJACENT_COORDS
        .map(|a| {
            let temp_coords = (coords.0 as i16 + a.0, coords.1 as i16 + a.1);
            if (temp_coords.0 >= 0 && temp_coords.0 < line_length as i16)
                && (temp_coords.1 >= 0 && temp_coords.1 < line_count as i16)
            {
                let idx = temp_coords.1 as usize * line_length + temp_coords.0 as usize;
                let temp = &input[idx..idx + 1];
                if temp == "@" { 1 } else { 0 }
            } else {
                0
            }
        })
        .iter()
        .sum::<u8>();
    if adjacencies < 4 {
        return true;
    }
    return false;
}

pub fn part_one(input: &str) -> Option<u64> {
    let line_length = input.lines().next().unwrap().len();
    let line_count = input.lines().collect::<Vec<&str>>().len();

    let input: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().map(|c| c).collect())
        .collect();

    // y axis
    Some(
        (0..line_count)
            .into_par_iter()
            .map(|y| {
                // x axis
                (0..line_length)
                    .map(|x| {
                        let coords = (x, y);
                        let current_symbol = input[x][y];
                        if current_symbol != '@' {
                            return 0;
                        } else {
                            if accessible(coords, input.clone(), line_length, line_count) {
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
    let line_length = input.lines().next().unwrap().len();
    let line_count = input.lines().collect::<Vec<&str>>().len();

    Some({
        let mut out = 0u64;
        let mut old_out = out;
        out += input
            .lines()
            .zip(0..line_count)
            .map(|(l, y)| {
                l.chars()
                    .zip(0..line_length)
                    .map(|(ref mut c, x)| {
                        if *c != '@' {
                            return 0;
                        } else {
                            if accessible_2((x, y), input, line_length, line_count) {
                                *c = '.';
                                1
                            } else {
                                0
                            }
                        }
                    })
                    .sum::<u64>()
            })
            .sum::<u64>();
        while old_out != out {
            old_out = out;
            out += input
                .lines()
                .zip(0..line_count)
                .map(|(l, y)| {
                    l.chars()
                        .zip(0..line_length)
                        .map(|(ref mut c, x)| {
                            if *c != '@' {
                                return 0;
                            } else {
                                if accessible_2((x, y), input, line_length, line_count) {
                                    *c = '.';
                                    1
                                } else {
                                    0
                                }
                            }
                        })
                        .sum::<u64>()
                })
                .sum::<u64>();
        }
        out
    })
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

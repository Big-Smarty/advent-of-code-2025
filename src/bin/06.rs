advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let lengths = lines[lines.len() - 1]
        .split("+")
        .map(|s| s.split("*"))
        .flatten()
        .map(|s| s.len())
        .filter(|l| *l != 0)
        .collect::<Vec<usize>>();
    let operators = lines[lines.len() - 1]
        .split(" ")
        .filter(|s| *s != "")
        .collect::<Vec<&str>>();
    let mut old_len = 0;
    let numbers = lengths
        .iter()
        .map(|l| {
            let out = (0..lines.len() - 1)
                .map(|i| {
                    let out = lines[i][old_len..=old_len + *l].trim();
                    out.parse::<u64>().unwrap()
                })
                .collect::<Vec<u64>>();

            old_len += l + 1;
            out
        })
        .collect::<Vec<Vec<u64>>>();

    Some(
        numbers
            .iter()
            .zip(operators)
            .map(|(n, o)| match o {
                "+" => n.iter().fold(0, |state, i| state + i),
                "*" => n.iter().fold(0, |state, i| state.max(1) * i),
                _ => unreachable!(),
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut lengths = lines[lines.len() - 1]
        .split("+")
        .map(|s| s.split("*"))
        .flatten()
        .map(|s| s.len() + 1)
        .filter(|l| *l != 0)
        .collect::<Vec<usize>>();
    let lengths_len = lengths.len();
    lengths[lengths_len - 1] += 1;

    let operators = lines[lines.len() - 1]
        .split(" ")
        .filter(|s| *s != "")
        .collect::<Vec<&str>>();
    let line_length = lines[0].len();

    let columns: Vec<Vec<&str>> = (0..line_length)
        .map(|x| (0..lines.len()).map(|y| &lines[y][x..x + 1]).collect())
        .filter(|h: &Vec<&str>| {
            h.iter()
                .filter(|f| **f != " ")
                .map(|f| *f)
                .collect::<Vec<&str>>()
                .len()
                != 0
        })
        .collect();

    let numbers: Vec<u64> = columns
        .iter()
        .map(|c| {
            let filtered = c
                .iter()
                .filter(|f| **f != " " && **f != "+" && **f != "*")
                .map(|f| *f)
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            filtered
        })
        .collect();

    let mut idx = 0;
    Some(
        lengths
            .iter()
            .zip(operators)
            .map(|(l, o)| match o {
                "+" => {
                    let out = (0..*l)
                        .map(|i| {
                            let numbers = numbers[idx + i];
                            numbers
                        })
                        .fold(0, |state, j| state + j);
                    idx += l;
                    out
                }
                "*" => {
                    let out = (0..*l)
                        .map(|i| {
                            let numbers = numbers[idx + i];
                            numbers
                        })
                        .fold(0, |state, j| state.max(1) * j);
                    idx += l;
                    out
                }
                _ => unreachable!(),
            })
            .sum::<u64>(),
    )
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

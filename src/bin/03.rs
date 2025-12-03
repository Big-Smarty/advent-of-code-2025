use rayon::{iter::ParallelIterator, str::ParallelString};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines()
            .map(|l| {
                let max = l.chars().map(|c| c as u8 - 48).max().unwrap();
                if l.chars()
                    .map(|c| c as u8 - 48)
                    .position(|y| y == max)
                    .unwrap() as usize
                    == l.len() - 1
                {
                    let second_max = l[..l.len() - 1]
                        .chars()
                        .map(|c| c as u8 - 48)
                        .max()
                        .unwrap();
                    return second_max * 10 + max;
                } else {
                    let second_max = l[l.chars().position(|x| x as u8 - 48 == max).unwrap() + 1..]
                        .chars()
                        .map(|x| x as u8 - 48)
                        .max()
                        .unwrap();
                    return max * 10 + second_max;
                }
            })
            .map(|i| i as u64)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = "987654321111111
811111111111119
234234234234278
818181911112111";
    None
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

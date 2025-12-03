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
    Some(
        input
            .par_lines()
            .map(|l| {
                let mut max_pos = l.len() - 12;
                let mut min_pos = 0;
                l[..12]
                    .chars()
                    .map(|c| c as u8 - 48)
                    .map(|m| {
                        let position = l.chars().position(|c| (c as u8 - 48) == m).unwrap();
                        // get max number within range
                        if let Some(max) = l[min_pos..=max_pos].chars().max() {
                            let max = max as u8 - 48;
                            // if max > m or m's position outside of range: return max
                            if max > m || !((min_pos <= position) && (position <= max_pos)) {
                                max_pos += 1;
                                min_pos = l[min_pos..max_pos]
                                    .chars()
                                    .position(|c| (c as u8 - 48) == max)
                                    .unwrap()
                                    + min_pos
                                    + 1;
                                max
                            } else {
                                //m's position within range | max !> m => return m
                                max_pos += 1;
                                min_pos += 1;
                                m
                            }
                        } else {
                            // there is DEFINITELY a max within the range
                            unreachable!()
                        }
                    })
                    .zip(1..13)
                    .map(|(m, i)| (m as u64) * 10u64.pow(12 - i))
                    .sum::<u64>()
            })
            .sum(),
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

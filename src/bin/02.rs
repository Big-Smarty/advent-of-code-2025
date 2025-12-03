use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(2);

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    pub fn new(input: &str) -> Self {
        let minmax = input
            .split('-')
            .into_iter()
            .map(|i| i.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Self {
            min: minmax[0],
            max: minmax[1],
        }
    }

    pub fn get_invalid_sum(&self) -> u64 {
        (self.min..=self.max)
            .into_par_iter()
            .filter(|i| !check_validity_improved(*i))
            .sum()
    }

    pub fn get_invalid_sum_2(&self) -> u64 {
        (self.min..=self.max)
            .into_iter()
            .filter(|i| !check_validity_2_improved(*i))
            .sum()
    }
}

fn check_validity(id: u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return true;
    } else {
        let (upper, lower) = id_str.split_at(id_str.len() / 2);
        if upper == lower {
            return false;
        } else {
            return true;
        }
    }
}

fn check_validity_improved(id: u64) -> bool {
    let id_len = id.ilog10() + 1;

    if id_len % 2 != 0 {
        true
    } else {
        let lower_half: u64 = id % 10u64.pow(id_len / 2);
        let upper_half = (id - lower_half) / 10u64.pow(id_len / 2);
        !(lower_half == upper_half)
    }
}

fn check_validity_2(id: u64) -> bool {
    let id_str = id.to_string();

    // go from 1 to half of string length (anything more would be a waste of time)
    (1..=id_str.len() / 2)
        .into_iter()
        .map(|i| {
            // OPTIMIZATION: skip current iteration if the id length is not divisible by i
            if id_str.len() % i != 0 {
                return true;
            } else {
                // iterate through all chunks; if all chunks are equal to the first chunk, the id is invalid!
                if id_str.as_bytes().chunks(i).all(|x| {
                    // check if current chunk is equal to first chunk
                    *x == id_str.as_bytes()[0..i]
                }) {
                    // id is invalid; all chunks were equal
                    //println!("invalid id: {id}");
                    return false;
                    // id is valid; not all chunks were equal
                } else {
                    return true;
                }
            }
        })
        .all(|i| i)
    //.fold(true, |out, b| out & b)
}

fn check_validity_2_improved(id: u64) -> bool {
    let id_len = id.ilog10() + 1;
    (1..=(id_len / 2))
        .into_iter()
        .map(|i| {
            if id_len % i != 0 {
                return true;
            } else {
                let chunk_size = i;

                // create chunks with size chunk_size (i)
                !(1..=(id_len / chunk_size))
                    .map(|j| {
                        (id % 10u64.pow(j * chunk_size) - (id % 10u64.pow((j - 1) * chunk_size)))
                            / 10u64.pow((j - 1) * chunk_size)
                    })
                    .all(|x| x == (id % 10u64.pow(chunk_size)))
            }
        })
        .all(|i| i)
}

fn get_range_vec(input: &str) -> Vec<Range> {
    input
        .replace("\n", "")
        .split(',')
        .map(|i| Range::new(i))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    //let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let ranges = get_range_vec(input);
    Some(ranges.par_iter().map(|r| r.get_invalid_sum()).sum::<u64>())
}

pub fn part_two(input: &str) -> Option<u64> {
    //let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let ranges = get_range_vec(input);
    Some(
        ranges
            .par_iter()
            .map(|r| r.get_invalid_sum_2())
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

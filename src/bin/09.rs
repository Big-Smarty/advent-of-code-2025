advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    /*
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    */
    let coords = input
        .lines()
        .map(|l| {
            let mut split = l.split(",").map(|s| s.parse::<u64>().unwrap());
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let pairs: Vec<((u64, u64), (u64, u64))> = coords
        .iter()
        .map(|c| coords.iter().filter(|f| **f != c.clone()).map(|f| (*c, *f)))
        .flatten()
        .collect();

    Some(
        pairs
            .iter()
            .map(|p| (p.0.0.abs_diff(p.1.0) + 1) * (p.0.1.abs_diff(p.1.1) + 1))
            .max()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
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

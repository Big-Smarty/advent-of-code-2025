use std::{collections::HashSet, ops::Deref};

use glam::I64Vec3;
#[derive(Clone, Debug)]
struct Circuit {
    pub boxes: HashSet<I64Vec3>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            boxes: HashSet::new(),
        }
    }

    pub fn insert(&mut self, coords: I64Vec3) {
        self.boxes.insert(coords);
    }

    pub fn distance(&self, other: &Circuit) -> i64 {
        self.boxes
            .iter()
            .map(|b1| {
                other
                    .boxes
                    .iter()
                    .map(|b2| b1.distance_squared(b2.clone()))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }

    pub fn merged(l: &Circuit, r: &Circuit) -> Self {
        Self {
            boxes: l.boxes.union(&r.boxes).map(|x| *x).collect(),
        }
    }
}

impl Deref for Circuit {
    type Target = HashSet<I64Vec3>;

    fn deref(&self) -> &Self::Target {
        &self.boxes
    }
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Self) -> bool {
        self.boxes == other.boxes
    }
}

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    #[rustfmt::skip]
                        let input =
                    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    let coords = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|c| I64Vec3::new(c[0], c[1], c[2]))
        .collect::<Vec<I64Vec3>>();

    println!("coords:");
    coords.iter().for_each(|c| println!("{c:?}"));

    let mut circuits = coords
        .iter()
        .map(|c| {
            let mut out = Circuit::new();
            out.insert(*c);
            out
        })
        .collect::<Vec<Circuit>>();

    for _ in 0..10 {
        // find closest pair
        let pair = circuits
            .iter()
            .map(|c| {
                println!("c: {c:?}");
                (
                    // add c to pair
                    c,
                    // add closest circuit to pair which is disjoint from c
                    circuits
                        .iter()
                        .filter(|f| f.is_disjoint(c))
                        .min_by(|x, y| x.distance(c).cmp(&y.distance(c)))
                        .unwrap(),
                )
            })
            .min_by(|x, y| (x.0.distance(x.1)).cmp(&y.0.distance(y.1)))
            .unwrap();
        let pair = Circuit::merged(pair.0, pair.1);
        println!("pair: {pair:?}");
        circuits = circuits
            .iter()
            .filter(|f| *f.clone() != pair)
            .map(|x| x.clone())
            .collect();
        circuits.push(pair.clone());
    }

    println!("circuits: ");
    for c in circuits.clone() {
        println!("{c:?}");
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
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

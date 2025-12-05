#![feature(new_range_api)]
#![feature(iter_map_windows)]

use std::ops::{Deref, RangeInclusive};

use itertools::Itertools;

advent_of_code::solution!(5);

trait Overlappable {
    fn overlapping(&self, other: Self) -> bool;
}

trait Mergeable {
    fn merge(&self, other: Self) -> Self;
}

trait Adjacency {
    fn adjacent(&self, other: Self) -> bool;
}

impl Overlappable for RangeInclusive<u64> {
    fn overlapping(&self, other: Self) -> bool {
        self.contains(other.start())
            | self.contains(other.end())
            | other.contains(self.start())
            | other.contains(self.end())
    }
}

impl Adjacency for RangeInclusive<u64> {
    fn adjacent(&self, other: Self) -> bool {
        (*self.end() + 1 == *other.start()) | (*other.end() + 1 == *self.start())
    }
}

impl Mergeable for RangeInclusive<u64> {
    fn merge(&self, other: Self) -> RangeInclusive<u64> {
        RangeInclusive::new(
            *self.start().min(other.start()),
            *self.end().max(other.end()),
        )
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    pub data: Vec<RangeInclusive<u64>>,
}

impl Deref for Ranges {
    type Target = Vec<RangeInclusive<u64>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl IntoIterator for Ranges {
    type Item = RangeInclusive<u64>;

    type IntoIter = MergingIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            ranges: self.clone(),
            index: 0,
            last: None,
        }
    }
}

struct MergingIter {
    ranges: Ranges,
    index: usize,
    last: Option<RangeInclusive<u64>>,
}

impl Iterator for MergingIter {
    type Item = RangeInclusive<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.ranges.len() {
            match self.last.clone() {
                Some(l) => {
                    if l.adjacent(self.ranges[self.index].clone())
                        | l.overlapping(self.ranges[self.index].clone())
                    {
                        let out = Some(Mergeable::merge(&l, self.ranges[self.index].clone()));
                        self.last = Some(self.ranges[self.index].clone());
                        self.index += 1;
                        out
                    } else {
                        let out = Some(self.ranges[self.index].clone());
                        self.last = Some(self.ranges[self.index].clone());
                        self.index += 1;
                        out
                    }
                }
                None => {
                    let out = Some(self.ranges[self.index].clone());
                    self.index += 1;
                    out
                }
            }
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges_and_ids = input.split("\n\n").collect::<Vec<&str>>();
    let ranges = ranges_and_ids[0];
    let ids = ranges_and_ids[1];

    let ranges = ranges
        .lines()
        .map(|l| l.split('-').collect::<Vec<&str>>())
        .map(|s| (s[0].parse::<u64>().unwrap())..=(s[1].parse::<u64>().unwrap()))
        .collect::<Vec<RangeInclusive<u64>>>();

    let ids = ids
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    Some(
        ids.iter()
            .map(|id| match ranges.iter().find(|r| r.contains(id)) {
                Some(_) => 1,
                None => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some({
        let ranges = input
            .split("\n\n")
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let range = l
                    .split('-')
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                RangeInclusive::new(range[0], range[1])
            })
            .map(|r| r);

        let sorted_ranges = ranges
            .clone()
            .sorted_unstable_by(|a, b| a.start().cmp(b.start()));

        let mut sorted_iter = sorted_ranges.clone().into_iter();

        let mut merged_ranges: Vec<RangeInclusive<u64>> = Vec::new();

        merged_ranges.push(sorted_iter.next().unwrap());

        sorted_iter.for_each(|s| {
            let merged_ranges_last_idx = merged_ranges.len() - 1;
            if s.overlapping(merged_ranges[merged_ranges_last_idx].clone())
                | s.adjacent(merged_ranges[merged_ranges_last_idx].clone())
            {
                merged_ranges[merged_ranges_last_idx] =
                    Mergeable::merge(&s, merged_ranges[merged_ranges_last_idx].clone());
            } else {
                merged_ranges.push(s);
            }
        });

        merged_ranges
            .iter()
            .map(|m| (m.end() - m.start()) + 1)
            .sum()
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

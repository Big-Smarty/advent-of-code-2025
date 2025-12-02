advent_of_code::solution!(1);

#[derive(Debug)]
enum Rotation {
    L(u16),
    R(u16),
}

struct Dial {
    pub position: u16,
}

impl Dial {
    pub fn rotate(&mut self, rotation: &Rotation) -> bool {
        match rotation {
            Rotation::L(r) => {
                let mod_r = r % 100;
                if mod_r > self.position {
                    self.position = 100 - (mod_r - self.position);
                } else {
                    self.position = self.position - mod_r;
                }
            }
            Rotation::R(r) => {
                self.position = (self.position + r) % 100;
            }
        }
        self.position == 0
    }

    pub fn rotate_2(&mut self, rotation: &Rotation) -> u16 {
        match rotation {
            Rotation::L(r) => {
                let mut out = 0;
                for _ in 0..*r {
                    if self.position == 0 {
                        self.position = 100;
                        out += 1;
                    }
                    self.position -= 1;
                }
                return out;
            }
            Rotation::R(r) => {
                let mut out = 0;
                for _ in 0..*r {
                    self.position += 1;
                    if self.position == 100 {
                        self.position = 0;
                        out += 1;
                    }
                }
                return out;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations: Vec<Rotation> = input
        .lines()
        .map(|l| {
            let (direction, count) = l.split_at(1);
            if direction == "R" {
                return Rotation::R(count.parse().unwrap());
            } else {
                return Rotation::L(count.parse().unwrap());
            }
        })
        .collect();

    let mut dial = Dial { position: 50 };

    Some(rotations.iter().fold(0, |mut count, r| {
        if dial.rotate(r) {
            count += 1;
        }
        count
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations: Vec<Rotation> = input
        .lines()
        .map(|l| {
            let (direction, count) = l.split_at(1);
            if direction == "R" {
                return Rotation::R(count.parse().unwrap());
            } else {
                return Rotation::L(count.parse().unwrap());
            }
        })
        .collect();

    let mut dial = Dial { position: 50 };

    Some(
        rotations
            .iter()
            .fold(0, |count, r| count + dial.rotate_2(r) as u64),
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

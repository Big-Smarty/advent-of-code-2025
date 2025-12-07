advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .filter(|l| !l.chars().all(|c| c == '.'))
        .collect::<Vec<&str>>();

    let mut cleared_lines: Vec<String> = Vec::new();

    for y in 0..lines.len() {
        cleared_lines.push(
            lines[y]
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if (c == '.') | (c == 'S') {
                        c
                    } else {
                        let mut out = '.';
                        // walk from bottom to top
                        for i in 1..=y {
                            // if encountered another splitter above: return . (clear because invalid)
                            if &cleared_lines[y - i][x..x + 1] == "^" {
                                break;
                            }
                            // if encountered neighbouring splitter to path: return ^ (beam can intersect current splitter)
                            else if (&cleared_lines[y - i][x - 1..x] == "^")
                                | (&cleared_lines[y - i][x + 1..x + 2] == "^")
                            {
                                out = '^';
                                break;
                            }
                            // if encountered starting point: return ^ (original beam can intersect current splitter)
                            else if &cleared_lines[y - i][x..x + 1] == "S" {
                                out = '^';
                                break;
                            }
                        }
                        out
                    }
                })
                .collect::<String>(),
        );
    }

    Some(
        cleared_lines
            .iter()
            .map(|l| {
                let mut encountered_splitter = false;
                let mut splitter_count = 0;
                for c in l.chars() {
                    if c != '^' && !encountered_splitter {
                        continue;
                    } else if c == '^' && !encountered_splitter {
                        encountered_splitter = true;
                        splitter_count += 1;
                    } else if c == '^' && encountered_splitter {
                        splitter_count += 1;
                    }
                }
                let out = splitter_count;
                out
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .filter(|l| !l.chars().all(|c| c == '.'))
        .collect::<Vec<&str>>();

    /*let mut cleared_lines: Vec<String> = Vec::new();

    for y in 0..lines.len() {
        cleared_lines.push(
            lines[y]
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    if (c == '.') | (c == 'S') {
                        c
                    } else {
                        let mut out = '.';
                        // walk from bottom to top
                        for i in 1..=y {
                            // if encountered another splitter above: return . (clear because invalid)
                            if &cleared_lines[y - i][x..x + 1] == "^" {
                                break;
                            }
                            // if encountered neighbouring splitter to path: return ^ (beam can intersect current splitter)
                            else if (&cleared_lines[y - i][x - 1..x] == "^")
                                | (&cleared_lines[y - i][x + 1..x + 2] == "^")
                            {
                                out = '^';
                                break;
                            }
                            // if encountered starting point: return ^ (original beam can intersect current splitter)
                            else if &cleared_lines[y - i][x..x + 1] == "S" {
                                out = '^';
                                break;
                            }
                        }
                        out
                    }
                })
                .collect::<String>(),
        );
    }*/

    let mut previous_timeline: Vec<u64> = Vec::new();
    previous_timeline.resize(lines[0].len(), 0);
    let mut new_timeline: Vec<u64> = previous_timeline.clone();

    for y in 0..lines.len() {
        if lines[y].contains("S") {
            new_timeline[lines[y].len() / 2] = 1;
        } else {
            new_timeline = {
                let mut out: Vec<u64> = Vec::new();
                out.resize(lines[0].len(), 0);
                lines[y].chars().enumerate().for_each(|(x, c)| {
                    let timeline_value = previous_timeline[x];
                    match c {
                        '^' => {
                            out[x - 1] += timeline_value;
                            out[x + 1] += timeline_value;
                        }
                        _ => {
                            out[x] += timeline_value;
                        }
                    }
                });
                out
            };
        }
        previous_timeline = new_timeline.clone();
    }

    Some(new_timeline.iter().sum())
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

use std::collections::{HashSet, VecDeque};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 16;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

        let mut queue = VecDeque::<((isize, isize), (isize, isize))>::new();
        queue.push_back(((0, 0), (1, 0)));

        let mut visited = HashSet::new();

        while let Some((pos, dir)) = queue.pop_front() {
            if pos.0 < 0
                || pos.1 < 0
                || pos.0 >= grid[0].len() as isize
                || pos.1 >= grid.len() as isize
            {
                continue;
            }

            let newly_inserted = visited.insert((pos, dir));
            if !newly_inserted {
                continue;
            }

            match (grid[pos.1 as usize][pos.0 as usize], dir) {
                (b'.', _) | (b'-', (_, 0)) | (b'|', (0, _)) => {
                    queue.push_back(((pos.0 + dir.0, pos.1 + dir.1), dir));
                }
                (b'/', (0, 1)) | (b'\\', (0, -1)) => {
                    queue.push_back(((pos.0 - 1, pos.1), (-1, 0)));
                }
                (b'/', (0, -1)) | (b'\\', (0, 1)) => {
                    queue.push_back(((pos.0 + 1, pos.1), (1, 0)));
                }
                (b'/', (1, 0)) | (b'\\', (-1, 0)) => {
                    queue.push_back(((pos.0, pos.1 - 1), (0, -1)));
                }
                (b'/', (-1, 0)) | (b'\\', (1, 0)) => {
                    queue.push_back(((pos.0, pos.1 + 1), (0, 1)));
                }
                (b'-', (0, _)) => {
                    queue.push_back(((pos.0 + 1, pos.1), (1, 0)));
                    queue.push_back(((pos.0 - 1, pos.1), (-1, 0)));
                }
                (b'|', (_, 0)) => {
                    queue.push_back(((pos.0, pos.1 + 1), (0, 1)));
                    queue.push_back(((pos.0, pos.1 - 1), (0, -1)));
                }
                (c, _) => {
                    panic!("unknown char: {c}");
                }
            }
        }

        visited
            .into_iter()
            .map(|(p, _)| p)
            .collect::<HashSet<_>>()
            .len()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

        (0..grid.len())
            .map(|y| ((0, y), (1, 0)))
            .chain((0..grid.len()).map(|y| ((grid[0].len() - 1, y), (-1, 0))))
            .chain((0..grid[0].len()).map(|x| ((x, 0), (0, 1))))
            .chain((0..grid[0].len()).map(|x| ((x, grid.len() - 1), (0, -1))))
            .map(|(pos, dir)| {
                let mut queue = VecDeque::<((isize, isize), (isize, isize))>::new();
                queue.push_back(((pos.0 as isize, pos.1 as isize), dir));

                let mut visited = HashSet::new();

                while let Some((pos, dir)) = queue.pop_front() {
                    if pos.0 < 0
                        || pos.1 < 0
                        || pos.0 >= grid[0].len() as isize
                        || pos.1 >= grid.len() as isize
                    {
                        continue;
                    }

                    let newly_inserted = visited.insert((pos, dir));
                    if !newly_inserted {
                        continue;
                    }

                    match (grid[pos.1 as usize][pos.0 as usize], dir) {
                        (b'.', _) | (b'-', (_, 0)) | (b'|', (0, _)) => {
                            queue.push_back(((pos.0 + dir.0, pos.1 + dir.1), dir));
                        }
                        (b'/', (0, 1)) | (b'\\', (0, -1)) => {
                            queue.push_back(((pos.0 - 1, pos.1), (-1, 0)));
                        }
                        (b'/', (0, -1)) | (b'\\', (0, 1)) => {
                            queue.push_back(((pos.0 + 1, pos.1), (1, 0)));
                        }
                        (b'/', (1, 0)) | (b'\\', (-1, 0)) => {
                            queue.push_back(((pos.0, pos.1 - 1), (0, -1)));
                        }
                        (b'/', (-1, 0)) | (b'\\', (1, 0)) => {
                            queue.push_back(((pos.0, pos.1 + 1), (0, 1)));
                        }
                        (b'-', (0, _)) => {
                            queue.push_back(((pos.0 + 1, pos.1), (1, 0)));
                            queue.push_back(((pos.0 - 1, pos.1), (-1, 0)));
                        }
                        (b'|', (_, 0)) => {
                            queue.push_back(((pos.0, pos.1 + 1), (0, 1)));
                            queue.push_back(((pos.0, pos.1 - 1), (0, -1)));
                        }
                        (c, _) => {
                            panic!("unknown char: {c}");
                        }
                    }
                }

                visited
                    .into_iter()
                    .map(|(p, _)| p)
                    .collect::<HashSet<_>>()
                    .len()
            })
            .max()
            .unwrap()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(46, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(7482, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(51, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(7896, output);
}

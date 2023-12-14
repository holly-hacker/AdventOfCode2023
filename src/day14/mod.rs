use core::panic;
use std::collections::{HashMap, HashSet};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 14;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // move to the north

        for _ in 0..grid.len() {
            for y in 1..grid.len() {
                for x in 0..grid.len() {
                    if grid[y][x] == 'O' && grid[y - 1][x] == '.' {
                        grid[y][x] = '.';
                        grid[y - 1][x] = 'O';
                    }
                }
            }
        }

        let mut sum = 0;
        for y in 0..grid.len() {
            for x in 0..grid.len() {
                if grid[y][x] == 'O' {
                    sum += grid.len() - y;
                }
            }
        }

        sum
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut visited = HashMap::<Vec<Vec<char>>, usize>::new();

        const MAX_CYCLES: usize = 1000000000;
        for cycle in 0..1000000000 {
            // up
            for _ in 0..grid.len() {
                for y in 1..grid.len() {
                    for x in 0..grid[0].len() {
                        if grid[y][x] == 'O' && grid[y - 1][x] == '.' {
                            grid[y][x] = '.';
                            grid[y - 1][x] = 'O';
                        }
                    }
                }
            }

            // left
            for _ in 0..grid[0].len() {
                for x in 1..grid[0].len() {
                    for y in 0..grid.len() {
                        if grid[y][x] == 'O' && grid[y][x - 1] == '.' {
                            grid[y][x] = '.';
                            grid[y][x - 1] = 'O';
                        }
                    }
                }
            }

            // down
            for _ in 0..grid.len() {
                for y in (0..(grid.len() - 1)).rev() {
                    for x in 0..grid[0].len() {
                        if grid[y][x] == 'O' && grid[y + 1][x] == '.' {
                            grid[y][x] = '.';
                            grid[y + 1][x] = 'O';
                        }
                    }
                }
            }

            // right
            for _ in 0..grid[0].len() {
                for x in (0..(grid[0].len() - 1)).rev() {
                    for y in 0..grid.len() {
                        if grid[y][x] == 'O' && grid[y][x + 1] == '.' {
                            grid[y][x] = '.';
                            grid[y][x + 1] = 'O';
                        }
                    }
                }
            }

            if let Some(last_cycle) = visited.get(&grid) {
                let cycle_len = cycle - last_cycle;
                let cycles_left = MAX_CYCLES - 1 - cycle;
                if cycles_left % cycle_len == 0 {
                    break;
                }
            }

            visited.insert(grid.clone(), cycle);
        }

        let mut sum = 0;
        for y in 0..grid.len() {
            for x in 0..grid.len() {
                if grid[y][x] == 'O' {
                    sum += grid.len() - y;
                }
            }
        }

        sum
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(136, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(111339, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(64, output);
}

// #[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(35799, output);
}

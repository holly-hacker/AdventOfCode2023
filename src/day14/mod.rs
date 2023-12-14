use std::collections::HashMap;

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
            let mut changed = false;

            for y in 1..grid.len() {
                for x in 0..grid.len() {
                    if grid[y][x] == 'O' && grid[y - 1][x] == '.' {
                        grid[y][x] = '.';
                        grid[y - 1][x] = 'O';
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
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
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut visited = HashMap::<Vec<Vec<u8>>, usize>::new();

        let width = grid[0].len();
        let height = grid.len();

        const MAX_ITERATIONS: usize = 1000000000;
        for current_iteration in 0..MAX_ITERATIONS {
            // up
            for _ in 1..height {
                let mut changed = false;

                for y in 1..width {
                    for x in 0..height {
                        if grid[y][x] == b'O' && grid[y - 1][x] == b'.' {
                            grid[y][x] = b'.';
                            grid[y - 1][x] = b'O';
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
            }

            // left
            for _ in 1..width {
                let mut changed = false;

                for x in 1..height {
                    for y in 0..width {
                        if grid[y][x] == b'O' && grid[y][x - 1] == b'.' {
                            grid[y][x] = b'.';
                            grid[y][x - 1] = b'O';
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
            }

            // down
            for _ in 1..height {
                let mut changed = false;

                for y in (0..(width - 1)).rev() {
                    for x in 0..height {
                        if grid[y][x] == b'O' && grid[y + 1][x] == b'.' {
                            grid[y][x] = b'.';
                            grid[y + 1][x] = b'O';
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
            }

            // right
            for _ in 1..width {
                let mut changed = false;

                for x in (0..(height - 1)).rev() {
                    for y in 0..width {
                        if grid[y][x] == b'O' && grid[y][x + 1] == b'.' {
                            grid[y][x] = b'.';
                            grid[y][x + 1] = b'O';
                            changed = true;
                        }
                    }
                }

                if !changed {
                    break;
                }
            }

            if let Some(last_iteration) = visited.get_mut(&grid) {
                // we've found a cycle! we can now calculate which iteration contains the grid the
                // final iteration would have had, so we don't need to go through the entire loop.
                let cycle = current_iteration - *last_iteration;
                let iterations_left = MAX_ITERATIONS - 1 - current_iteration;
                let last_iteration_with_correct_grid = *last_iteration + iterations_left % cycle;

                let (final_grid, _) = visited
                    .into_iter()
                    .find(|(_, v)| *v == last_iteration_with_correct_grid)
                    .expect("find final cycle");
                grid = final_grid;
                break;
            } else {
                visited.insert(grid.clone(), current_iteration);
            }
        }

        let mut sum = 0;
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == b'O' {
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

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(93736, output);
}

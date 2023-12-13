use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 13;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|grid| {
                let grid = grid
                    .lines()
                    .map(|line| line.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                let w = grid[0].len();
                let h = grid.len();

                let check_reflect_horizontal = |grid: &[Vec<_>], x: usize| {
                    for c in 1..=(x.min(w - x)) {
                        for y in 0..h {
                            if grid[y][x - c] != grid[y][x + c - 1] {
                                return false;
                            }
                        }
                    }

                    true
                };

                let check_reflect_vertical = |grid: &[Vec<_>], y: usize| {
                    for c in 1..=(y.min(h - y)) {
                        for x in 0..w {
                            if grid[y - c][x] != grid[y + c - 1][x] {
                                return false;
                            }
                        }
                    }

                    true
                };

                for x in 1..w {
                    if check_reflect_horizontal(&grid, x) {
                        return x;
                    }
                }

                for y in 1..h {
                    if check_reflect_vertical(&grid, y) {
                        return y * 100;
                    }
                }

                unreachable!("No solution found")
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .split("\n\n")
            .map(|grid| {
                let mut grid = grid
                    .lines()
                    .map(|line| line.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                let w = grid[0].len();
                let h = grid.len();

                let check_reflect_horizontal = |grid: &[Vec<_>], x: usize| {
                    for c in 1..=(x.min(w - x)) {
                        for y in 0..h {
                            if grid[y][x - c] != grid[y][x + c - 1] {
                                return false;
                            }
                        }
                    }

                    true
                };

                let check_reflect_vertical = |grid: &[Vec<_>], y: usize| {
                    for c in 1..=(y.min(h - y)) {
                        for x in 0..w {
                            if grid[y - c][x] != grid[y + c - 1][x] {
                                return false;
                            }
                        }
                    }

                    true
                };

                let mut mirror_horizontal = None;
                let mut mirror_vertical = None;
                for x in 1..w {
                    if check_reflect_horizontal(&grid, x) {
                        mirror_horizontal = Some(x);
                    }
                }

                for y in 1..h {
                    if check_reflect_vertical(&grid, y) {
                        mirror_vertical = Some(y);
                    }
                }

                for mut_x in 0..w {
                    for mut_y in 0..h {
                        grid[mut_y][mut_x] = match grid[mut_y][mut_x] {
                            '#' => '.',
                            '.' => '#',
                            _ => unreachable!(),
                        };

                        for x in 1..(w) {
                            if mirror_horizontal != Some(x) && check_reflect_horizontal(&grid, x) {
                                return x;
                            }
                        }

                        for y in 1..(h) {
                            if mirror_vertical != Some(y) && check_reflect_vertical(&grid, y) {
                                return y * 100;
                            }
                        }

                        // revert it again if no match was found
                        grid[mut_y][mut_x] = match grid[mut_y][mut_x] {
                            '#' => '.',
                            '.' => '#',
                            _ => unreachable!(),
                        };
                    }
                }

                unreachable!("No solution found")
            })
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(405, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(36448, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(400, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(35799, output);
}

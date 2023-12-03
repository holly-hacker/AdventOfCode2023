use std::collections::BTreeSet;

use crate::utils::*;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 3;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let grid = input
            .lines()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut sum = 0;
        for (y, line) in grid.iter().enumerate() {
            let mut line_idx = 0;
            loop {
                if line_idx >= line.len() {
                    break;
                }

                let byte_run = line
                    .iter()
                    .skip(line_idx)
                    .copied()
                    .take_while(u8::is_ascii_digit)
                    .collect::<Vec<_>>();

                if byte_run.is_empty() {
                    line_idx += 1;
                    continue;
                }

                let parsed_part_num = fast_parse_int_from_bytes(byte_run.as_slice());

                let b = (line_idx..(line_idx + byte_run.len())).any(|x| {
                    let neighbours = [
                        grid[y.saturating_sub(1)][x.saturating_sub(1)],
                        grid[y.saturating_sub(1)][x],
                        grid[y.saturating_sub(1)][(x + 1) % line.len()],
                        grid[y][x.saturating_sub(1)],
                        // grid[y][x],
                        grid[y][(x + 1) % line.len()],
                        grid[(y + 1) % grid.len()][x.saturating_sub(1)],
                        grid[(y + 1) % grid.len()][x],
                        grid[(y + 1) % grid.len()][(x + 1) % line.len()],
                    ];
                    neighbours.iter().any(|n| *n != b'.' && !n.is_ascii_digit())
                });

                if b {
                    sum += parsed_part_num;
                }
                line_idx += byte_run.len();
            }
        }
        sum
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let grid = input
            .lines()
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<_>>();

        let mut sum = 0;
        for (y, line) in grid.iter().enumerate() {
            for (x, char) in line.iter().enumerate() {
                if *char != b'*' {
                    continue;
                }

                // find the 2 neighbouring numbers, if any
                let neighbours = [
                    (y.saturating_sub(1), x.saturating_sub(1)),
                    (y.saturating_sub(1), x),
                    (y.saturating_sub(1), (x + 1) % line.len()),
                    (y, x.saturating_sub(1)),
                    (y, (x + 1) % line.len()),
                    ((y + 1) % grid.len(), x.saturating_sub(1)),
                    ((y + 1) % grid.len(), x),
                    ((y + 1) % grid.len(), (x + 1) % line.len()),
                ];
                let set = neighbours
                    .into_iter()
                    .filter(|(y, x)| grid[*y][*x].is_ascii_digit())
                    .map(|(y, x)| {
                        // check to the left for numbers
                        let mut left = x;
                        while left != 0 && grid[y][left - (1)].is_ascii_digit() {
                            left = left.saturating_sub(1);
                        }
                        let mut right = x;
                        while right != line.len() - 1 && grid[y][right + 1].is_ascii_digit() {
                            right = right.saturating_add(1);
                        }

                        ((left, right), y)
                    })
                    .collect::<BTreeSet<_>>();

                if set.len() != 2 {
                    // we want 2 numbers
                    continue;
                }

                // get first and second item from set
                let mut iter = set.iter();
                let num1 = iter.next().unwrap();
                let num1 = grid[num1.1][num1.0 .0..=num1.0 .1].to_vec();
                let num1 = fast_parse_int_from_bytes(&num1);

                let num2 = iter.next().unwrap();
                let num2 = grid[num2.1][num2.0 .0..=num2.0 .1].to_vec();
                let num2 = fast_parse_int_from_bytes(&num2);
                sum += num1 * num2;
            }
        }
        sum
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(4361, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(544664, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(467835, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(84495585, output);
}

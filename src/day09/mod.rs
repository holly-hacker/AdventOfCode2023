#![allow(clippy::naive_bytecount)]

use std::collections::VecDeque;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .map(|l| {
                let first_row = l
                    .split(' ')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();

                let mut all_rows = vec![first_row.clone()];
                while !all_rows.last().unwrap().windows(2).all(|w| w[0] == w[1]) {
                    let mut new_row = Vec::new();
                    for w in all_rows.last().unwrap().windows(2) {
                        new_row.push(w[1] - w[0]);
                    }
                    all_rows.push(new_row);
                }

                for i in (1..all_rows.len()).rev() {
                    let up = *all_rows[i - 1].last().unwrap();
                    let cur = *all_rows[i].last().unwrap();
                    all_rows[i - 1].push(up + cur);
                }

                *all_rows[0].last().unwrap()
            })
            .sum::<isize>() as usize
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .lines()
            .map(|l| {
                let first_row = l
                    .split(' ')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<VecDeque<_>>();

                let mut all_rows = vec![first_row.clone()];
                while !all_rows.last().unwrap().iter().all(|b| *b == 0) {
                    let mut new_row = VecDeque::new();
                    // TODO: pretty hacky, improve this
                    for w in all_rows
                        .last()
                        .unwrap()
                        .iter()
                        .collect::<Vec<_>>()
                        .windows(2)
                        .rev()
                    {
                        new_row.push_front(w[1] - w[0]);
                    }
                    all_rows.push(new_row);
                }

                for i in (1..all_rows.len()).rev() {
                    let up = all_rows[i - 1][0];
                    let cur = all_rows[i][0];
                    all_rows[i - 1].push_front(up - cur);
                }

                all_rows[0][0]
            })
            .sum::<isize>() as usize
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(114, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(1757008019, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(2, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(995, output);
}

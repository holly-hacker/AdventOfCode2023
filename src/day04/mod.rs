use std::collections::{HashMap, HashSet};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 4;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let (_, line) = line.split_once(':').unwrap();
                let (good, gotten) = line.split_once(" | ").unwrap();
                let good_nums = good
                    .split(' ')
                    .flat_map(|num| num.parse::<usize>())
                    .collect::<HashSet<_>>();
                let gotten_nums = gotten
                    .split(' ')
                    .flat_map(|num| num.parse::<usize>())
                    .collect::<HashSet<_>>();

                // intersect
                let count = gotten_nums
                    .iter()
                    .filter(|gotten_num| good_nums.contains(gotten_num))
                    .count();

                if count > 0 {
                    usize::pow(2, (count - 1) as u32)
                } else {
                    0
                }
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut scratchcard_count = HashMap::<usize, usize>::new();
        input.lines().enumerate().for_each(|(idx, line)| {
            let card_no = idx + 1;
            *scratchcard_count.entry(card_no).or_default() += 1;
            let current_count = scratchcard_count.get(&card_no).cloned().unwrap_or_default();

            let (_, line) = line.split_once(':').unwrap();
            let (good, gotten) = line.split_once(" | ").unwrap();
            let good_nums = good
                .split(' ')
                .flat_map(|num| num.parse::<usize>())
                .collect::<HashSet<_>>();
            let gotten_nums = gotten
                .split(' ')
                .flat_map(|num| num.parse::<usize>())
                .collect::<HashSet<_>>();

            // intersect
            let count = gotten_nums
                .iter()
                .filter(|gotten_num| good_nums.contains(gotten_num))
                .count();

            for i in 0..count {
                *scratchcard_count.entry(card_no + i + 1).or_default() += current_count;
            }
        });

        scratchcard_count.values().sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(13, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(25183, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(30, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(5667240, output);
}

use std::collections::VecDeque;

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
                let (_, line) = line.split_once(": ").unwrap();
                let (want, have) = line.split_once(" | ").unwrap();

                // create a lookup table for the winning numbers
                // this is faster than iterating over a collection
                let mut lut = [false; 100];
                want.as_bytes()
                    .chunks(3)
                    // NOTE: space is 0x20 which would become 0, no need to check for that :)
                    .map(|c| (c[0] & 0x0F) * 10 + (c[1] & 0x0F))
                    .for_each(|b| {
                        lut[b as usize] = true;
                    });

                // create iterator to parse gotten numbers
                let matches = have
                    .as_bytes()
                    .chunks(3)
                    .map(|c| (c[0] & 0x0F) * 10 + (c[1] & 0x0F))
                    .filter(|gotten_num| lut[*gotten_num as usize])
                    .count();

                if matches > 0 {
                    1 << (matches - 1) as u32
                } else {
                    0
                }
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut next_scratchcard_count = VecDeque::<usize>::new();

        input
            .lines()
            .map(|line| {
                let (_, line) = line.split_once(": ").unwrap();
                let (want, have) = line.split_once(" | ").unwrap();

                // create a lookup table for the winning numbers
                // this is faster than iterating over a collection
                let mut lut = [false; 100];
                want.as_bytes()
                    .chunks(3)
                    // NOTE: space is 0x20 which would become 0, no need to check for that :)
                    .map(|c| (c[0] & 0x0F) * 10 + (c[1] & 0x0F))
                    .for_each(|b| {
                        lut[b as usize] = true;
                    });

                // create iterator to parse gotten numbers
                let matches = have
                    .as_bytes()
                    .chunks(3)
                    .map(|c| (c[0] & 0x0F) * 10 + (c[1] & 0x0F))
                    .filter(|gotten_num| lut[*gotten_num as usize])
                    .count();

                let current_count = next_scratchcard_count.pop_front().unwrap_or_default() + 1;
                for i in 0..matches {
                    if next_scratchcard_count.len() < i + 1 {
                        next_scratchcard_count.push_back(0);
                    }
                    next_scratchcard_count[i] += current_count;
                }
                current_count
            })
            .sum()
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

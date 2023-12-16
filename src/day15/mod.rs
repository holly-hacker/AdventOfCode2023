use std::collections::HashMap;

use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 15;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .as_bytes()
            .split(|b| *b == b',')
            .map(|seq| {
                let val = seq.iter().fold(0, |mut acc, c| {
                    acc += *c as usize;
                    acc *= 17;
                    acc %= 256;
                    acc
                });

                val
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut map = HashMap::<usize, Vec<(u32, usize)>>::new();
        input.as_bytes().split(|b| *b == b',').for_each(|bytes| {
            let label = &bytes[..bytes.iter().position(|b| *b == b'-' || *b == b'=').unwrap()];

            let op = &bytes[label.len()..];

            // labels are ascii 'a'-'z', which we can mask to 5 bits
            // the longest label in the input is 6 chars, so that fits in 32 bits
            let label_bits = label
                .iter()
                .enumerate()
                .fold(0u32, |acc, (i, b)| acc + ((*b as u32 & 0b11111) << (i * 5)));

            let label_hash = label.iter().fold(0, |mut acc, c| {
                acc += *c as usize;
                acc *= 17;
                acc %= 256;
                acc
            });

            if op[0] == b'=' {
                let last = fast_parse_int_from_bytes(&op[1..]);
                let vec = map.entry(label_hash).or_default();
                if let Some(v) = vec.iter_mut().find(|i| i.0 == label_bits) {
                    v.1 = last;
                } else {
                    vec.push((label_bits, last));
                }
            } else if op[0] == b'-' {
                let vec = map.entry(label_hash).or_default();
                if let Some(v) = vec.iter_mut().position(|i| i.0 == label_bits) {
                    vec.remove(v);
                }
            } else {
                unreachable!("invalid op: {:?}", op);
            }
        });

        map.into_iter()
            .map(|(box_idx, v)| {
                v.iter()
                    .enumerate()
                    .map(|(pos, focal_len)| (box_idx + 1) * (pos + 1) * (focal_len.1))
                    .sum::<usize>()
            })
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(1320, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(511257, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(145, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(239484, output);
}

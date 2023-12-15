use std::collections::HashMap;

use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 14;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .split(',')
            .map(|seq| {
                let val = seq.bytes().fold(0, |mut acc, c| {
                    acc += c as usize;
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
        let mut map = HashMap::<usize, Vec<(Vec<u8>, usize)>>::new();

        input.split(',').for_each(|seq| {
            let bytes = seq.as_bytes();
            let label = &bytes[..bytes
                .iter()
                .position(|b| !b.is_ascii_alphanumeric())
                .unwrap()];
            let op = &bytes[label.len()..];

            let label_hash = label.iter().fold(0, |mut acc, c| {
                acc += *c as usize;
                acc *= 17;
                acc %= 256;
                acc
            });

            if op[0] == b'=' {
                let last = fast_parse_int_from_bytes(&op[1..]);
                let vec = map.entry(label_hash).or_default();
                if let Some(v) = vec.iter_mut().find(|i| i.0 == label) {
                    v.1 = last;
                } else {
                    vec.push((label.to_vec(), last));
                }
            } else if op[0] == b'-' {
                let vec = map.entry(label_hash).or_default();
                if let Some(v) = vec.iter_mut().position(|i| i.0 == label) {
                    vec.remove(v);
                }
            } else {
                unreachable!("invalid op: {:?}", op);
            }
        });

        map.into_iter()
            .map(|(_box, v)| {
                v.iter()
                    .enumerate()
                    .map(|(pos, focal_len)| (_box + 1) * (pos + 1) * (focal_len.1))
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

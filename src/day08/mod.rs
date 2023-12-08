#![allow(clippy::naive_bytecount)]

use std::collections::HashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 8;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut lines = input.lines();
        let instructions = lines.next().unwrap();
        _ = lines.next().unwrap();

        let map = lines
            .map(|l| {
                let node: [u8; 3] = (&l.as_bytes()[..3]).try_into().unwrap();
                let left: [u8; 3] = (&l.as_bytes()[7..10]).try_into().unwrap();
                let right: [u8; 3] = (&l.as_bytes()[12..15]).try_into().unwrap();

                (node, (left, right))
            })
            .collect::<HashMap<_, _>>();

        let mut current = *b"AAA";
        for repeat in 0.. {
            for (i, b) in instructions.bytes().enumerate() {
                if current[0] == b'Z' && current[1] == b'Z' && current[2] == b'Z' {
                    return repeat * instructions.len() + i;
                }
                current = match b {
                    b'L' => map[&current].0,
                    b'R' => map[&current].1,
                    _ => unreachable!(),
                };
            }
        }

        unreachable!()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let mut lines = input.lines();
        let instructions = lines.next().unwrap();
        _ = lines.next().unwrap();

        let nodes = lines
            .map(|l| {
                let node: [u8; 3] = (&l.as_bytes()[..3]).try_into().unwrap();
                let left: [u8; 3] = (&l.as_bytes()[7..10]).try_into().unwrap();
                let right: [u8; 3] = (&l.as_bytes()[12..15]).try_into().unwrap();

                (node, (left, right))
            })
            .collect::<HashMap<_, _>>();

        // figure out how long it takes to get to the end of each loop
        nodes
            .keys()
            .filter(|k| k[2] == b'A')
            .cloned()
            .map(|node| {
                let mut current = node;
                for count in 0.. {
                    for instruction in instructions.bytes() {
                        current = match instruction {
                            b'L' => nodes[&current].0,
                            b'R' => nodes[&current].1,
                            _ => unreachable!(),
                        };
                    }

                    if current[2] == b'Z' {
                        return count + 1;
                    }
                }

                unreachable!()
            })
            .reduce(lcm)
            .unwrap()
            * instructions.len()
    }
}

// https://www.hackertouch.com/least-common-multiple-in-rust.html
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(6, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(17287, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(6, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(18625484023687, output);
}

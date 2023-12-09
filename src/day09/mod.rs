use std::cell::Cell;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 9;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut buffer = vec![];
        input
            .lines()
            .map(|l| {
                // re-usable buffer for cache efficiency and reducing heap allocs
                buffer.clear();
                buffer.extend(l.split(' ').map(|n| n.parse::<isize>().unwrap()));

                // the amount of rows we currently have
                let mut depth = 0;
                let len = buffer.len();
                loop {
                    let subslice = &mut buffer[..(len - depth)];
                    if subslice.iter().skip(1).all(|w| subslice[0] == *w) {
                        // all good
                        break;
                    }

                    // hack described by the `.windows(_)` docs, to emulate `.windows_mut(_)`
                    let buffer_cells = Cell::from_mut(subslice).as_slice_of_cells();
                    buffer_cells.windows(2).for_each(|w| {
                        w[0].set(w[1].get() - w[0].get());
                    });

                    depth += 1;
                }

                let start_index = buffer.len() - 1 - depth;
                buffer[start_index..].iter().sum::<isize>()
            })
            .sum::<isize>() as usize
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut buffer = vec![];
        input
            .lines()
            .map(|l| {
                buffer.clear();
                buffer.extend(l.split(' ').map(|n| n.parse::<isize>().unwrap()));

                let mut depth = 0;
                loop {
                    let subslice = &mut buffer[depth..];
                    if subslice.iter().skip(1).all(|w| subslice[0] == *w) {
                        break;
                    }

                    let buffer_cells = Cell::from_mut(subslice).as_slice_of_cells();
                    buffer_cells.windows(2).rev().for_each(|w| {
                        w[1].set(w[1].get() - w[0].get());
                    });

                    depth += 1;
                }

                let start_index = depth + 1;
                buffer[..start_index]
                    .iter()
                    .enumerate()
                    .map(|(i, v)| if i % 2 == 0 { *v } else { -*v })
                    .sum::<isize>()
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

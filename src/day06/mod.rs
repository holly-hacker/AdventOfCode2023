use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 6;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (time, distance) = input.split_once('\n').unwrap();
        let times = time.as_bytes()[11..]
            .split(|b| *b == b' ')
            .filter(|s| !s.is_empty())
            .map(fast_parse_int_from_bytes)
            .collect::<Vec<_>>();
        let distances = distance.as_bytes()[11..]
            .split(|b| *b == b' ')
            .filter(|s| !s.is_empty())
            .map(fast_parse_int_from_bytes)
            .collect::<Vec<_>>();

        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| {
                let mut count = 0;
                for push_time in 0..t {
                    let travel = push_time * (t - push_time);
                    if travel > d {
                        count += 1
                    }
                }

                count
            })
            .product()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let (time, distance) = input.split_once('\n').unwrap();
        let times = time.as_bytes()[11..]
            .split(|b| *b == b' ')
            .filter(|s| !s.is_empty())
            .map(fast_parse_int_from_bytes)
            .collect::<Vec<_>>();
        let distances = distance.as_bytes()[11..]
            .split(|b| *b == b' ')
            .filter(|s| !s.is_empty())
            .map(fast_parse_int_from_bytes)
            .collect::<Vec<_>>();

        let (t, d) = times.into_iter().zip(distances).fold((0, 0), |acc, val| {
            (
                format!("{}{}", acc.0, val.0).parse::<usize>().unwrap(),
                format!("{}{}", acc.1, val.1).parse::<usize>().unwrap(),
            )
        });

        {
            let mut count = 0;
            for push_time in 0..t {
                let travel = push_time * (t - push_time);
                if travel > d {
                    count += 1
                }
            }

            count
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(288, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(220320, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(71503, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(34454850, output);
}

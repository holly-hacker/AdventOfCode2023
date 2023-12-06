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
            .map(fast_parse_int_from_bytes);
        let distances = distance.as_bytes()[11..]
            .split(|b| *b == b' ')
            .filter(|s| !s.is_empty())
            .map(fast_parse_int_from_bytes);

        times
            .into_iter()
            .zip(distances)
            .map(|(t, d)| {
                // find push_time so that push_time * (t-push_time) > d
                // push_time*t - push_time^2 > d
                // push_time^2 - push_time*t - d == 0 (rounded somehow)

                // formula for second-degree polynomial is (-b +- sqrt(b^2 - 4ac)) / 2a

                let a = 1.;
                let b = t as f32;
                let c = d as f32;
                let x1 = (b + f32::sqrt(b * b - 4. * a * c)) / (2. * a);
                let x2 = (b - f32::sqrt(b * b - 4. * a * c)) / (2. * a);

                debug_assert!((x1 * b - x1 * x1 - c).abs() < 0.001);
                debug_assert!((x2 * b - x2 * x2 - c).abs() < 0.001);

                let (x_min, x_max) = (x1.min(x2), x1.max(x2));
                let x_min = (x_min + 1.).floor();
                let x_max = (x_max - 1.).ceil();

                (x_max - x_min + 1.) as usize
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
            .map(fast_parse_int_from_bytes);
        let distances = distance.as_bytes()[11..]
            .split(|b| *b == b' ')
            .filter(|s| !s.is_empty())
            .map(fast_parse_int_from_bytes);

        let (t, d) = times.into_iter().zip(distances).fold((0, 0), |acc, val| {
            (
                acc.0 * 10f32.powi((val.0 as f32).log10() as i32 + 1) as usize + val.0,
                acc.1 * 10f32.powi((val.1 as f32).log10() as i32 + 1) as usize + val.1,
            )
        });

        // find push_time so that push_time * (t-push_time) > d
        // push_time*t - push_time^2 > d
        // push_time^2 - push_time*t - d == 0 (rounded somehow)

        // formula for second-degree polynomial is (-b +- sqrt(b^2 - 4ac)) / 2a

        // NOTE: the numbers in part 2 are a lot larger so 64-bit floats are required
        let a = 1.;
        let b = t as f64;
        let c = d as f64;
        let x1 = (b + f64::sqrt(b * b - 4. * a * c)) / (2. * a);
        let x2 = (b - f64::sqrt(b * b - 4. * a * c)) / (2. * a);

        debug_assert!((x1 * b - x1 * x1 - c).abs() < 0.001);
        debug_assert!((x2 * b - x2 * x2 - c).abs() < 0.001);

        let (x_min, x_max) = (x1.min(x2), x1.max(x2));
        let x_min = (x_min + 1.).floor();
        let x_max = (x_max - 1.).ceil();

        (x_max - x_min + 1.) as usize
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

use super::*;
use crate::utils::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = "";
    const INPUT_REAL: &'static str = "";
    // const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    // const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        todo!()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        todo!()
    }
}

#[test]
fn test_silver_sample() {
    // let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    // assert_eq!(0, output);
}

#[test]
fn test_silver_real() {
    // let output = Day::calculate_silver(Day::INPUT_REAL);
    // assert_eq!(0, output);
}

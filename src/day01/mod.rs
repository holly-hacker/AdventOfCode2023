use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let lines: usize = input
            .lines()
            .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>())
            .map(|v| format!("{}{}", v[0], v.last().cloned().unwrap()))
            .map(|v| v.parse::<usize>().unwrap())
            .sum();
        lines
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let numbers = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let lines: usize = input
            .lines()
            .map(|l| {
                (0..l.len())
                    .map(|i| l.split_at(i).1)
                    .filter_map(|substr| {
                        numbers.iter().enumerate().find_map(|(i, n)| {
                            if substr.chars().next().map(|c| c.is_ascii_digit()) == Some(true) {
                                Some(substr.chars().next().unwrap().to_digit(10).unwrap() as usize)
                            } else if substr.starts_with(n) {
                                Some(i + 1)
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .map(|v| format!("{}{}", v[0], v.last().cloned().unwrap()))
            .map(|v| v.parse::<usize>().unwrap())
            .sum();
        lines
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(142, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(57346, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(281, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(57345, output);
}

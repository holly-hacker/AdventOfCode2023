use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 1;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let lines: usize = input
            .as_bytes()
            .split(|c| *c == b'\n')
            .map(|l| l.iter().filter(|c| *c & 0xF0 == 0x30))
            .map(|mut v| {
                let first = v.next().unwrap() & 0x0F;
                let last = v.last().unwrap_or(&first) & 0x0F;
                (first * 10 + last) as usize
            })
            .sum();
        lines
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        let lines: usize = input
            .as_bytes()
            .split(|c| *c == b'\n')
            .map(|l| {
                let mut skip_ahead = 0;
                (0..l.len())
                    .map(|i| l.split_at(i).1)
                    .filter_map(move |substr| {
                        if skip_ahead > 0 {
                            skip_ahead -= 1;
                            return None;
                        }
                        if let Some(digit) = substr.first().and_then(|c| {
                            if *c & 0xF0 == 0x30 {
                                Some(c & 0x0F)
                            } else {
                                None
                            }
                        }) {
                            Some(digit as usize)
                        } else {
                            // if a number is found, skip ahead the bytes that are guaranteed to not match another number
                            const SKIP_AHEAD_COUNT: [usize; 9] = [2, 2, 3, 4, 2, 3, 4, 4, 3];

                            match substr.first() {
                                Some(b'o') if substr.starts_with(b"one") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[0] - 1;
                                    Some(1)
                                }
                                Some(b't') if substr.starts_with(b"two") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[1] - 1;
                                    Some(2)
                                }
                                Some(b't') if substr.starts_with(b"three") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[2] - 1;
                                    Some(3)
                                }
                                Some(b'f') if substr.starts_with(b"four") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[3] - 1;
                                    Some(4)
                                }
                                Some(b'f') if substr.starts_with(b"five") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[4] - 1;
                                    Some(5)
                                }
                                Some(b's') if substr.starts_with(b"six") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[5] - 1;
                                    Some(6)
                                }
                                Some(b's') if substr.starts_with(b"seven") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[6] - 1;
                                    Some(7)
                                }
                                Some(b'e') if substr.starts_with(b"eight") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[7] - 1;
                                    Some(8)
                                }
                                Some(b'n') if substr.starts_with(b"nine") => {
                                    skip_ahead = SKIP_AHEAD_COUNT[8] - 1;
                                    Some(9)
                                }
                                _ => None,
                            }
                        }
                    })
            })
            .map(|mut v| {
                let first = v.next().unwrap() & 0xF;
                let last = v.last().unwrap_or(first) & 0xF;
                first * 10 + last
            })
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

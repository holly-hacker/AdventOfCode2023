use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 2;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .filter_map(|l| {
                let (id, game) = l.split_at("Game ".len()).1.split_once(": ").unwrap();

                game.split("; ")
                    .all(|set| {
                        let mut red = 0;
                        let mut green = 0;
                        let mut blue = 0;

                        set.split(", ").for_each(|draw| {
                            let (num, col) = draw.split_once(' ').unwrap();
                            let num = fast_parse_int(num);

                            match col.as_bytes()[0] {
                                b'r' => red = num,
                                b'g' => green = num,
                                b'b' => blue = num,
                                _ => {
                                    debug_assert!(false, "Unknown color: {col}");
                                }
                            }
                        });

                        red <= 12 && green <= 13 && blue <= 14
                    })
                    .then(|| fast_parse_int(id))
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .lines()
            .map(|l| {
                let (_, game) = l.split_once(": ").unwrap();

                let folded = game.split("; ").fold((0, 0, 0), |acc, set| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    set.split(", ").for_each(|draw| {
                        let (num, col) = draw.split_once(' ').unwrap();
                        let num = fast_parse_int(num);

                        match col.as_bytes()[0] {
                            b'r' => red = num,
                            b'g' => green = num,
                            b'b' => blue = num,
                            _ => {
                                debug_assert!(false, "Unknown color: {col}");
                            }
                        }
                    });

                    (acc.0.max(red), acc.1.max(green), acc.2.max(blue))
                });

                folded.0 * folded.1 * folded.2
            })
            .sum()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(8, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(2545, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(2286, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(78111, output);
}

use std::collections::HashMap;

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
                let (id, game) = l.split_once(": ").unwrap();

                game.split("; ")
                    .all(|set| {
                        let map = set
                            .split(", ")
                            .map(|draw| {
                                let (num, col) = draw.split_once(' ').unwrap();

                                (col, num.parse::<usize>().unwrap())
                            })
                            .collect::<HashMap<&str, usize>>();

                        !((map.get("red").cloned()).unwrap_or_default() > 12
                            || (map.get("green").cloned()).unwrap_or_default() > 13
                            || (map.get("blue").cloned()).unwrap_or_default() > 14)
                    })
                    .then(|| (id.split_once(' ').unwrap().1).parse::<usize>().unwrap())
            })
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    // const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold.txt");

    fn calculate_gold(input: &str) -> usize {
        input
            .lines()
            .map(|l| {
                let (_, game) = l.split_once(": ").unwrap();

                let folded = game.split("; ").fold((0, 0, 0), |acc, set| {
                    let map = set
                        .split(", ")
                        .map(|draw| {
                            let (num, col) = draw.split_once(' ').unwrap();

                            (col, num.parse::<usize>().unwrap())
                        })
                        .collect::<HashMap<&str, usize>>();

                    (
                        usize::max((map.get("red").cloned()).unwrap_or_default(), acc.0),
                        usize::max((map.get("green").cloned()).unwrap_or_default(), acc.1),
                        usize::max((map.get("blue").cloned()).unwrap_or_default(), acc.2),
                    )
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

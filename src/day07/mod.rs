use std::collections::{HashMap, HashSet};

use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut plays = input
            .lines()
            .map(|l| {
                let (hand, score) = l.split_once(' ').unwrap();
                let score = score.parse::<usize>().unwrap();
                (hand, score)
            })
            .collect::<Vec<(_, _)>>();

        fn get_hand_rank(hand: &str) -> usize {
            assert_eq!(hand.len(), 5);

            let char_count = hand.chars().fold(HashMap::<char, usize>::new(), |acc, a| {
                let mut acc = acc;
                let count = acc.entry(a).or_insert(0);
                *count += 1;
                acc
            });

            if char_count.values().any(|v| *v == 5) {
                return 6; // 5 of a kind
            }
            if char_count.values().any(|v| *v == 4) {
                return 5; // 4 of a kind
            }
            if char_count.values().any(|v| *v == 3) && char_count.values().any(|v| *v == 2) {
                return 4; // full house
            }
            if char_count.values().any(|v| *v == 3) {
                return 3; // 3 of a kind
            }
            if char_count.values().filter(|v| **v == 2).count() == 2 {
                return 2; // 2 pair
            }
            if char_count.values().any(|v| *v == 2) {
                return 1; // pair
            }

            0
        }

        fn get_value(c: char) -> usize {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                _ => fast_parse_int_from_bytes(&[c as u8]),
            }
        }

        plays.sort_by(|(hand1, _), (hand2, _)| {
            let hand1_rank = get_hand_rank(hand1);
            let hand2_rank = get_hand_rank(hand2);
            match hand1_rank.cmp(&hand2_rank) {
                std::cmp::Ordering::Equal => hand1
                    .chars()
                    .zip(hand2.chars())
                    .filter_map(|(a, b)| {
                        let a = get_value(a);
                        let b = get_value(b);
                        match a.cmp(&b) {
                            std::cmp::Ordering::Equal => None,
                            o => Some(o),
                        }
                    })
                    .next()
                    .unwrap_or(std::cmp::Ordering::Equal),
                o => o,
            }
        });

        plays
            .into_iter()
            .enumerate()
            .map(|(i, (_, val))| (i + 1) * val)
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut plays = input
            .lines()
            .map(|l| {
                let (hand, score) = l.split_once(' ').unwrap();
                let score = score.parse::<usize>().unwrap();
                (hand, score)
            })
            .collect::<Vec<(_, _)>>();

        fn get_value(c: char) -> usize {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                _ => fast_parse_int_from_bytes(&[c as u8]),
            }
        }

        plays.sort_by(|(hand1, _), (hand2, _)| {
            let hand1_rank = get_hand_rank_2(hand1);
            let hand2_rank = get_hand_rank_2(hand2);
            match hand1_rank.cmp(&hand2_rank) {
                std::cmp::Ordering::Equal => hand1
                    .chars()
                    .zip(hand2.chars())
                    .filter_map(|(a, b)| {
                        let a = get_value(a);
                        let b = get_value(b);
                        match a.cmp(&b) {
                            std::cmp::Ordering::Equal => None,
                            o => Some(o),
                        }
                    })
                    .next()
                    .unwrap_or(std::cmp::Ordering::Equal),
                o => o,
            }
        });

        plays
            .into_iter()
            .enumerate()
            .map(|(i, (_, val))| (i + 1) * val)
            .sum()
    }
}

fn get_hand_rank_2(hand: &str) -> usize {
    assert_eq!(hand.len(), 5);

    let char_count = hand.chars().fold(HashMap::<char, usize>::new(), |acc, a| {
        let mut acc = acc;

        if a == 'J' {
            *acc.entry('2').or_insert(0) += 1;
            *acc.entry('3').or_insert(0) += 1;
            *acc.entry('4').or_insert(0) += 1;
            *acc.entry('5').or_insert(0) += 1;
            *acc.entry('6').or_insert(0) += 1;
            *acc.entry('7').or_insert(0) += 1;
            *acc.entry('8').or_insert(0) += 1;
            *acc.entry('9').or_insert(0) += 1;
            *acc.entry('T').or_insert(0) += 1;
            // *acc.entry('J').or_insert(0) += 1;
            *acc.entry('Q').or_insert(0) += 1;
            *acc.entry('K').or_insert(0) += 1;
            *acc.entry('A').or_insert(0) += 1;
        } else {
            *acc.entry(a).or_insert(0) += 1;
        }

        acc
    });

    // AAJBB -> full house
    // AJJJ2 -> 4 of a kind
    // ABCJJ -> not full house
    // 1234J -> not 2 pair!
    // the way to know if we have a full house with this method is to check if we have
    // exactly 2 different cards, and both have a count of at least 2
    let card_type_count_without_jokers = hand.chars().filter(|c| *c != 'J').collect::<HashSet<_>>();
    let joker_count = hand.chars().filter(|c| *c == 'J').count();

    if char_count.values().any(|v| *v >= 5) {
        return 6; // 5 of a kind
    }
    if char_count.values().any(|v| *v >= 4) {
        return 5; // 4 of a kind
    }
    if char_count.values().filter(|v| **v >= 2).count() >= 2
        && card_type_count_without_jokers.len() == 2
    {
        return 4; // full house
    }
    if char_count.values().any(|v| *v >= 3) {
        return 3; // 3 of a kind
    }
    if joker_count >= 2
        || (joker_count == 1 && card_type_count_without_jokers.len() < 4)
        || (joker_count == 0 && char_count.values().filter(|v| **v >= 2).count() >= 2)
    {
        return 2; // 2 pair
    }
    if char_count.values().any(|v| *v >= 2) {
        return 1; // pair
    }

    0
}

#[test]
fn test_get_rank_hand_2() {
    assert_eq!(6, get_hand_rank_2("AAAAA"));
    assert_eq!(5, get_hand_rank_2("AAAA1"));
    assert_eq!(4, get_hand_rank_2("22333"));
    assert_eq!(3, get_hand_rank_2("11123"));
    assert_eq!(2, get_hand_rank_2("11223"));
    assert_eq!(1, get_hand_rank_2("11234"));
    assert_eq!(0, get_hand_rank_2("12345"));

    assert_eq!(5, get_hand_rank_2("223JJ")); // 4 of a kind
    assert_eq!(4, get_hand_rank_2("2233J")); // full house
    assert_eq!(3, get_hand_rank_2("234JJ")); // 3 of a kind
    assert_eq!(1, get_hand_rank_2("2345J")); // pair
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(6440, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(247961593, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(5905, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(248750699, output);
}

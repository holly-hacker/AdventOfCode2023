#![allow(clippy::naive_bytecount)]

use std::collections::BTreeMap;

use crate::utils::fast_parse_int_from_bytes;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 7;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let plays = input
            .lines()
            .map(|l| {
                let (hand, score) = l.split_once(' ').unwrap();
                let score = fast_parse_int_from_bytes(score.as_bytes());

                debug_assert_eq!(5, hand.len());
                let hand: [u8; 5] = hand.as_bytes().try_into().unwrap();
                let hand = hand.map(|b| match b {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 11,
                    b'T' => 10,
                    b => b & 0x0F,
                });

                (hand, score)
            })
            .map(|(hand, score)| {
                let hand_type = get_hand_rank_1(hand);
                ((hand_type, hand), score)
            })
            .collect::<BTreeMap<(_, _), _>>();

        plays
            .into_iter()
            .enumerate()
            .map(|(i, (_, val))| (i + 1) * val)
            .sum()
    }
}

fn get_hand_rank_1(hand: [u8; 5]) -> usize {
    let mut char_count_lut = [0u8; 15];
    for char in &hand {
        char_count_lut[*char as usize] += 1;
    }

    if char_count_lut.iter().any(|v| *v == 5) {
        return 6; // 5 of a kind
    }
    if char_count_lut.iter().any(|v| *v == 4) {
        return 5; // 4 of a kind
    }
    if char_count_lut.iter().any(|v| *v == 3) && char_count_lut.iter().any(|v| *v == 2) {
        return 4; // full house
    }
    if char_count_lut.iter().any(|v| *v == 3) {
        return 3; // 3 of a kind
    }
    if char_count_lut.iter().filter(|v| **v == 2).count() == 2 {
        return 2; // 2 pair
    }
    if char_count_lut.iter().any(|v| *v == 2) {
        return 1; // pair
    }

    0
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let plays = input
            .lines()
            .map(|l| {
                let (hand, score) = l.split_once(' ').unwrap();
                let score = fast_parse_int_from_bytes(score.as_bytes());

                debug_assert_eq!(5, hand.len());
                let hand: [u8; 5] = hand.as_bytes().try_into().unwrap();
                let hand = hand.map(|b| match b {
                    b'A' => 13,
                    b'K' => 12,
                    b'Q' => 11,
                    b'T' => 10,
                    b'J' => 1,
                    b => b & 0x0F,
                });

                (hand, score)
            })
            .map(|(hand, score)| {
                let hand_type = get_hand_rank_2(hand);
                ((hand_type, hand), score)
            })
            .collect::<BTreeMap<(_, _), _>>();

        plays
            .into_iter()
            .enumerate()
            .map(|(i, (_, val))| (i + 1) * val)
            .sum()
    }
}

fn get_hand_rank_2(hand: [u8; 5]) -> usize {
    let mut card_lut = [0u8; 12];
    for char in &hand {
        if *char == 1 {
            // joker
            card_lut[0] += 1;
            card_lut[1] += 1;
            card_lut[2] += 1;
            card_lut[3] += 1;
            card_lut[4] += 1;
            card_lut[5] += 1;
            card_lut[6] += 1;
            card_lut[7] += 1;
            card_lut[8] += 1;
            card_lut[9] += 1;
            card_lut[10] += 1;
            card_lut[11] += 1;
        } else {
            card_lut[*char as usize - 2] += 1;
        }
    }

    let card_bitmap = hand.iter().fold(0, |acc, b| acc | 1usize << b);
    let card_types_no_joker = (card_bitmap & !0b10).count_ones() as usize;
    let joker_count = hand.iter().filter(|c| **c == 1).count();

    if card_lut.iter().any(|v| *v >= 5) {
        return 6; // 5 of a kind
    }
    if card_lut.iter().any(|v| *v >= 4) {
        return 5; // 4 of a kind
    }
    if card_lut.iter().filter(|v| **v >= 2).count() >= 2 && card_types_no_joker == 2 {
        return 4; // full house
    }
    if card_lut.iter().any(|v| *v >= 3) {
        return 3; // 3 of a kind
    }
    if joker_count >= 2
        || (joker_count == 1 && card_types_no_joker < 4)
        || (joker_count == 0 && card_lut.iter().filter(|v| **v >= 2).count() >= 2)
    {
        return 2; // 2 pair
    }
    if card_lut.iter().any(|v| *v >= 2) {
        return 1; // pair
    }

    0
}

#[test]
fn test_get_rank_hand_2() {
    let map = |hand: [u8; 5]| {
        hand.map(|b| match b {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'T' => 10,
            b'J' => 1,
            b => b & 0x0F,
        })
    };

    assert_eq!(6, get_hand_rank_2(map(*b"AAAAA")));
    assert_eq!(5, get_hand_rank_2(map(*b"AAAA2")));
    assert_eq!(4, get_hand_rank_2(map(*b"22333")));
    assert_eq!(3, get_hand_rank_2(map(*b"22234")));
    assert_eq!(2, get_hand_rank_2(map(*b"AA223")));
    assert_eq!(1, get_hand_rank_2(map(*b"AA234")));
    assert_eq!(0, get_hand_rank_2(map(*b"A2345")));

    assert_eq!(5, get_hand_rank_2(map(*b"223JJ"))); // 4 of a kind
    assert_eq!(4, get_hand_rank_2(map(*b"2233J"))); // full house
    assert_eq!(3, get_hand_rank_2(map(*b"234JJ"))); // 3 of a kind
    assert_eq!(1, get_hand_rank_2(map(*b"2345J"))); // pair
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

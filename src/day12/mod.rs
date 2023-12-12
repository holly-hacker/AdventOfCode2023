use std::collections::HashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 12;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        input
            .lines()
            .map(|l| {
                let (line, arrangement) = l.split_once(' ').unwrap();
                let line = line.as_bytes();

                let mut arrangement = arrangement
                    .split(',')
                    .map(|a| a.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                let mut line = line.to_vec();
                let mut map = HashMap::new();
                match_count_recursive(&mut line, &mut arrangement, &mut map)
            })
            .sum::<usize>()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        input
            .lines()
            .map(|l| {
                let (line, arrangement) = l.split_once(' ').unwrap();
                let mut once = line.as_bytes().to_vec();
                once.push(b'?');
                let mut line = once.repeat(5);
                line.pop();

                let mut arrangement = arrangement
                    .split(',')
                    .map(|a| a.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
                    .repeat(5);

                let mut map = HashMap::new();
                match_count_recursive(&mut line, &mut arrangement, &mut map)
            })
            .sum::<usize>()
    }
}

fn match_count_recursive(
    line: &mut [u8],
    arrangement: &mut [usize],
    map: &mut HashMap<(Vec<u8>, Vec<usize>), usize>,
) -> usize {
    // don't keep matching the first bit over and over, cut off the first matches if possible
    let Some((line, arrangement)) = match_no_wildcards_partial(line, arrangement) else {
        return 0;
    };

    // trim starting dots
    let pos = line.iter().position(|b| *b != b'.').unwrap_or_default();
    let line = &mut line[pos..];

    // trim early if there's no way we can match this
    if line.len() < arrangement.len() + arrangement.iter().sum::<usize>().saturating_sub(1) {
        return 0;
    }

    if let Some(&count) = map.get(&(line.to_vec(), arrangement.to_vec())) {
        return count;
    }

    if let Some(pos) = line.iter().position(|b| *b == b'?') {
        if pos != 0 {
            panic!(
                "pos != 0, arr: {:?}, line: {:?}",
                arrangement,
                line.iter().map(|b| *b as char).collect::<String>()
            );
        }

        // only try `#` if we can fill out a group
        let possible_group_size = line
            .iter()
            .take_while(|c| **c == b'#' || **c == b'?')
            .count();
        let can_take_immediately = !arrangement.is_empty()
            && possible_group_size >= arrangement[0]
            && line.len() >= arrangement[0] // TODO: useless check, this is done earlier
            // && line.get(arrangement[0] + 1).cloned() != Some(b'#')
            ;
        let with_true = if can_take_immediately {
            // let to_skip = (arrangement[0] + 1).min(line.len());
            // match_count_recursive(&mut line[to_skip..], &mut arrangement[1..])

            line[pos] = b'#';
            match_count_recursive(line, arrangement, map)
        } else {
            0
        };

        // line[pos] = b'.';
        let with_false = match_count_recursive(&mut line[1..], arrangement, map);

        line[pos] = b'?';

        let sum = with_true + with_false;

        map.insert((line.to_vec(), arrangement.to_vec()), sum);

        sum
    } else {
        match_no_wildcards(line, arrangement) as usize
    }
}

fn match_no_wildcards(line: &mut [u8], arrangement: &mut [usize]) -> bool {
    if let Some((line_rest, arr_rest)) = match_no_wildcards_partial(line, arrangement) {
        line_rest.is_empty() && arr_rest.is_empty()
    } else {
        false
    }
}

fn match_no_wildcards_partial<'a, 'b>(
    line: &'a mut [u8],
    arrangement: &'b mut [usize],
) -> Option<(&'a mut [u8], &'b mut [usize])> {
    let mut index = 0;
    for (group_index, group_len) in arrangement.iter().enumerate() {
        if index >= line.len() {
            // end of line!
            // group {group_index} not matched, past end of line
            return None;
        }

        // skip empty bits
        while line[index] == b'.' {
            index += 1;

            if index >= line.len() {
                // group {group_index} not matched, didn't find start
                return None;
            }
        }

        if index != line.len() && line[index] == b'?' {
            // group {group_index} start not matched, found wildcard
            return Some((&mut line[index..], &mut arrangement[group_index..]));
        }

        let group_start_index = index;

        // skip non-empty bits
        // only match starting on `#`, but accept non-first `?`
        while line[index] == b'#'
            || (index != group_start_index
                && index - group_start_index != *group_len
                && line[index] == b'?')
        {
            index += 1;

            if index == line.len() {
                if index - group_start_index != *group_len {
                    // group {group_index} not matched, past end of line
                    return None;
                } else {
                    break;
                }
            }
        }

        // if we match a wildcard after matching a group, assume it is a `.`
        if index != line.len() && line[index] == b'?' {
            // group {group_index} not matched, found wildcard
            index += 1;
            continue;
        }

        let matched_len = index - group_start_index;
        if matched_len != *group_len {
            // group {group_index} not matched, wrong length {matched_len}
            return None;
        }
    }

    if index == line.len() {
        // matched line, no trailing `.`
        return Some((&mut line[index..], &mut []));
    }

    // skip empty bits
    // TODO: ok?
    while line[index] == b'.' {
        index += 1;

        if index >= line.len() {
            break;
        }
    }

    if index < line.len() {
        // line not matched, not at end
        if line[index] == b'?' {
            return Some((&mut line[index..], &mut []));
        }
        return None;
    }

    // matched line
    Some((&mut line[index..], &mut []))
}

#[test]
fn test_could_match_no_wildcard() {
    assert!(match_no_wildcards(&mut b"#".to_vec(), &mut [1]));
    assert!(match_no_wildcards(&mut b"##".to_vec(), &mut [2]));
    assert!(match_no_wildcards(&mut b"#.".to_vec(), &mut [1]));
    assert!(match_no_wildcards(&mut b".#".to_vec(), &mut [1]));
    assert!(match_no_wildcards(&mut b".#.".to_vec(), &mut [1]));
    assert!(match_no_wildcards(&mut b".".to_vec(), &mut []));
    assert!(match_no_wildcards(&mut b"..".to_vec(), &mut []));

    assert!(match_no_wildcards(&mut b".#.#.".to_vec(), &mut [1, 1]));
    assert!(match_no_wildcards(&mut b".###.".to_vec(), &mut [3]));

    assert!(!match_no_wildcards(&mut b".".to_vec(), &mut [1]));
    assert!(!match_no_wildcards(&mut b"#".to_vec(), &mut []));

    assert!(!match_no_wildcards(&mut b"##".to_vec(), &mut [3]));
    assert!(!match_no_wildcards(&mut b"##".to_vec(), &mut [1, 1]));
}

#[test]
fn test_could_match() {
    let mut map = HashMap::new();
    assert_eq!(
        match_count_recursive(&mut b"#?".to_vec(), &mut [2], &mut map),
        1
    );
    assert_eq!(
        match_count_recursive(&mut b"?#".to_vec(), &mut [2], &mut map),
        1
    );
    assert_eq!(
        match_count_recursive(&mut b"#?".to_vec(), &mut [1], &mut map),
        1
    );
    assert_eq!(
        match_count_recursive(&mut b"?#".to_vec(), &mut [1], &mut map),
        1
    );

    assert_eq!(
        match_count_recursive(&mut b"#?#".to_vec(), &mut [1, 1], &mut map),
        1
    );
    assert_eq!(
        match_count_recursive(&mut b"#?#".to_vec(), &mut [3], &mut map),
        1
    );

    assert_eq!(
        match_count_recursive(&mut b"?###".to_vec(), &mut [3], &mut map),
        1
    );

    assert_eq!(
        match_count_recursive(&mut b"?#..".to_vec(), &mut [3], &mut map),
        0
    );
    assert_eq!(
        match_count_recursive(&mut b"..?#".to_vec(), &mut [3], &mut map),
        0
    );

    assert_eq!(
        match_count_recursive(&mut b"???".to_vec(), &mut [1], &mut map),
        3
    );
    assert_eq!(
        match_count_recursive(&mut b"?#?".to_vec(), &mut [2], &mut map),
        2
    );

    // samples from part 2
    assert_eq!(
        match_count_recursive(
            &mut b".#?.#?.#?.#?.#".to_vec(),
            &mut [1, 1, 1, 1, 1],
            &mut map
        ),
        1
    );

    assert_eq!(
        match_count_recursive(
            &mut b"???.###????.###????.###????.###????.###".to_vec(),
            &mut [1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3],
            &mut map
        ),
        1
    );
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(21, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(6827, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(525152, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(1537505634471, output);
}

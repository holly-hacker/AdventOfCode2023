use std::ops::Range;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 5;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut lines = input.lines();

        let line1 = lines.next().unwrap();
        let mut nums = line1
            .split_once(' ')
            .unwrap()
            .1
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        _ = lines.next().unwrap();

        // 7 categories
        for _ in 0..7 {
            _ = lines.next().unwrap();

            let mut mapped = vec![];
            loop {
                let next_line = lines.next();
                let Some(next_line) = next_line else {
                    break;
                };
                if next_line.is_empty() {
                    break;
                }

                // line has 3 nums
                let (dst_start, rest) = next_line.split_once(' ').unwrap();
                let (src_start, len) = rest.split_once(' ').unwrap();

                let dst_start = dst_start.parse::<usize>().unwrap();
                let src_start = src_start.parse::<usize>().unwrap();
                let len = len.parse::<usize>().unwrap();

                for i in (0..nums.len()).rev() {
                    if nums[i] > src_start && nums[i] < src_start + len {
                        mapped.push(nums.remove(i) - src_start + dst_start);
                    }
                }
            }
            // move rest over
            nums.extend(mapped);
        }

        *nums.iter().min().unwrap()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut lines = input.lines();

        let line1 = lines.next().unwrap();
        let nums = line1
            .split_once(' ')
            .unwrap()
            .1
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mut nums = nums
            .chunks(2)
            .map(|a| (a[0]..(a[0] + a[1])))
            .collect::<Vec<Range<usize>>>();
        _ = lines.next().unwrap();

        // 7 categories
        for _ in 0..7 {
            _ = lines.next().unwrap();

            let mut mapped = vec![];
            loop {
                let next_line = lines.next();
                let Some(next_line) = next_line else {
                    break;
                };
                if next_line.is_empty() {
                    break;
                }

                // line has 3 nums
                let (dst_start, rest) = next_line.split_once(' ').unwrap();
                let (src_start, len) = rest.split_once(' ').unwrap();

                let dst_start = dst_start.parse::<usize>().unwrap();
                let src_start = src_start.parse::<usize>().unwrap();
                let len = len.parse::<usize>().unwrap();

                let src = src_start..(src_start + len);
                // let dst = dst_start..(dst_start + len);

                for i in (0..nums.len()).rev() {
                    // check ifrange overlaps
                    let input = nums[i].clone();

                    let overlaps = input.contains(&src.start)
                        || input.contains(&(src.end - 1))
                        || src.contains(&input.start)
                        || src.contains(&(input.end - 1));

                    // split the input range
                    if overlaps {
                        nums.remove(i);

                        // range before src, if any
                        if input.start < src.start {
                            let before = input.start..src.start.min(input.end);
                            nums.push(before);
                        }

                        // range after src, if any
                        if input.end > src.end {
                            let after = src.end.max(input.start)..(input.end);
                            nums.push(after);
                        }

                        // overlapping range
                        let overlap = (input.start.max(src.start))..(input.end.min(src.end));
                        // map the overlap to the dst
                        let overlap = (overlap.start - src_start + dst_start)
                            ..(overlap.end - src_start + dst_start);
                        mapped.push(overlap);
                    }
                }
            }
            // move rest over
            nums.extend(mapped);
            // nums.sort_by_key(|a| (a.start, a.end));
            // nums.dedup();
        }

        nums.iter().map(|r| r.start).min().unwrap()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(35, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(309796150, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(46, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(50716416, output);
}

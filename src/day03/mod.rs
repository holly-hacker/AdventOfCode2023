use crate::utils::*;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 3;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.split(|b| *b == b'\n').next().unwrap().len();
        let stride = width + 1;
        let height = (input.len() + 1) / stride; // +1 because no trailing newline
        debug_assert_eq!(height, input.split(|b| *b == b'\n').count());
        debug_assert_eq!(height * stride, input.len() + 1);

        let mut sum = 0;
        for (y, line) in input.split(|b| *b == b'\n').enumerate() {
            let mut x = 0;
            debug_assert_eq!(line.len(), width);
            while x <= width {
                // find next digit
                let mut start_x = x;
                while start_x < width && !line[start_x].is_ascii_digit() {
                    start_x += 1;
                }

                let mut end_x = start_x;
                while end_x < width && line[end_x].is_ascii_digit() {
                    end_x += 1;
                }

                let byte_run = &line[start_x..end_x];
                if byte_run.is_empty() {
                    x = end_x + 1;
                    continue;
                }

                let parsed_part_num = fast_parse_int_from_bytes(byte_run);

                // look in a box around the number
                let mut found_neighbour = false;
                let min_x = start_x.saturating_sub(1);
                let max_x = end_x.min(width - 1);
                if min_x > 0 {
                    let b = input[y * stride + min_x];
                    found_neighbour |= b != b'.' && !b.is_ascii_digit();
                }
                if max_x < width {
                    let b = input[y * stride + max_x];
                    found_neighbour |= b != b'.' && !b.is_ascii_digit();
                }
                if !found_neighbour && y > 0 {
                    let slice = &input[(y - 1) * stride + min_x..=(y - 1) * stride + max_x];
                    found_neighbour |= slice.iter().any(|b| *b != b'.' && !b.is_ascii_digit());
                }
                if !found_neighbour && y + 1 < height {
                    let slice = &input[(y + 1) * stride + min_x..=(y + 1) * stride + max_x];
                    found_neighbour |= slice.iter().any(|b| *b != b'.' && !b.is_ascii_digit());
                }

                if found_neighbour {
                    sum += parsed_part_num;
                }
                x = end_x + 1;
            }
        }
        sum
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let input = input.as_bytes();
        let width = input.split(|b| *b == b'\n').next().unwrap().len();
        let stride = width + 1;
        let height = (input.len() + 1) / stride; // +1 because no trailing newline
        debug_assert_eq!(height, input.split(|b| *b == b'\n').count());
        debug_assert_eq!(height * stride, input.len() + 1);

        let mut sum = 0;
        for (y, line) in input.split(|b| *b == b'\n').enumerate() {
            let mut x = 0;
            debug_assert_eq!(line.len(), width);
            while x <= width {
                // find next digit
                while x < width && line[x] != b'*' {
                    x += 1;
                }

                if x == width {
                    break;
                }

                // find the 2 neighbouring numbers, if any
                let neighbours = [
                    (y.saturating_sub(1), x.saturating_sub(1)),
                    (y.saturating_sub(1), x),
                    (y.saturating_sub(1), (x + 1) % stride),
                    (y, x.saturating_sub(1)),
                    (y, (x + 1) % stride),
                    ((y + 1) % height, x.saturating_sub(1)),
                    ((y + 1) % height, x),
                    ((y + 1) % height, (x + 1) % stride),
                ];
                let mut item1 = None;
                let mut item2 = None;
                let mut more_than_2 = false;
                for (y, x) in neighbours {
                    if !input[y * stride + x].is_ascii_digit() {
                        continue;
                    }

                    // widen left and right to get the full number
                    let mut start_x = x;
                    while start_x != 0 && input[y * stride + start_x - 1].is_ascii_digit() {
                        start_x = start_x.saturating_sub(1);
                    }
                    let mut start_y = x;
                    while start_y != width && input[y * stride + start_y + 1].is_ascii_digit() {
                        start_y = start_y.saturating_add(1);
                    }

                    let item = Some(((start_x, start_y), y));
                    if item == item1 || item == item2 {
                        continue;
                    }
                    if item1.is_none() {
                        item1 = item;
                    } else if item2.is_none() {
                        item2 = item;
                    } else {
                        more_than_2 = true;
                        break;
                    }
                }

                if item2.is_none() || more_than_2 {
                    // we want 2 numbers
                    x += 1;
                    continue;
                }

                // get first and second item from set
                let num1 = item1.unwrap();
                let num1 =
                    input[num1.1 * stride + num1.0 .0..=num1.1 * stride + num1.0 .1].to_vec();
                let num1 = fast_parse_int_from_bytes(&num1);

                let num2 = item2.unwrap();
                let num2 =
                    input[num2.1 * stride + num2.0 .0..=num2.1 * stride + num2.0 .1].to_vec();
                let num2 = fast_parse_int_from_bytes(&num2);
                sum += num1 * num2;

                x += 1;
            }
        }
        sum
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(4361, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(544664, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(467835, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(84495585, output);
}

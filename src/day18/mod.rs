use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use ahash::AHashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 18;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut map = HashSet::new();
        let mut pos = (0, 0);
        map.insert(pos);

        input.lines().for_each(|line| {
            let (dir, line) = line.split_once(' ').unwrap();
            let (len, _code) = line.split_once(' ').unwrap();

            let direction = match dir {
                "U" => (0, -1),
                "D" => (0, 1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => panic!("Unknown direction {}", dir),
            };

            let len = len.parse::<usize>().unwrap();

            for _ in 0..len {
                pos.0 += direction.0;
                pos.1 += direction.1;
                map.insert(pos);
            }
        });

        // flood-fill all
        let min_x = *map.iter().map(|(x, _)| x).min().unwrap();
        let min_y = *map.iter().map(|(_, y)| y).min().unwrap();
        let max_x = *map.iter().map(|(x, _)| x).max().unwrap();
        let max_y = *map.iter().map(|(_, y)| y).max().unwrap();

        let mut visited_outside = HashSet::<(i32, i32)>::new();

        for start_y in min_y..=max_y {
            for start_x in min_x..=max_x {
                // ignore items already visited
                if map.contains(&(start_x, start_y)) {
                    continue;
                }
                if visited_outside.contains(&(start_x, start_y)) {
                    continue;
                }

                // flood-fill until escaped area or exhausted all possibilities
                let mut visited_now = HashSet::new();
                let mut queue = VecDeque::new();
                queue.push_back((start_x, start_y));

                let mut escaped = false;
                while let Some((x, y)) = queue.pop_front() {
                    // ignore items already visited
                    if map.contains(&(x, y)) {
                        continue;
                    }
                    if visited_now.contains(&(x, y)) {
                        continue;
                    }
                    if visited_outside.contains(&(x, y)) {
                        escaped = true;
                        break;
                    }

                    if x < min_x || x > max_x || y < min_y || y > max_y {
                        // escaped area
                        escaped = true;
                        break;
                    }

                    // mark as visited
                    visited_now.insert((x, y));

                    // add neighbors to queue
                    queue.push_back((x - 1, y));
                    queue.push_back((x + 1, y));
                    queue.push_back((x, y - 1));
                    queue.push_back((x, y + 1));
                }

                if escaped {
                    // TODO: needed for perf but wrong?
                    visited_outside.extend(visited_now);
                } else {
                    map.extend(visited_now);
                }
            }
        }

        map.len()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut coordinates = vec![];
        let mut x_positions = vec![];
        let mut y_positions = vec![];
        let mut pos = (0, 0);

        coordinates.push(pos);
        x_positions.push(pos.0);
        y_positions.push(pos.1);

        input.lines().for_each(|line| {
            let (_dir, line) = line.split_once(' ').unwrap();
            let (_len, code) = line.split_once(' ').unwrap();

            let code = code
                .trim_start_matches('(')
                .trim_start_matches('#')
                .trim_end_matches(')');

            let len = i32::from_str_radix(&code.chars().take(5).collect::<String>(), 16).unwrap();

            let dir = code.chars().last().unwrap();
            let direction = match code.chars().last().unwrap() {
                '0' => (1, 0),
                '1' => (0, 1),
                '2' => (-1, 0),
                '3' => (0, -1),
                _ => panic!("Unknown direction {}", dir),
            };

            let next_pos = (pos.0 + direction.0 * len, pos.1 + direction.1 * len);
            coordinates.push(next_pos);
            x_positions.push(next_pos.0);
            y_positions.push(next_pos.1);
            pos = next_pos;
        });
        x_positions.sort_unstable();
        x_positions.dedup();
        y_positions.sort_unstable();
        y_positions.dedup();

        let mut visited_inside_edges = HashSet::new();

        // add all lines to the new map
        for pair in coordinates.windows(2) {
            let (start, end) = (pair[0], pair[1]);

            let min_x = start.0.min(end.0);
            let max_x = start.0.max(end.0);
            let min_y = start.1.min(end.1);
            let max_y = start.1.max(end.1);

            if min_x == max_x {
                // vertical line
                visited_inside_edges.insert(((min_x..=max_x), (min_y..=min_y)));
                visited_inside_edges.insert(((min_x..=max_x), (min_y + 1..=max_y - 1)));
                visited_inside_edges.insert(((min_x..=max_x), (max_y..=max_y)));
            } else {
                debug_assert_eq!(min_y, max_y);
                // horizontal line
                visited_inside_edges.insert(((min_x..=min_x), (min_y..=max_y)));
                visited_inside_edges.insert(((min_x + 1..=max_x - 1), (min_y..=max_y)));
                visited_inside_edges.insert(((max_x..=max_x), (min_y..=max_y)));
            }
        }

        // flood-fill all
        let mut visited_inside = HashSet::new();
        let mut visited_outside = HashSet::<(RangeInclusive<i32>, RangeInclusive<i32>)>::new();

        for y_pos_idx in 0..(y_positions.len() - 1) {
            let y_start = y_positions[y_pos_idx] + 1;
            let y_end = y_positions[y_pos_idx + 1] - 1;
            let y_range = y_start..=y_end;
            debug_assert!(y_start <= y_end);

            if y_start >= y_end {
                todo!();
                // continue;
            }

            for x_pos_idx in 0..(x_positions.len() - 1) {
                let x_start = x_positions[x_pos_idx] + 1;
                let x_end = x_positions[x_pos_idx + 1] - 1;
                let x_range = x_start..=x_end;
                debug_assert!(x_start <= x_end);

                if x_start >= x_end {
                    todo!();
                    // continue;
                }

                // ignore items already visited
                if visited_inside_edges.contains(&(x_range.clone(), y_range.clone())) {
                    // println!("already visited inside {x_range:?}, {y_range:?}");
                    continue;
                }
                if visited_outside.contains(&(x_range.clone(), y_range.clone())) {
                    // println!("already visited outside {x_range:?}, {y_range:?}");
                    continue;
                }

                // flood-fill until escaped area or exhausted all possibilities
                let mut visited_now = HashSet::new();
                let mut visited_edges = HashSet::new();
                let mut queue = VecDeque::new();
                queue.push_back((x_pos_idx as isize, y_pos_idx as isize));

                let mut escaped = false;
                while let Some((x_pos_idx, y_pos_idx)) = queue.pop_front() {
                    if x_pos_idx < 0
                        || x_pos_idx >= (x_positions.len() as isize - 1)
                        || y_pos_idx < 0
                        || y_pos_idx >= (y_positions.len() as isize - 1)
                    {
                        // escaped area
                        escaped = true;
                        break;
                    }

                    // copy/paste, construct the ranges again
                    let y_start = y_positions[y_pos_idx as usize] + 1;
                    let y_end = y_positions[y_pos_idx as usize + 1] - 1;
                    let y_range = y_start..=y_end;
                    if y_start >= y_end {
                        continue;
                    }
                    let x_start = x_positions[x_pos_idx as usize] + 1;
                    let x_end = x_positions[x_pos_idx as usize + 1] - 1;
                    let x_range = x_start..=x_end;
                    if x_start >= x_end {
                        continue;
                    }
                    debug_assert!(y_start <= y_end);
                    debug_assert!(x_start <= x_end);

                    // ignore items already visited
                    if visited_inside_edges.contains(&(x_range.clone(), y_range.clone())) {
                        continue;
                    }
                    if visited_now.contains(&(x_range.clone(), y_range.clone())) {
                        continue;
                    }
                    if visited_outside.contains(&(x_range.clone(), y_range.clone())) {
                        escaped = true;
                        break;
                    }

                    // mark as visited
                    visited_now.insert((x_range.clone(), y_range.clone()));

                    let can_go_left = !visited_inside_edges
                        .iter()
                        .any(|r| r.0.contains(&(x_start - 1)) && r.1.contains(&y_start));
                    let can_go_right = !visited_inside_edges
                        .iter()
                        .any(|r| r.0.contains(&(x_end + 1)) && r.1.contains(&y_start));
                    let can_go_up = !visited_inside_edges
                        .iter()
                        .any(|r| r.1.contains(&(y_start - 1)) && r.0.contains(&x_start));
                    let can_go_down = !visited_inside_edges
                        .iter()
                        .any(|r| r.1.contains(&(y_end + 1)) && r.0.contains(&x_start));

                    // add neighbors to queue, iff there is no wall blocking them
                    if can_go_left {
                        visited_edges.insert(((x_start - 1..=x_start - 1), y_range.clone()));
                        visited_edges
                            .insert(((x_start - 1..=x_start - 1), (y_start - 1..=y_start - 1)));
                        visited_edges
                            .insert(((x_start - 1..=x_start - 1), (y_end + 1..=y_end + 1)));
                        queue.push_back((x_pos_idx - 1, y_pos_idx));
                    }
                    if can_go_right {
                        visited_edges.insert(((x_end + 1..=x_end + 1), y_range.clone()));
                        visited_edges
                            .insert(((x_end + 1..=x_end + 1), (y_start - 1..=y_start - 1)));
                        visited_edges.insert(((x_end + 1..=x_end + 1), (y_end + 1..=y_end + 1)));
                        queue.push_back((x_pos_idx + 1, y_pos_idx));
                    }
                    if can_go_up {
                        visited_edges.insert((x_range.clone(), y_start - 1..=y_start - 1));
                        visited_edges
                            .insert(((x_start - 1..=x_start - 1), (y_start - 1..=y_start - 1)));
                        visited_edges
                            .insert(((x_end + 1..=x_end + 1), (y_start - 1..=y_start - 1)));
                        queue.push_back((x_pos_idx, y_pos_idx - 1));
                    }
                    if can_go_down {
                        visited_edges.insert((x_range.clone(), y_end + 1..=y_end + 1));
                        visited_edges
                            .insert(((x_start - 1..=x_start - 1), (y_end + 1..=y_end + 1)));
                        visited_edges.insert(((x_end + 1..=x_end + 1), (y_end + 1..=y_end + 1)));
                        queue.push_back((x_pos_idx, y_pos_idx + 1));
                    }
                }

                if escaped {
                    // println!("escaped {x_range:?}, {y_range:?}");
                    visited_outside.extend(visited_now);
                } else {
                    // println!("not escaped {x_range:?}, {y_range:?}");
                    visited_inside.extend(visited_now);
                    visited_inside_edges.extend(visited_edges);
                }
            }
        }

        let new_total = visited_inside
            .iter()
            .flat_map(|region| {
                let left = region.0.start() - 1..=region.0.start() - 1;
                let right = region.0.end() + 1..=region.0.end() + 1;
                let top = region.1.start() - 1..=region.1.start() - 1;
                let bottom = region.1.end() + 1..=region.1.end() + 1;

                [
                    (region.0.clone(), region.1.clone()), // inner
                    (left.clone(), region.1.clone()),     // left
                    (right.clone(), region.1.clone()),    // right
                    (region.0.clone(), top.clone()),      // top
                    (region.0.clone(), bottom.clone()),   // bottom
                    (left.clone(), top.clone()),          // top-left
                    (right.clone(), top.clone()),         // top-right
                    (left.clone(), bottom.clone()),       // bottom-left
                    (right.clone(), bottom.clone()),      // bottom-right
                ]
            })
            .collect::<HashSet<_>>();

        let ret = new_total
            .into_iter()
            .map(|(range_x, range_y)| range_x.count() * range_y.count())
            .sum::<usize>();

        // dbg!(ret);

        // std::thread::sleep(std::time::Duration::from_secs(1));

        ret
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(62, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(34329, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(952408144115, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(42617947302920, output);
}

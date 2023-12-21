use std::collections::VecDeque;

use ahash::AHashMap;
use memchr::memchr;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 21;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        const MAX_DISTANCE: usize = 64;

        let input = input.as_bytes();
        let width = memchr(b'\n', input).unwrap();
        let stride = width + 1;

        let start_position = memchr(b'S', input).unwrap();

        let mut queue = VecDeque::new();
        let mut visited = AHashMap::new();
        queue.push_back((0, start_position));

        while let Some((steps, pos)) = queue.pop_front() {
            let entry = visited.entry(pos).or_insert(usize::MAX);
            if *entry != usize::MAX {
                continue;
            }
            *entry = steps;

            if steps == MAX_DISTANCE {
                continue;
            }

            let x = pos % stride;
            let y = pos / stride;

            if x > 0 && input[pos - 1] != b'#' {
                queue.push_back((steps + 1, pos - 1));
            }
            if x < width - 1 && input[pos + 1] != b'#' {
                queue.push_back((steps + 1, pos + 1));
            }
            if y > 0 && input[pos - stride] != b'#' {
                queue.push_back((steps + 1, pos - stride));
            }
            if y < width - 1 && input[pos + stride] != b'#' {
                queue.push_back((steps + 1, pos + stride));
            }
        }

        visited
            .values()
            .filter(|&v| v % 2 == MAX_DISTANCE % 2)
            .count()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_nowall.txt");

    fn calculate_gold(input: &str) -> usize {
        const MAX_DISTANCE: usize = 26501365;

        let input = input.as_bytes();
        let width = memchr(b'\n', input).unwrap();
        let stride = width + 1;

        let start_position = memchr(b'S', input).unwrap();

        let calculate_grid_endpoints =
            |start_position: usize, start_steps: usize, max_distance: usize| {
                if max_distance < start_steps {
                    return 0;
                }

                let mut queue = VecDeque::new();
                let mut visited = AHashMap::new();
                queue.push_back((start_steps, start_position));

                while let Some((steps, pos)) = queue.pop_front() {
                    debug_assert!(matches!(input[pos], b'.' | b'S'));

                    let entry = visited.entry(pos).or_insert(usize::MAX);
                    if *entry != usize::MAX {
                        continue;
                    }
                    *entry = steps;

                    if steps == max_distance {
                        continue;
                    }

                    let x = pos % stride;
                    let y = pos / stride;

                    if x > 0 && input[pos - 1] != b'#' {
                        queue.push_back((steps + 1, pos - 1));
                    }
                    if x < width - 1 && input[pos + 1] != b'#' {
                        queue.push_back((steps + 1, pos + 1));
                    }
                    if y > 0 && input[pos - stride] != b'#' {
                        queue.push_back((steps + 1, pos - stride));
                    }
                    if y < width - 1 && input[pos + stride] != b'#' {
                        queue.push_back((steps + 1, pos + stride));
                    }
                }

                visited
                    .values()
                    .filter(|&v| v % 2 == MAX_DISTANCE % 2)
                    .count()
            };

        let first_grid_endpoints = calculate_grid_endpoints(start_position, 0, MAX_DISTANCE);
        let odd_grids_endpoints = calculate_grid_endpoints(1, 0, usize::MAX);
        let even_grids_endpoints = calculate_grid_endpoints(0, 0, usize::MAX);

        // calculate the manhattan distance of to the last grid that is fully covered
        // this is inclusive of the starting grid
        // this crashes if there is the result should be 0
        let distance_into_corner_grid = (MAX_DISTANCE + width - (width / 2 + 1)) % width;

        // the amount of grids we covered in 1 direction. includes start grid
        let total_grid_distance_without_start = (MAX_DISTANCE + width - (width / 2 + 1)) / width;

        // how many of the outer grids are incomplete
        // this is usually 1, but can be 2 if we travel less than half the tiles into the last grid
        let tip_is_under_half_len = distance_into_corner_grid < (width / 2);

        let complete_grids_distance_without_start = total_grid_distance_without_start
            .saturating_sub(if tip_is_under_half_len { 2 } else { 1 });
        let complete_grids_distance_with_start = complete_grids_distance_without_start + 1; // TODO: start can be incomplete!

        // we know how many grids to travel, but we don't yet know how many targets we will find in
        // the outer grids

        // 1 -> 1
        // 2 -> 5 (1 + (1+3))
        // 3 -> 13 (1 + (2*2) + (4*2)) -> 1 + 6*2
        // 4 -> 25 (1 + (2*2) + (4*2) + (6*2)) -> 1 + 12*2
        // 5 -> 41 (1 + (2*2) + (4*2) + (6*2) + (8*2)) -> 1 + 20*2
        // 6 -> 61 (1 + (2*2) + (4*2) + (6*2) + (8*2) + (10*2)) -> 1 + 30*2
        // 7 -> 85 (1 + (2*2) + (4*2) + (6*2) + (8*2) + (10*2) + (12*2)) -> 1 + 42*2
        // 8 -> 113 (1 + (2*2) + (4*2) + (6*2) + (8*2) + (10*2) + (12*2) + (14*2)) -> 1 + 56*2

        // seems to be 2n^2 + -2n + 1
        // the amount of tiles that are covered by the manhattan distance, including the center
        let complete_grid_count =
            (2 * complete_grids_distance_with_start * complete_grids_distance_with_start)
                - (complete_grids_distance_with_start * 2)
                + 1;

        let even_grid_count = (complete_grids_distance_with_start + 1) / 2;
        let even_grid_count = even_grid_count * (even_grid_count - 1) * 4;
        let odd_grid_count = complete_grids_distance_with_start / 2;
        let odd_grid_count = odd_grid_count * odd_grid_count * 4;

        debug_assert_eq!(complete_grid_count, 1 + even_grid_count + odd_grid_count);

        let inside_endpoints =
            even_grids_endpoints * even_grid_count + odd_grids_endpoints * odd_grid_count;

        // calculate edges
        // TODO: for small numbers
        let outer_corner_count = 1;
        let inner_corner_count = if tip_is_under_half_len { 1 } else { 0 };
        let outer_edge_count = if !tip_is_under_half_len {
            total_grid_distance_without_start
        } else {
            total_grid_distance_without_start.saturating_sub(1)
        };
        let inner_edge_count = outer_edge_count.saturating_sub(1);

        let distance_to_outer_corner_grid =
            width / 2 + 1 + width * total_grid_distance_without_start.saturating_sub(1);
        let distance_to_inner_corner_grid = distance_to_outer_corner_grid.saturating_sub(width);
        let distance_to_outer_edge_grid = (width + 1) + width * (outer_edge_count - 1);
        let distance_to_inner_edge_grid = distance_to_outer_edge_grid - width;

        debug_assert!((distance_to_outer_corner_grid - (width / 2 + 1)) % width == 0);
        debug_assert!((distance_to_inner_corner_grid - (width / 2 + 1)) % width == 0);
        debug_assert!((distance_to_inner_edge_grid - (width + 1)) % width == 0);
        debug_assert!((distance_to_outer_edge_grid - (width + 1)) % width == 0);

        let x_left = 0;
        let x_mid = width / 2;
        let x_right = width - 1;
        let y_top = 0;
        let y_mid = (width / 2) * stride;
        let y_bottom = (width - 1) * stride;
        let muls_distances_starts = [
            // outer corners in top, left, bottom, right
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                y_top + x_mid,
            ),
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                y_mid + x_left,
            ),
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                y_bottom + x_mid,
            ),
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                y_mid + x_right,
            ),
            // inner corners, if any
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                y_top + x_mid,
            ),
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                y_mid + x_left,
            ),
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                y_bottom + x_mid,
            ),
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                y_mid + x_right,
            ),
            // outer edges in top-left, top-right, bottom-left, bottom-right
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                y_top + x_left,
            ),
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                y_top + x_right,
            ),
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                y_bottom + x_left,
            ),
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                y_bottom + x_right,
            ),
            // inner edges in top-left, top-right, bottom-left, bottom-right
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                y_top + x_left,
            ),
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                y_top + x_right,
            ),
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                y_bottom + x_left,
            ),
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                y_bottom + x_right,
            ),
        ];

        let edge_endpoints: usize = muls_distances_starts
            .into_iter()
            .map(|(multiplier, start_steps, start_position)| {
                let in_grid = calculate_grid_endpoints(start_position, start_steps, MAX_DISTANCE);
                in_grid * multiplier
            })
            .sum();

        first_grid_endpoints + inside_endpoints + edge_endpoints
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    // assert_eq!(16, output);
    assert_eq!(42, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(3795, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(702322399865956, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(630129824772393, output);
}

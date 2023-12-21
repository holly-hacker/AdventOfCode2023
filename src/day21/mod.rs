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

        let (odd_grids_endpoints, even_grids_endpoints) = if start_position % 2 != 0 {
            debug_assert_eq!(
                first_grid_endpoints,
                calculate_grid_endpoints(0, 0, usize::MAX)
            );
            (
                calculate_grid_endpoints(1, 0, usize::MAX),
                first_grid_endpoints,
            )
        } else {
            debug_assert_eq!(
                first_grid_endpoints,
                calculate_grid_endpoints(1, 0, usize::MAX)
            );
            (
                first_grid_endpoints,
                calculate_grid_endpoints(0, 0, usize::MAX),
            )
        };

        // the amount of grids we crossed if we went straight in 1 direction
        let total_grid_count_excluding_start = (MAX_DISTANCE + width - (width / 2 + 1)) / width;

        // how far we get into the final grid if we were to go straight. this final grid is not
        // fully covered by our search.
        let distance_into_corner_grid = (MAX_DISTANCE + width - (width / 2 + 1)) % width;

        // if we went past the halfway point of the corner grid, the grid before this is also not
        // covered
        let corner_covers_two_grids = distance_into_corner_grid < (width / 2);

        // the total amount of grids we crossed that we fully cover. This includes the start grid
        // because it makes later math a bit easier.
        let complete_grid_count_including_start = if corner_covers_two_grids {
            total_grid_count_excluding_start - 1
        } else {
            total_grid_count_excluding_start
        };

        // we know how many grids to travel, but we don't yet know how many targets we will find in
        // the outer grids

        // calculate the total amount of fully covered grids. this is not technically needed, but we
        // use it for an assertion later.
        // Whether a tile is included depends on the manhattan distance from the center. The formula
        // seems to be `2n^2 + -2n + 1`. This includes the center.
        let complete_grid_count =
            (2 * complete_grid_count_including_start * complete_grid_count_including_start)
                - (complete_grid_count_including_start * 2)
                + 1;

        // We actually nned this number to be split up into "even" and "odd" grids. This is because
        // the grid has an odd size so the amount of reachable tiles is different for even and odd
        // grids. The formula is `n*(n-1)*4`, where `n` is the amount of complete grids in 1
        // direction.
        let even_grid_count = (complete_grid_count_including_start + 1) / 2;
        let even_grid_count = even_grid_count * (even_grid_count - 1) * 4;
        let odd_grid_count = complete_grid_count_including_start / 2;
        let odd_grid_count = odd_grid_count * odd_grid_count * 4;

        debug_assert_eq!(complete_grid_count, 1 + even_grid_count + odd_grid_count);

        // The total amount of points we can end up at after the given walk distance.
        let inside_endpoints =
            even_grids_endpoints * even_grid_count + odd_grids_endpoints * odd_grid_count;

        // Calculate the amount of endpoints for edges and corners that are not fully covered.
        let outer_corner_count = 1;
        let inner_corner_count = if corner_covers_two_grids { 1 } else { 0 };
        let outer_edge_count = if corner_covers_two_grids {
            total_grid_count_excluding_start - 1
        } else {
            total_grid_count_excluding_start
        };
        let inner_edge_count = outer_edge_count - 1;

        // We also need the distance to each of these corners and edges. These are the same for each
        // group.
        let distance_to_outer_corner_grid =
            width / 2 + 1 + width * total_grid_count_excluding_start.saturating_sub(1);
        let distance_to_inner_corner_grid = distance_to_outer_corner_grid.saturating_sub(width);
        let distance_to_outer_edge_grid = (width + 1) + width * (outer_edge_count - 1);
        let distance_to_inner_edge_grid = distance_to_outer_edge_grid - width;

        debug_assert!((distance_to_outer_corner_grid - (width / 2 + 1)) % width == 0);
        debug_assert!((distance_to_inner_corner_grid - (width / 2 + 1)) % width == 0);
        debug_assert!((distance_to_inner_edge_grid - (width + 1)) % width == 0);
        debug_assert!((distance_to_outer_edge_grid - (width + 1)) % width == 0);

        // Now iterate over each corner/edge and calculate the amount of endpoints in them.
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
                if multiplier == 0 {
                    return 0;
                }
                multiplier * calculate_grid_endpoints(start_position, start_steps, MAX_DISTANCE)
            })
            .sum();

        // Add everything up.
        first_grid_endpoints + inside_endpoints + edge_endpoints
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
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

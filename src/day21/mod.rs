use std::collections::VecDeque;

use ahash::{AHashMap, AHashSet};
use memchr::memchr;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 21;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
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

            if steps == 64 {
                continue;
            }

            let x = pos % stride;
            let y = pos / stride;

            if x > 0 && input[pos - 1] != b'#' {
                queue.push_back((steps + 1, (pos - 1)));
            }
            if x < width - 1 && input[pos + 1] != b'#' {
                queue.push_back((steps + 1, (pos + 1)));
            }
            if y > 0 && input[pos - stride] != b'#' {
                queue.push_back((steps + 1, pos - stride));
            }
            if y < width - 1 && input[pos + stride] != b'#' {
                queue.push_back((steps + 1, pos + stride));
            }
        }

        visited.values().filter(|&v| v % 2 == 0).count()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_nowall.txt");

    #[allow(clippy::erasing_op)]
    fn calculate_gold(input: &str) -> usize {
        // let max_distance = 1;
        let max_distance = 6 * 1 + 11 * 4 + 0 - 1;
        let max_distance = 26501365;
        dbg!(max_distance);
        // assert!(max_distance % 2 == 0);

        let distance_mod_2 = max_distance % 2;

        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();
        assert!(width % 2 == 1);
        assert_eq!(width, height);
        dbg!(height);

        let start_position = grid
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        let calculate_grid_endpoints =
            |start_position: (usize, usize), start_steps: usize, max_distance: usize| {
                if max_distance < start_steps {
                    return 0;
                }

                let mut targets = AHashSet::new();

                let mut queue = VecDeque::new();
                let mut visited = AHashMap::new();
                queue.push_back((start_steps, start_position));

                while let Some((steps, (x, y))) = queue.pop_front() {
                    debug_assert!(matches!(grid[y][x], 'S' | '.'));

                    let entry = visited.entry((x, y)).or_insert(usize::MAX);

                    // TODO: not really needed?
                    if *entry <= steps {
                        continue;
                    }

                    *entry = steps;

                    // BUG: technically we're not allowed to do this if steps == 0 && max_distance < 2
                    // max_distance will always be >= 2 though
                    if steps % 2 == distance_mod_2 {
                        targets.insert((x, y));
                    }

                    if steps == max_distance {
                        continue;
                    }

                    if x > 0 && grid[y][x - 1] != '#' {
                        queue.push_back((steps + 1, (x - 1, y)));
                    }
                    if x < grid[y].len() - 1 && grid[y][x + 1] != '#' {
                        queue.push_back((steps + 1, (x + 1, y)));
                    }
                    if y > 0 && grid[y - 1][x] != '#' {
                        queue.push_back((steps + 1, (x, y - 1)));
                    }
                    if y < grid.len() - 1 && grid[y + 1][x] != '#' {
                        queue.push_back((steps + 1, (x, y + 1)));
                    }
                }

                if true {
                    for y in 0..height {
                        for x in 0..width {
                            if targets.contains(&(x, y)) {
                                print!("X");
                            } else if visited.contains_key(&(x, y)) {
                                print!("O");
                            } else {
                                print!(".");
                            }
                        }
                        println!();
                    }
                    println!();
                }

                targets.len()
            };

        let first_grid_endpoints = calculate_grid_endpoints(start_position, 0, max_distance);
        let odd_grids_endpoints = calculate_grid_endpoints((0, 1), 0, usize::MAX);
        let even_grids_endpoints = calculate_grid_endpoints((0, 0), 0, usize::MAX);

        dbg!(
            first_grid_endpoints,
            odd_grids_endpoints,
            even_grids_endpoints
        );

        // calculate the manhattan distance of to the last grid that is fully covered
        // this is inclusive of the starting grid
        // this crashes if there is the result should be 0
        // let distance_to_inner_top = height / 2;

        let distance_into_corner_grid = (max_distance + height - (height / 2 + 1)) % height;
        dbg!(distance_into_corner_grid);

        // the amount of grids we covered in 1 direction. includes start grid
        let total_grid_distance_without_start = (max_distance + height - (height / 2 + 1)) / height;
        dbg!(total_grid_distance_without_start);
        let total_grid_distance_with_start = total_grid_distance_without_start + 1;
        dbg!(total_grid_distance_with_start);

        // how many of the outer grids are incomplete
        // this is usually 1, but can be 2 if we travel less than half the tiles into the last grid
        let tip_is_under_half_len = distance_into_corner_grid < (width / 2);

        let complete_grids_distance_without_start = total_grid_distance_without_start
            .saturating_sub(if tip_is_under_half_len { 2 } else { 1 });
        let complete_grids_distance_with_start = complete_grids_distance_without_start + 1; // TODO: start can be incomplete!
        dbg!(complete_grids_distance_without_start);
        dbg!(complete_grids_distance_with_start);

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
        dbg!(even_grid_count);
        let odd_grid_count = complete_grids_distance_with_start / 2;
        let odd_grid_count = odd_grid_count * odd_grid_count * 4;
        dbg!(odd_grid_count);

        assert_eq!(complete_grid_count, 1 + even_grid_count + odd_grid_count);

        let inside_endpoints =
            even_grids_endpoints * even_grid_count + odd_grids_endpoints * odd_grid_count;
        dbg!(inside_endpoints);

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
        dbg!(
            outer_corner_count,
            inner_corner_count,
            outer_edge_count,
            inner_edge_count
        );

        let distance_to_outer_corner_grid =
            height / 2 + 1 + height * total_grid_distance_without_start.saturating_sub(1);
        let distance_to_inner_corner_grid = distance_to_outer_corner_grid.saturating_sub(width);
        let distance_to_outer_edge_grid = (height + 1) + height * (outer_edge_count - 1);
        let distance_to_inner_edge_grid = distance_to_outer_edge_grid - width;
        // let distance_to_inner_edge_grid = (height + 1) + height * inner_edge_count;
        dbg!(
            distance_to_outer_corner_grid,
            distance_to_inner_corner_grid,
            distance_to_outer_edge_grid,
            distance_to_inner_edge_grid,
        );

        assert!((distance_to_outer_corner_grid - (height / 2 + 1)) % width == 0);
        assert!((distance_to_inner_corner_grid - (height / 2 + 1)) % width == 0);
        assert!((distance_to_inner_edge_grid - (height + 1)) % width == 0);
        assert!((distance_to_outer_edge_grid - (height + 1)) % width == 0);

        let muls_distances_starts = [
            // outer corners in top, left, bottom, right
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                (width / 2, 0),
            ),
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                (0, height / 2),
            ),
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                (width / 2, height - 1),
            ),
            (
                outer_corner_count,
                distance_to_outer_corner_grid,
                (width - 1, height / 2),
            ),
            // inner corners, if any
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                (width / 2, 0),
            ),
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                (0, height / 2),
            ),
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                (width / 2, height - 1),
            ),
            (
                inner_corner_count,
                distance_to_inner_corner_grid,
                (width - 1, height / 2),
            ),
            // outer edges in top-left, top-right, bottom-left, bottom-right
            (outer_edge_count, distance_to_outer_edge_grid, (0, 0)),
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                (width - 1, 0),
            ),
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                (0, height - 1),
            ),
            (
                outer_edge_count,
                distance_to_outer_edge_grid,
                (width - 1, height - 1),
            ),
            // inner edges in top-left, top-right, bottom-left, bottom-right
            (inner_edge_count, distance_to_inner_edge_grid, (0, 0)),
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                (width - 1, 0),
            ),
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                (0, height - 1),
            ),
            (
                inner_edge_count,
                distance_to_inner_edge_grid,
                (width - 1, height - 1),
            ),
        ];

        let edge_endpoints: usize = muls_distances_starts
            .into_iter()
            .map(|(multiplier, start_steps, start_position)| {
                let in_grid = calculate_grid_endpoints(start_position, start_steps, max_distance);
                println!("{} * {} = {}", multiplier, in_grid, in_grid * multiplier);
                in_grid * multiplier
            })
            .sum();

        dbg!(first_grid_endpoints) + dbg!(inside_endpoints) + dbg!(edge_endpoints)
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
    assert_eq!(usize::MAX, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert!(output > 10621590);
    assert!(output < 635988199081920);
    assert!(output < 636007300316278);
    assert_ne!(635978946937155, output);
    assert_ne!(630208481451755, output);
    assert_ne!(630208480642555, output);
    assert_ne!(630126630467155, output);
    assert_ne!(630126794127099, output);
    assert_ne!(630124408414992, output);
    assert_ne!(630129824741644, output);
    assert_ne!(630129775960239, output);
    //         630129020575773
    //         630123537296764
    assert_ne!(630123537288724, output);
    assert_eq!(630129824772393, output);
}

use std::collections::{HashSet, VecDeque};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 10;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample_silver.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let grid = input
            .lines()
            .flat_map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        let start_position = grid.iter().position(|&c| *c == b'S').unwrap();

        let mut distances = vec![usize::MAX; width * height];

        let mut queue = VecDeque::new();
        queue.push_back((start_position, 0));
        distances[start_position] = 0;

        while let Some((position, distance)) = queue.pop_front() {
            let x = position % width;
            let y = position / width;

            // left
            if x > 0 {
                let target_position = position - 1;
                if matches!(grid[position], b'S' | b'-' | b'J' | b'7')
                    && matches!(grid[target_position], b'-' | b'L' | b'F')
                    && distances[target_position] > distance + 1
                {
                    distances[target_position] = distance + 1;
                    queue.push_back((target_position, distance + 1));
                }
            }

            // right
            if x < width - 1 {
                let target_position = position + 1;
                if matches!(grid[position], b'S' | b'-' | b'L' | b'F')
                    && matches!(grid[target_position], b'-' | b'J' | b'7')
                    && distances[target_position] > distance + 1
                {
                    distances[target_position] = distance + 1;
                    queue.push_back((target_position, distance + 1));
                }
            }

            // top
            if y > 0 {
                let target_position = position - width;
                if matches!(grid[position], b'S' | b'|' | b'L' | b'J')
                    && matches!(grid[target_position], b'|' | b'F' | b'7')
                    && distances[target_position] > distance + 1
                {
                    distances[target_position] = distance + 1;
                    queue.push_back((target_position, distance + 1));
                }
            }

            // bottom
            if y < height - 1 {
                let target_position = position + width;
                if matches!(grid[position], b'S' | b'|' | b'F' | b'7')
                    && matches!(grid[target_position], b'|' | b'L' | b'J')
                    && distances[target_position] > distance + 1
                {
                    distances[target_position] = distance + 1;
                    queue.push_back((target_position, distance + 1));
                }
            }
        }

        distances
            .iter()
            .filter(|&&d| d != usize::MAX)
            .max()
            .unwrap()
            .to_owned()
    }
}

impl SolutionGold<usize, usize> for Day {
    const INPUT_SAMPLE_GOLD: &'static str = include_str!("input_sample_gold3.txt");

    fn calculate_gold(input: &str) -> usize {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let grid = input
            .lines()
            .flat_map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        let start_position = grid.iter().position(|&c| *c == b'S').unwrap();

        let mut the_loop = vec![false; width * height];

        let mut queue = VecDeque::new();
        queue.push_back(start_position);
        the_loop[start_position] = true;

        while let Some(position) = queue.pop_front() {
            let x = position % width;
            let y = position / width;

            // left
            if x > 0 {
                let target_position = position - 1;
                if matches!(grid[position], b'S' | b'-' | b'J' | b'7')
                    && matches!(grid[target_position], b'-' | b'L' | b'F')
                    && !the_loop[target_position]
                {
                    the_loop[target_position] = true;
                    queue.push_back(target_position);
                }
            }

            // right
            if x < width - 1 {
                let target_position = position + 1;
                if matches!(grid[position], b'S' | b'-' | b'L' | b'F')
                    && matches!(grid[target_position], b'-' | b'J' | b'7')
                    && !the_loop[target_position]
                {
                    the_loop[target_position] = true;
                    queue.push_back(target_position);
                }
            }

            // top
            if y > 0 {
                let target_position = position - width;
                if matches!(grid[position], b'S' | b'|' | b'L' | b'J')
                    && matches!(grid[target_position], b'|' | b'F' | b'7')
                    && !the_loop[target_position]
                {
                    the_loop[target_position] = true;
                    queue.push_back(target_position);
                }
            }

            // bottom
            if y < height - 1 {
                let target_position = position + width;
                if matches!(grid[position], b'S' | b'|' | b'F' | b'7')
                    && matches!(grid[target_position], b'|' | b'L' | b'J')
                    && !the_loop[target_position]
                {
                    the_loop[target_position] = true;
                    queue.push_back(target_position);
                }
            }
        }

        // flood-fill the grid to find which tiles are within the loop
        let mut area_outside_loop = vec![false; width * height];
        let mut queue = VecDeque::new();

        // find any position outside the loop by walking the edges
        for x in 0..width {
            for y in 0..height {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    let position = x + y * width;
                    if !the_loop[position] {
                        queue.push_back(position);
                    }
                }
            }
        }

        // flood-fill the outside of the grid
        while let Some(pos) = queue.pop_front() {
            debug_assert!(!the_loop[pos]);

            if area_outside_loop[pos] {
                continue;
            }

            let x = pos % width;
            let y = pos / width;

            // left
            if x > 0 {
                let target_position = pos - 1;
                if !the_loop[target_position] {
                    // not part of the loop
                    queue.push_back(target_position);
                }
            }

            // right
            if x < width - 1 {
                let target_position = pos + 1;
                if !the_loop[target_position] {
                    queue.push_back(target_position);
                }
            }

            // top
            if y > 0 {
                let target_position = pos - width;
                if !the_loop[target_position] {
                    queue.push_back(target_position);
                }
            }

            // bottom
            if y < height - 1 {
                let target_position = pos + width;
                if !the_loop[target_position] {
                    queue.push_back(target_position);
                }
            }

            area_outside_loop[pos] = true;
        }

        let mut loop_count = 0;
        let mut grid_positions_within_loop = HashSet::<usize>::new();
        let mut grid_positions_outside_loop = HashSet::<usize>::new();
        for position in 0..grid.len() {
            let grid_width = width + 1;
            let grid_height = height + 1;
            let start_x = position % width;
            let start_y = position / width;

            if the_loop[position] || area_outside_loop[position] {
                continue;
            }

            // try to escape the grid using line coordinates rather than cell coordinates
            let mut queue = VecDeque::new();
            queue.push_back(grid_width * start_y + start_x);
            queue.push_back(grid_width * (start_y + 1) + start_x);
            queue.push_back(grid_width * start_y + (start_x + 1));
            queue.push_back(grid_width * (start_y + 1) + (start_x + 1));

            let mut visited = vec![false; grid_width * grid_height];

            let mut escaped = false;
            while let Some(grid_pos) = queue.pop_front() {
                if visited[grid_pos] {
                    // already visited!
                    continue;
                }

                if grid_positions_within_loop.contains(&grid_pos) {
                    // we found a loop
                    escaped = false;
                    break;
                }
                if grid_positions_outside_loop.contains(&grid_pos) {
                    // we didn't find a loop
                    escaped = true;
                    break;
                }

                let grid_pos_x = grid_pos % grid_width;
                let grid_pos_y = grid_pos / grid_width;

                if grid_pos_x == 0 || grid_pos_x == width || grid_pos_y == 0 || grid_pos_y == height
                {
                    escaped = true;
                    break;
                }

                let top_left_pos = (grid_pos_x - 1) + (grid_pos_y - 1) * width;
                let bottom_left_pos = (grid_pos_x - 1) + grid_pos_y * width;
                let top_right_pos = grid_pos_x + (grid_pos_y - 1) * width;
                let bottom_right_pos = grid_pos_x + grid_pos_y * width;

                if area_outside_loop[top_left_pos]
                    || area_outside_loop[bottom_left_pos]
                    || area_outside_loop[top_right_pos]
                    || area_outside_loop[bottom_right_pos]
                {
                    escaped = true;
                    break;
                }

                let top_left = grid[top_left_pos];
                let bottom_left = grid[bottom_left_pos];
                let top_right = grid[top_right_pos];
                let bottom_right = grid[bottom_right_pos];

                // left
                if matches!(top_left, b'-' | b'L' | b'J' | b'.')
                    || matches!(bottom_left, b'-' | b'F' | b'7' | b'.')
                {
                    // we can slip through
                    queue.push_back(grid_pos - 1);
                }

                // right
                if matches!(top_right, b'-' | b'L' | b'J' | b'.')
                    || matches!(bottom_right, b'-' | b'F' | b'7' | b'.')
                {
                    queue.push_back(grid_pos + 1);
                }

                // top
                if matches!(top_left, b'|' | b'7' | b'J' | b'.')
                    || matches!(top_right, b'|' | b'F' | b'L' | b'.')
                {
                    queue.push_back(grid_pos - grid_width);
                }

                // bottom
                if matches!(bottom_left, b'|' | b'7' | b'J' | b'.')
                    || matches!(bottom_right, b'|' | b'F' | b'L' | b'.')
                {
                    queue.push_back(grid_pos + grid_width);
                }

                visited[grid_pos] = true;
            }

            if !escaped {
                // we found a loop
                loop_count += 1;
                for grid_pos in visited
                    .iter()
                    .enumerate()
                    .filter(|(_, &v)| v)
                    .map(|(i, _)| i)
                {
                    grid_positions_within_loop.insert(grid_pos);
                }
            } else {
                // we didn't find a loop
                for grid_pos in queue {
                    grid_positions_outside_loop.insert(grid_pos);
                }
            }
        }

        loop_count
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
    assert_eq!(6778, output);
}

#[test]
fn test_gold_sample_1() {
    let output = Day::calculate_gold(include_str!("input_sample_gold1.txt"));
    assert_eq!(4, output);
}

#[test]
fn test_gold_sample_1_alt() {
    let output = Day::calculate_gold(include_str!("input_sample_gold1_alt.txt"));
    assert_eq!(4, output);
}

#[test]
fn test_gold_sample_2() {
    let output = Day::calculate_gold(include_str!("input_sample_gold2.txt"));
    assert_eq!(8, output);
}

#[test]
fn test_gold_sample_3() {
    let output = Day::calculate_gold(include_str!("input_sample_gold3.txt"));
    assert_eq!(10, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(433, output);
}

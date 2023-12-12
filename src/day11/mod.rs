use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 11;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // duplicate empty columns
        for x in (0..grid[0].len()).rev() {
            if (0..grid.len()).all(|y| grid[y][x] == '.') {
                for row in &mut grid {
                    row.insert(x, '.');
                }
            }
        }

        // duplicate empty rows
        for y in (0..grid.len()).rev() {
            if grid[y].iter().all(|c| *c == '.') {
                grid.insert(y, vec!['.'; grid[0].len()]);
            }
        }

        debug_assert!(grid.iter().all(|r| r.len() == grid[0].len()));

        let hashes = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(|(_, c)| **c == '#')
            .map(|(p, _)| p)
            .collect::<Vec<_>>();

        let mut sum = 0;
        for i in 0..hashes.len() {
            // manhattan distance between hashes
            let a = hashes[i];
            for b in hashes.iter().skip(i + 1) {
                let dx = (a.0 as isize - b.0 as isize).abs();
                let dy = (a.1 as isize - b.1 as isize).abs();
                sum += dx + dy;
            }
        }

        sum as usize
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mul_const = 1_000_000 - 1;
        // let mul_const = 10 - 1;

        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut hashes = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((x, y), c)))
            .filter(|(_, c)| **c == '#')
            .map(|(p, _)| p)
            .collect::<Vec<_>>();

        // duplicate empty columns
        for x in (0..grid[0].len()).rev() {
            let extend_count = (0..x)
                .filter(|x| grid.iter().all(|row| row[*x] == '.'))
                .count();

            hashes
                .iter_mut()
                .filter(|(x_, _)| *x_ == x)
                .for_each(|(x, _)| *x += mul_const * extend_count);
        }

        // duplicate empty rows
        for y in (0..grid.len()).rev() {
            let extend_count = (0..y)
                .filter(|y| grid[*y].iter().all(|c| *c == '.'))
                .count();

            hashes
                .iter_mut()
                .filter(|(_, y_)| *y_ == y)
                .for_each(|(_, y)| *y += mul_const * extend_count);
        }

        let mut sum = 0;
        for i in 0..hashes.len() {
            // manhattan distance between hashes
            let a = hashes[i];
            for b in hashes.iter().skip(i + 1) {
                let dx = (a.0 as isize - b.0 as isize).abs();
                let dy = (a.1 as isize - b.1 as isize).abs();
                sum += dx + dy;
            }
        }

        sum as usize
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(374, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(10313550, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(82000210, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(611998089572, output);
}

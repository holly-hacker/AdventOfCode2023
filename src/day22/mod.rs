use ahash::{AHashMap, AHashSet};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 22;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut bricks = parse_input(input);
        simulate_falling(&mut bricks);

        // loop over bricks to see which ones are supporting others
        let mut unsupported_bricks = Vec::new();
        let lookup = build_position_lut(&bricks);
        for brick_to_remove_i in 0..bricks.len() {
            for falling_brick in &bricks {
                let is_on_ground = falling_brick.0 .2 == 1 || falling_brick.1 .2 == 1;

                let is_supported = is_on_ground
                    || get_brick_iterator(*falling_brick)
                        .filter(|(x, y, z)| {
                            let voxel_below_is_own_shape =
                                (falling_brick.0 .0..=falling_brick.1 .0).contains(x)
                                    && (falling_brick.0 .1..=falling_brick.1 .1).contains(y)
                                    && (falling_brick.0 .2..=falling_brick.1 .2).contains(&(z - 1));

                            !voxel_below_is_own_shape
                        })
                        .any(|(fall_x, fall_y, fall_z)| {
                            let fall_idx = lookup.get(&(fall_x, fall_y, fall_z - 1)).copied();
                            let is_supported =
                                matches!(fall_idx, Some(idx) if idx != brick_to_remove_i);
                            is_supported
                        });

                if !is_supported {
                    unsupported_bricks.push(brick_to_remove_i);
                    break;
                }
            }
        }

        bricks.len() - unsupported_bricks.len()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut bricks = parse_input(input);
        simulate_falling(&mut bricks);

        // loop over bricks to see which ones are supporting others
        let mut unsupported_bricks = Vec::new();
        let lookup = build_position_lut(&bricks);
        for brick_to_remove_i in 0..bricks.len() {
            for falling_brick in &bricks {
                let is_on_ground = falling_brick.0 .2 == 1 || falling_brick.1 .2 == 1;

                let is_supported = is_on_ground
                    || get_brick_iterator(*falling_brick)
                        .filter(|(x, y, z)| {
                            let voxel_below_is_own_shape =
                                (falling_brick.0 .0..=falling_brick.1 .0).contains(x)
                                    && (falling_brick.0 .1..=falling_brick.1 .1).contains(y)
                                    && (falling_brick.0 .2..=falling_brick.1 .2).contains(&(z - 1));

                            !voxel_below_is_own_shape
                        })
                        .any(|(fall_x, fall_y, fall_z)| {
                            let fall_idx = lookup.get(&(fall_x, fall_y, fall_z - 1)).copied();
                            let is_supported =
                                matches!(fall_idx, Some(idx) if idx != brick_to_remove_i);
                            is_supported
                        });

                if !is_supported {
                    let mut cloned = bricks.clone();
                    cloned.swap_remove(brick_to_remove_i);
                    let would_fall = simulate_falling(&mut cloned);
                    unsupported_bricks.push(would_fall);
                    break;
                }
            }
        }

        unsupported_bricks.into_iter().sum()
    }
}

fn parse_input(input: &str) -> Vec<((usize, usize, usize), (usize, usize, usize))> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();

            let (start_x, start) = start.split_once(',').unwrap();
            let (start_y, start_z) = start.split_once(',').unwrap();

            let (end_x, end) = end.split_once(',').unwrap();
            let (end_y, end_z) = end.split_once(',').unwrap();

            (
                (
                    start_x.parse::<usize>().unwrap(),
                    start_y.parse::<usize>().unwrap(),
                    start_z.parse::<usize>().unwrap(),
                ),
                (
                    end_x.parse::<usize>().unwrap(),
                    end_y.parse::<usize>().unwrap(),
                    end_z.parse::<usize>().unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>()
}

fn get_brick_iterator(
    brick: ((usize, usize, usize), (usize, usize, usize)),
) -> impl Iterator<Item = (usize, usize, usize)> {
    (brick.0 .2..=brick.1 .2).flat_map(move |z| {
        (brick.0 .1..=brick.1 .1)
            .flat_map(move |y| (brick.0 .0..=brick.1 .0).map(move |x| (x, y, z)))
    })
}

fn build_position_lut(
    bricks: &[((usize, usize, usize), (usize, usize, usize))],
) -> AHashMap<(usize, usize, usize), usize> {
    let mut map = AHashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        get_brick_iterator(*brick).for_each(|(x, y, z)| {
            map.insert((x, y, z), i);
        });
    }

    map
}

fn simulate_falling(bricks: &mut [((usize, usize, usize), (usize, usize, usize))]) -> usize {
    let mut changed = true;
    let mut fallen = AHashSet::new();

    let mut lookup = build_position_lut(bricks);
    while changed {
        changed = false;

        for own_i in 0..bricks.len() {
            let own = bricks[own_i];
            // check if the item under the brick is either us, or empty

            if own.0 .2 == 1 || own.1 .2 == 1 {
                continue;
            }

            let can_fall = get_brick_iterator(own)
                .filter(|(x, y, z)| {
                    let voxel_below_is_own_shape = (own.0 .0..=own.1 .0).contains(x)
                        && (own.0 .1..=own.1 .1).contains(y)
                        && (own.0 .2..=own.1 .2).contains(&(z - 1));

                    !voxel_below_is_own_shape
                })
                .all(|(x, y, z)| {
                    let shape_idx = lookup.get(&(x, y, z - 1)).copied();
                    let is_supported = matches!(shape_idx, Some(idx) if idx != own_i);
                    !is_supported
                });

            if can_fall {
                bricks[own_i].0 .2 -= 1;
                bricks[own_i].1 .2 -= 1;
                changed = true;
                fallen.insert(own_i);

                // patch the lookup table instead of fully rebuilding it
                lookup.retain(|_, v| *v != own_i);
                for z in own.0 .2..=own.1 .2 {
                    for y in own.0 .1..=own.1 .1 {
                        for x in own.0 .0..=own.1 .0 {
                            lookup.insert((x, y, z - 1), own_i);
                        }
                    }
                }
            }
        }
    }

    fallen.len()
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(5, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(430, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(7, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(60558, output);
}

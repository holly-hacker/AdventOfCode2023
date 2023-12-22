use ahash::AHashSet;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 22;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut bricks = input
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
            .collect::<Vec<_>>();

        simulate_falling(&mut bricks);

        // bricks have settled
        // dbg!(&bricks);

        // loop over bricks to see which ones are supporting others
        let mut bricks_1 = Vec::new();
        for brick_to_remove_i in 0..bricks.len() {
            for falling_brick in &bricks {
                let is_on_ground = falling_brick.0 .2 == 1 || falling_brick.1 .2 == 1;

                let is_supported = is_on_ground
                    || (falling_brick.0 .2..=falling_brick.1 .2)
                        .flat_map(|z| {
                            (falling_brick.0 .1..=falling_brick.1 .1).flat_map(move |y| {
                                (falling_brick.0 .0..=falling_brick.1 .0).map(move |x| (x, y, z))
                            })
                        })
                        .filter(|(x, y, z)| {
                            let voxel_below_is_own_shape =
                                (falling_brick.0 .0..=falling_brick.1 .0).contains(x)
                                    && (falling_brick.0 .1..=falling_brick.1 .1).contains(y)
                                    && (falling_brick.0 .2..=falling_brick.1 .2).contains(&(z - 1));

                            !voxel_below_is_own_shape
                        })
                        .any(|(fall_x, fall_y, fall_z)| {
                            let is_supported = bricks.iter().enumerate().any(|(other_i, other)| {
                                let other_x = other.0 .0..=other.1 .0;
                                let other_y = other.0 .1..=other.1 .1;
                                let other_z = other.0 .2..=other.1 .2;

                                other_i != brick_to_remove_i
                                    && other_x.contains(&fall_x)
                                    && other_y.contains(&fall_y)
                                    && other_z.contains(&(fall_z - 1))
                            });

                            is_supported
                        });

                if !is_supported {
                    // println!(
                    //     "When {} is removed, {} can fall",
                    //     char::from(b'A' + brick_to_remove_i as u8),
                    //     char::from(b'A' + falling_brick_i as u8),
                    // );
                    bricks_1.push(brick_to_remove_i);
                    break;
                }
            }
        }

        bricks.len() - bricks_1.len()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let mut bricks = input
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
            .collect::<Vec<_>>();

        simulate_falling(&mut bricks);

        // bricks have settled
        // dbg!(&bricks);

        // loop over bricks to see which ones are supporting others
        let mut bricks_1 = Vec::new();
        for brick_to_remove_i in 0..bricks.len() {
            // println!("To remove: {brick_to_remove_i}/{}", bricks.len());
            for falling_brick in &bricks {
                let is_on_ground = falling_brick.0 .2 == 1 || falling_brick.1 .2 == 1;

                let is_supported = is_on_ground
                    || (falling_brick.0 .2..=falling_brick.1 .2)
                        .flat_map(|z| {
                            (falling_brick.0 .1..=falling_brick.1 .1).flat_map(move |y| {
                                (falling_brick.0 .0..=falling_brick.1 .0).map(move |x| (x, y, z))
                            })
                        })
                        .filter(|(x, y, z)| {
                            let voxel_below_is_own_shape =
                                (falling_brick.0 .0..=falling_brick.1 .0).contains(x)
                                    && (falling_brick.0 .1..=falling_brick.1 .1).contains(y)
                                    && (falling_brick.0 .2..=falling_brick.1 .2).contains(&(z - 1));

                            !voxel_below_is_own_shape
                        })
                        .any(|(fall_x, fall_y, fall_z)| {
                            let is_supported = bricks.iter().enumerate().any(|(other_i, other)| {
                                let other_x = other.0 .0..=other.1 .0;
                                let other_y = other.0 .1..=other.1 .1;
                                let other_z = other.0 .2..=other.1 .2;

                                other_i != brick_to_remove_i
                                    && other_x.contains(&fall_x)
                                    && other_y.contains(&fall_y)
                                    && other_z.contains(&(fall_z - 1))
                            });

                            is_supported
                        });

                if !is_supported {
                    let mut cloned = bricks.clone();
                    cloned.remove(brick_to_remove_i);
                    let would_fall = simulate_falling(&mut cloned);
                    // println!(
                    //     "When {} is removed, {} can fall",
                    //     char::from(b'A' + brick_to_remove_i as u8),
                    //     char::from(b'A' + falling_brick_i as u8),
                    // );
                    bricks_1.push(would_fall);
                    break;
                }
            }
        }

        bricks_1.into_iter().sum()
    }
}

fn simulate_falling(bricks: &mut Vec<((usize, usize, usize), (usize, usize, usize))>) -> usize {
    let mut changed = true;
    let mut fallen = AHashSet::new();
    while changed {
        changed = false;

        for own_i in 0..bricks.len() {
            let own = bricks[own_i];
            // check if the item under the brick is either us, or empty

            if own.0 .2 == 1 || own.1 .2 == 1 {
                continue;
            }

            let can_fall = (own.0 .2..=own.1 .2)
                .flat_map(|z| {
                    (own.0 .1..=own.1 .1)
                        .flat_map(move |y| (own.0 .0..=own.1 .0).map(move |x| (x, y, z)))
                })
                .all(|(x, y, z)| {
                    ((own.0 .0..=own.1 .0).contains(&x)
                        && (own.0 .1..=own.1 .1).contains(&y)
                        && (own.0 .2..=own.1 .2).contains(&(z - 1)))
                        || !bricks.iter().any(|other| {
                            let other_x = other.0 .0..=other.1 .0;
                            let other_y = other.0 .1..=other.1 .1;
                            let other_z = other.0 .2..=other.1 .2;

                            other_x.contains(&x)
                                && other_y.contains(&y)
                                && other_z.contains(&(z - 1))
                        })
                });

            if can_fall {
                // println!("Falling brick {:?}", own_i);
                bricks[own_i].0 .2 -= 1;
                bricks[own_i].1 .2 -= 1;
                changed = true;
                fallen.insert(own_i);
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

// #[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(630129824772393, output);
}

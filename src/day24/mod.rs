use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 24;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input
            .lines()
            .map(|l| {
                let (p, v) = l.split_once(" @ ").unwrap();
                let (p_x, p_yz) = p.split_once(", ").unwrap();
                let (p_y, p_z) = p_yz.split_once(", ").unwrap();

                let (v_x, v_yz) = v.split_once(", ").unwrap();
                let (v_y, v_z) = v_yz.split_once(", ").unwrap();

                (
                    (
                        p_x.trim().parse::<f64>().unwrap(),
                        p_y.trim().parse::<f64>().unwrap(),
                        p_z.trim().parse::<f64>().unwrap(),
                    ),
                    (
                        v_x.trim().parse::<f64>().unwrap(),
                        v_y.trim().parse::<f64>().unwrap(),
                        v_z.trim().parse::<f64>().unwrap(),
                    ),
                )
            })
            .collect::<Vec<_>>();

        let range = if input.len() < 10 {
            7.0..=27.0
        } else {
            200000000000000.0..=400000000000000.0
        };
        let mut sum = 0;
        for i1 in 0..input.len() {
            let ((px_1, py_1, _), (vx_1, vy_1, _)) = input[i1];

            assert!(vx_1 != 0.);
            let slope_1 = vy_1 / vx_1;

            for i2 in (i1 + 1)..input.len() {
                // check if it intersects on x,y somewhere
                let ((px_2, py_2, _), (vx_2, vy_2, _)) = input[i2];

                assert!(vx_2 != 0.);
                let slope_2 = vy_2 / vx_2;

                if slope_1 == slope_2 {
                    // never intersects
                    // println!("{i1} {i2} -> never intersects");
                    continue;
                }

                // don't ask
                let x = (px_1 * slope_1 - py_1 - px_2 * slope_2 + py_2) / (slope_1 - slope_2);
                let y = slope_1 * (x - px_1) + py_1;

                let t_1 = (x - px_1) * vx_1.signum();
                let t_2 = (x - px_2) * vx_2.signum();

                if t_1.is_sign_negative() {
                    // println!("{i1} {i2} -> {x} {y} (t_1 is negative)");
                    continue;
                }
                if t_2.is_sign_negative() {
                    // println!("{i1} {i2} -> {x} {y} (t_2 is negative)");
                    continue;
                }

                if range.contains(&x) && range.contains(&y) {
                    sum += 1;
                    // println!("{i1} {i2} -> {x} {y} (inside area)");
                } else {
                    // println!("{i1} {i2} -> {x} {y} (outside area)");
                }
            }
        }

        sum
    }
}

/*
a_1 + x_1 * t_1 = a_0 + x_0 * t_1
a_2 + x_2 * t_2 = a_0 + x_0 * t_2
a_3 + x_3 * t_3 = a_0 + x_0 * t_3

b_1 + y_1 * t_1 = b_0 + y_0 * t_1
b_2 + y_2 * t_2 = b_0 + y_0 * t_2
b_3 + y_3 * t_3 = b_0 + y_0 * t_3

c_1 + z_1 * t_1 = c_0 + z_0 * t_1
c_2 + z_2 * t_2 = c_0 + z_0 * t_2
c_3 + z_3 * t_3 = c_0 + z_0 * t_3

# extract t_1 by subtracting the first equation from the second which eliminates a_0
(a_2 + x_2 * t_2) - (a_1 + x_1 * t_1) = (a_0 + x_0 * t_2) - (a_0 + x_0 * t_1)
a_2 + x_2 * t_2 - a_1 - x_1 * t_1 = a_0 + x_0 * t_2 - a_0 - x_0 * t_1
a_2 - a_1 + x_2 * t_2 - x_1 * t_1 = x_0 * (t_2 - t_1)

t_1 = (a_1 - a_2 + t_2 * (x_0 - x_2)) / (x_0 - x_1)

# we need an equation that has at most 1 unknown that is not {x,y,z}_0

# somehow brute-force x_0, y_0, z_0. these probably have the lowest range of values
# only one of of t_{n} and {a,b,c}_0 may be part the equation, the other unknowns should be
# {x,y,z}_0.

t_1 = (a_1 - a_2 + t_2 * (x_0 - x_2)) / (x_0 - x_1)
t_1 = (b_1 - b_2 + t_2 * (y_0 - y_2)) / (y_0 - y_1)
t_1 = (c_1 - c_2 + t_2 * (z_0 - z_2)) / (z_0 - z_1)

(a_1 - a_2 + t_2 * (x_0 - x_2)) / (x_0 - x_1) = (b_1 - b_2 + t_2 * (y_0 - y_2)) / (y_0 - y_1)

# solve for t_2 (thank you, wolfram alpha)
t_2 = ((a_1 - a_2) * (y_0 - y_1) + b_2 * (x_0 - x_1) + b_1 * (x_1 - x_0))
    / (x_2 * (y_0 - y_1) + x_0 * (y_1 - y_2) + x_1 * (y_2 - y_0))

# additional formulas to calculate the rest
x_0 = (-a_0 + a_2 + t_2*x_2) / t_2
x_0 = (a_1 - a_2 + t_1 * x_1 - t_2 * x_2) / (t_1 - t_2)
a_0 = a_1 + t_1 * (x_1 - x_0)

*/

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let input = input
            .lines()
            .map(|l| {
                let (p, v) = l.split_once(" @ ").unwrap();
                let (p_x, p_yz) = p.split_once(", ").unwrap();
                let (p_y, p_z) = p_yz.split_once(", ").unwrap();

                let (v_x, v_yz) = v.split_once(", ").unwrap();
                let (v_y, v_z) = v_yz.split_once(", ").unwrap();

                (
                    (
                        p_x.trim().parse::<f64>().unwrap(),
                        p_y.trim().parse::<f64>().unwrap(),
                        p_z.trim().parse::<f64>().unwrap(),
                    ),
                    (
                        v_x.trim().parse::<f64>().unwrap(),
                        v_y.trim().parse::<f64>().unwrap(),
                        v_z.trim().parse::<f64>().unwrap(),
                    ),
                )
            })
            .collect::<Vec<_>>();

        let ((a_1, b_1, c_1), (x_1, y_1, z_1)) = input[0];
        let ((a_2, b_2, c_2), (x_2, y_2, z_2)) = input[1];
        let ((a_3, b_3, _), (x_3, y_3, _)) = input[2];

        let mut search_depth = 0;
        loop {
            search_depth += 1;
            // println!("searching with depth {}", search_depth);

            // TODO: optimize this, we don't need to do inner values since we did them before
            let range_x = -search_depth..=search_depth;
            let range_y = -search_depth..=search_depth;
            let range_z = -search_depth..=search_depth;

            for x_0 in range_x.clone() {
                for y_0 in range_y.clone() {
                    for z_0 in range_z.clone() {
                        let x_0 = x_0 as f64;
                        let y_0 = y_0 as f64;
                        let z_0 = z_0 as f64;

                        let t_2_xy =
                            ((a_1 - a_2) * (y_0 - y_1) + b_2 * (x_0 - x_1) + b_1 * (x_1 - x_0))
                                / (x_2 * (y_0 - y_1) + x_0 * (y_1 - y_2) + x_1 * (y_2 - y_0));

                        let t_2_xz =
                            ((a_1 - a_2) * (z_0 - z_1) + c_2 * (x_0 - x_1) + c_1 * (x_1 - x_0))
                                / (x_2 * (z_0 - z_1) + x_0 * (z_1 - z_2) + x_1 * (z_2 - z_0));

                        let t_2_yz =
                            ((b_1 - b_2) * (z_0 - z_1) + c_2 * (y_0 - y_1) + c_1 * (y_1 - y_0))
                                / (y_2 * (z_0 - z_1) + y_0 * (z_1 - z_2) + y_1 * (z_2 - z_0));

                        let diff_xy_xz = (t_2_xy - t_2_xz).abs();
                        let diff_xy_yz = (t_2_xy - t_2_yz).abs();
                        let diff_xz_yz = (t_2_xz - t_2_yz).abs();

                        if diff_xy_yz < 0.01 && diff_xy_xz < 0.01 && diff_xz_yz < 0.01 {
                            // println!("found matching t_2 value: {x_0}, {y_0}, {z_0} -> {t_2_xy}");
                            // return 0;
                        } else {
                            continue;
                        }

                        let t_2 = t_2_xy;

                        // calculate t_1
                        let t_1_x = (a_1 - a_2 + t_2 * (x_0 - x_2)) / (x_0 - x_1);
                        let t_1 = t_1_x;

                        // TODO: for some reason, this results in a wrong result for sample but
                        // correct for the real input. why? has I ever?
                        // let t_3_x = (a_3 - a_2 + t_2 * (x_0 - x_2)) / (x_0 - x_3);
                        // let t_3 = t_3_x;

                        let t_3_xy =
                            ((a_1 - a_3) * (y_0 - y_1) + b_3 * (x_0 - x_1) + b_1 * (x_1 - x_0))
                                / (x_3 * (y_0 - y_1) + x_0 * (y_1 - y_3) + x_1 * (y_3 - y_0));
                        let t_3 = t_3_xy;

                        let a_0_1 = a_1 + t_1 * (x_1 - x_0);
                        let a_0_3 = a_3 + t_3 * (x_3 - x_0);

                        if a_0_1 == a_0_3 {
                            // println!("found matching a_0 value!: {x_0}, {y_0}, {z_0} -> t1={t_1}, t2={t_2}, t3={t_3}");
                            // dbg!(t_1_x, t_1_y, t_1_z);

                            let a_0 = a_0_1;
                            let b_0 = b_1 + t_1 * (y_1 - y_0);
                            let c_0 = c_1 + t_1 * (z_1 - z_0);

                            return (a_0 + b_0 + c_0) as usize;
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(2, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    // less than 25186
    assert_eq!(18184, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(47, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(557789988450159, output);
}

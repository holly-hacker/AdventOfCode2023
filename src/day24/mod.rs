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

I initially did a bunch of math to extract `x_0` and wrote a somewhat smart brute-forcer for `x_0`,
but even after a search depth of >2.5m for `t`, it didnt find anything.

I'm still not sure what the "proper" method is, I may look into that later. I assume I need to
brute-force the velocity (as that is the only thing that is semi-guaranteed to be fairly low), but
I don't know how to extract that out of the equations above.
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
                        p_x.trim().parse::<u64>().unwrap(),
                        p_y.trim().parse::<u64>().unwrap(),
                        p_z.trim().parse::<u64>().unwrap(),
                    ),
                    (
                        v_x.trim().parse::<i64>().unwrap(),
                        v_y.trim().parse::<i64>().unwrap(),
                        v_z.trim().parse::<i64>().unwrap(),
                    ),
                )
            })
            .collect::<Vec<_>>();

        let ((a_1, b_1, c_1), (x_1, y_1, z_1)) = input[0];
        let ((a_2, b_2, c_2), (x_2, y_2, z_2)) = input[1];
        let ((a_3, b_3, c_3), (x_3, y_3, z_3)) = input[2];

        use z3::ast;
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let t_1 = ast::Int::new_const(&ctx, "t_1");
        let t_2 = ast::Int::new_const(&ctx, "t_2");
        let t_3 = ast::Int::new_const(&ctx, "t_3");
        let a_0 = ast::Int::new_const(&ctx, "a_0");
        let b_0 = ast::Int::new_const(&ctx, "b_0");
        let c_0 = ast::Int::new_const(&ctx, "c_0");
        let x_0 = ast::Int::new_const(&ctx, "x_0");
        let y_0 = ast::Int::new_const(&ctx, "y_0");
        let z_0 = ast::Int::new_const(&ctx, "z_0");
        let a_1 = ast::Int::from_u64(&ctx, a_1);
        let b_1 = ast::Int::from_u64(&ctx, b_1);
        let c_1 = ast::Int::from_u64(&ctx, c_1);
        let x_1 = ast::Int::from_i64(&ctx, x_1);
        let y_1 = ast::Int::from_i64(&ctx, y_1);
        let z_1 = ast::Int::from_i64(&ctx, z_1);
        let a_2 = ast::Int::from_u64(&ctx, a_2);
        let b_2 = ast::Int::from_u64(&ctx, b_2);
        let c_2 = ast::Int::from_u64(&ctx, c_2);
        let x_2 = ast::Int::from_i64(&ctx, x_2);
        let y_2 = ast::Int::from_i64(&ctx, y_2);
        let z_2 = ast::Int::from_i64(&ctx, z_2);
        let a_3 = ast::Int::from_u64(&ctx, a_3);
        let b_3 = ast::Int::from_u64(&ctx, b_3);
        let c_3 = ast::Int::from_u64(&ctx, c_3);
        let x_3 = ast::Int::from_i64(&ctx, x_3);
        let y_3 = ast::Int::from_i64(&ctx, y_3);
        let z_3 = ast::Int::from_i64(&ctx, z_3);

        use std::ops::{Add, Mul};
        use z3::ast::Ast;
        let solver = z3::Solver::new(&ctx);
        solver.assert(&a_1.add(&x_1.mul(&t_1))._eq(&(&a_0).add(&(&x_0).mul(&t_1))));
        solver.assert(&a_2.add(&x_2.mul(&t_2))._eq(&(&a_0).add(&(&x_0).mul(&t_2))));
        solver.assert(&a_3.add(&x_3.mul(&t_3))._eq(&(&a_0).add(&(&x_0).mul(&t_3))));

        solver.assert(&b_1.add(&y_1.mul(&t_1))._eq(&(&b_0).add(&(&y_0).mul(&t_1))));
        solver.assert(&b_2.add(&y_2.mul(&t_2))._eq(&(&b_0).add(&(&y_0).mul(&t_2))));
        solver.assert(&b_3.add(&y_3.mul(&t_3))._eq(&(&b_0).add(&(&y_0).mul(&t_3))));

        solver.assert(&c_1.add(&z_1.mul(&t_1))._eq(&(&c_0).add(&(&z_0).mul(&t_1))));
        solver.assert(&c_2.add(&z_2.mul(&t_2))._eq(&(&c_0).add(&(&z_0).mul(&t_2))));
        solver.assert(&c_3.add(&z_3.mul(&t_3))._eq(&(&c_0).add(&(&z_0).mul(&t_3))));

        let result = solver.check();
        assert_eq!(result, z3::SatResult::Sat);

        let model = solver.get_model().unwrap();
        let a_0_value = model.eval(&a_0, true).unwrap().as_i64().unwrap();
        let b_0_value = model.eval(&b_0, true).unwrap().as_i64().unwrap();
        let c_0_value = model.eval(&c_0, true).unwrap().as_i64().unwrap();

        (a_0_value + b_0_value + c_0_value) as usize
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

#![allow(clippy::missing_const_for_fn)]

use criterion::criterion_main;

macro_rules! gen_benchmarks {
    ( $($day_index:expr,)* ) => {
        $(
            paste::paste! {
                #[cfg(feature = "day" $day_index)]
                pub fn [<day_ $day_index _benches>](c: &mut criterion::Criterion) {
                    use criterion::black_box;
                    use aoc2023::*;

                    c.bench_function(&format!("Day {} silver (sample)", stringify!($day_index)), |b| {
                        use [<day $day_index>]::Day;
                        b.iter(|| Day::calculate_silver(black_box(Day::INPUT_SAMPLE)))
                    });
                    c.bench_function(&format!("Day {} silver (real)", stringify!($day_index)), |b| {
                        use [<day $day_index>]::Day;
                        b.iter(|| Day::calculate_silver(black_box(Day::INPUT_REAL)))
                    });
                    c.bench_function(&format!("Day {} gold (sample)", stringify!($day_index)), |b| {
                        use [<day $day_index>]::Day;
                        b.iter(|| Day::calculate_gold(black_box(Day::INPUT_SAMPLE)))
                    });
                    c.bench_function(&format!("Day {} gold (real)", stringify!($day_index)), |b| {
                        use [<day $day_index>]::Day;
                        b.iter(|| Day::calculate_gold(black_box(Day::INPUT_REAL)))
                    });
                }

                cfg_if::cfg_if! {
                    if #[cfg(feature = "day" $day_index)] {
                        criterion::criterion_group!([<day_ $day_index>], [<day_ $day_index _benches>]);
                    } else {
                        fn [<day_ $day_index>]() {}
                    }
                }

            }
        )*

        paste::paste!{
            criterion_main! {
                $([<day_ $day_index>],)*
            }
        }
    }
}

gen_benchmarks! {
    01,
}

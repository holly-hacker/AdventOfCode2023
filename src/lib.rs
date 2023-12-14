#![allow(
    clippy::cast_possible_truncation,
    clippy::wildcard_imports,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::unreadable_literal,
    clippy::identity_op,
    clippy::type_complexity,
    clippy::option_if_let_else,
    clippy::needless_range_loop
)]

use std::fmt::Display;

mod utils;

macro_rules! register_days {
    ( $($day_index:literal $type:ident,)* ) => {
        // add `mod`
        $(paste::paste! { #[cfg(feature = "day" $day_index)] pub mod [<day $day_index>]; })*

        /// Run implemenation for all days that are included in the feature set
        pub fn execute_all() {
            $(paste::paste! {
                #[cfg(feature = "day" $day_index)] register_days!(impl $day_index $type);
            })*
        }
    };
    (impl $day_index:literal gold  ) => { paste::paste! { [<day $day_index>]::Day::execute(); }};
    (impl $day_index:literal silver) => { paste::paste! { [<day $day_index>]::Day::execute_silver(); }};
}

// === Register days here! ===
register_days! {
    01 gold,
    02 gold,
    03 gold,
    04 gold,
    05 gold,
    06 gold,
    07 gold,
    08 gold,
    09 gold,
    10 gold,
    11 gold,
    12 gold,
    13 gold,
    14 gold,
}

fn run_timed<T, F>(fun: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let now = std::time::Instant::now();
    let ret = fun();
    let elapsed = now.elapsed();
    (ret, elapsed)
}

pub trait SolutionSilver<TSilver: Display> {
    const DAY: u32;
    const INPUT_SAMPLE: &'static str;
    const INPUT_REAL: &'static str;

    fn execute_silver() {
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_SAMPLE));
        println!("Day {:02}, silver (s) | {time:>10?} | {output}", Self::DAY);
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_REAL));
        println!("Day {:02}, silver     | {time:>10?} | {output}", Self::DAY);

        // println!("Day {:02} has no gold implementation", Self::DAY);
    }

    fn calculate_silver(input: &str) -> TSilver;
}

pub trait SolutionGold<TSilver: Display, TGold: Display>: SolutionSilver<TSilver> {
    const INPUT_SAMPLE_GOLD: &'static str = Self::INPUT_SAMPLE;

    fn execute() {
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_SAMPLE));
        println!("Day {:02}, silver (s) | {time:>10?} | {output}", Self::DAY);
        let (output, time) = run_timed(|| Self::calculate_silver(Self::INPUT_REAL));
        println!("Day {:02}, silver     | {time:>10?} | {output}", Self::DAY);

        let (output, time) = run_timed(|| Self::calculate_gold(Self::INPUT_SAMPLE_GOLD));
        println!("Day {:02}, gold (s)   | {time:>10?} | {output}", Self::DAY);
        let (output, time) = run_timed(|| Self::calculate_gold(Self::INPUT_REAL));
        println!("Day {:02}, gold       | {time:>10?} | {output}", Self::DAY);

        #[cfg(feature = "profile")]
        for _ in 0..100 {
            Self::calculate_gold(Self::INPUT_REAL);
        }
    }

    fn calculate_gold(input: &str) -> TGold;
}

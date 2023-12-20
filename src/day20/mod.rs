use std::collections::VecDeque;

use ahash::AHashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 20;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let map = input
            .lines()
            .map(|l| {
                let (start, targets) = l.split_once(" -> ").unwrap();

                let (prefix, module) = match start.chars().next().unwrap() {
                    '&' => (Some('&'), start.trim_start_matches('&')),
                    '%' => (Some('%'), start.trim_start_matches('%')),
                    _ => (None, start),
                };

                (module, (prefix, targets))
            })
            .collect::<AHashMap<_, _>>();

        // convert parsed lines to nodes
        let mut nodes = map
            .iter()
            .map(|(module, (prefix, targets))| {
                let targets = targets.split(", ").collect::<Vec<_>>();
                let node = match prefix {
                    Some('%') => Node::FlipFlop { state: false },
                    Some('&') => Node::Conjunction {
                        input_states: map
                            .iter()
                            .filter(|(_, (_, v_targets))| v_targets.contains(module))
                            .map(|(k, (_, _))| (*k, false))
                            .collect::<Vec<_>>(),
                    },
                    None => Node::Broadcast,
                    _ => unreachable!(),
                };

                (*module, (node, targets))
            })
            .collect::<AHashMap<_, _>>();

        let mut count_high = 0;
        let mut count_low = 0;

        let mut queue = VecDeque::new();
        for _iteration in 0..1000 {
            queue.push_back(("button", "broadcaster", false));

            while let Some((from, to, incoming_signal)) = queue.pop_front() {
                if incoming_signal {
                    count_high += 1;
                } else {
                    count_low += 1;
                }

                let Some((node, targets)) = nodes.get_mut(to) else {
                    continue;
                };

                let outgoing_signal = match node {
                    Node::FlipFlop { state } => {
                        if incoming_signal {
                            continue;
                        }

                        // flip if low
                        *state = !*state;

                        *state
                    }
                    Node::Conjunction { input_states } => {
                        if let Some((_, v)) = input_states.iter_mut().find(|(k, _)| *k == from) {
                            *v = incoming_signal;
                        }

                        !input_states.iter().all(|(_, v)| *v)
                    }
                    Node::Broadcast => incoming_signal,
                };

                for target in targets {
                    queue.push_back((to, *target, outgoing_signal));
                }
            }
        }

        count_low * count_high
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let map = input
            .lines()
            .map(|l| {
                let (start, end) = l.split_once(" -> ").unwrap();

                let (prefix, module) = match start.chars().next().unwrap() {
                    '&' => (Some('&'), start.trim_start_matches('&')),
                    '%' => (Some('%'), start.trim_start_matches('%')),
                    _ => (None, start),
                };
                let targets = end.split(", ").collect::<Vec<_>>();

                (module, (prefix, targets))
            })
            .collect::<AHashMap<_, _>>();

        if !map.values().any(|v| v.1.contains(&"rx")) {
            return usize::MAX;
        }

        // convert parsed lines to nodes
        let mut nodes = map
            .iter()
            .map(|(module, (prefix, targets))| {
                let node = match prefix {
                    Some('%') => Node::FlipFlop { state: false },
                    Some('&') => Node::Conjunction {
                        input_states: map
                            .iter()
                            .filter(|(_, (_, v_targets))| v_targets.contains(module))
                            .map(|(k, (_, _))| (*k, false))
                            .collect::<Vec<_>>(),
                    },
                    None => Node::Broadcast,
                    _ => unreachable!(),
                };

                (*module, (node, targets))
            })
            .collect::<AHashMap<_, _>>();

        // run simulation for a while
        const SAMPLE_SIZE: usize = 10_000;
        let mut queue = VecDeque::new();
        let mut states = vec![];
        for iteration in 0..SAMPLE_SIZE {
            queue.push_back(("button", "broadcaster", false));

            while let Some((from, to, incoming_signal)) = queue.pop_front() {
                // this is not going to get hit, but it's here for correctness
                if to == "rx" && !incoming_signal {
                    return iteration + 1;
                }

                let Some((node, targets)) = nodes.get_mut(to) else {
                    continue;
                };

                let outgoing_signal = match node {
                    Node::FlipFlop { state } => {
                        if incoming_signal {
                            continue;
                        }

                        // flip if low
                        *state = !*state;

                        *state
                    }
                    Node::Conjunction { input_states } => {
                        if let Some((_, v)) = input_states.iter_mut().find(|(k, _)| *k == from) {
                            *v = incoming_signal;
                        }

                        !input_states.iter().all(|(_, v)| *v)
                    }
                    Node::Broadcast => incoming_signal,
                };

                for target in *targets {
                    queue.push_back((to, *target, outgoing_signal));
                }
            }

            // NOTE: this order is unstable, uses compile-time rng!
            let nodes_values = nodes
                .values()
                .map(|v| (v.0.fold_state_to_int()))
                .collect::<Vec<_>>();

            states.push(nodes_values);
        }

        // we've run the loop for a while, but we didn't write `high` to `rx`
        // assume we're in a very large loop and we'll write to `rx` at the end of the loop
        // this is a stupid assumption, but it seems to be how the challenge was designed

        let state_len = states.first().unwrap().len();
        let mut loops = vec![];
        'len: for i in 0..state_len {
            // check if this is a loop
            for loop_len in 1.. {
                let mut state_chunks = states.chunks(loop_len);
                let first_chunk_of_states = state_chunks.next().unwrap();
                let is_loop = state_chunks.all(|chunk_of_states| {
                    first_chunk_of_states
                        .iter()
                        .zip(chunk_of_states.iter())
                        .all(|(a, b)| a[i] == b[i])
                });

                if is_loop {
                    loops.push(loop_len);
                    continue 'len;
                }
            }

            panic!("no loop found for idx {}", i);
        }

        loops.iter().fold(1, |acc, len| lcm(acc, *len))
    }
}

// https://www.hackertouch.com/least-common-multiple-in-rust.html
fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node<'i> {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        input_states: Vec<(&'i str, bool)>, // essentially a map
    },
    Broadcast,
}

impl Node<'_> {
    fn fold_state_to_int(&self) -> u32 {
        match self {
            Node::FlipFlop { state } => {
                if *state {
                    1
                } else {
                    0
                }
            }
            Node::Conjunction { input_states } => input_states
                .iter()
                .enumerate()
                .fold(0, |acc, (i, (_, state))| {
                    acc | if *state { 1 << i } else { 0 }
                }),
            Node::Broadcast => 0,
        }
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(32000000, output);
}

#[test]
fn test_silver_sample_2() {
    let output = Day::calculate_silver(include_str!("input_sample_2.txt"));
    assert_eq!(11687500, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(712543680, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE);
    assert_eq!(usize::MAX, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(238920142622879, output);
}

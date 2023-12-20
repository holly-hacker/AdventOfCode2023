use std::collections::VecDeque;

use ahash::AHashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 20;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        // %: flip-plop. flips if receiving low pulse. when flipped, send high or low depending on
        // new state. initial state is low so first sent state is high.
        // &: conjunction. remembers last state for all inputs. if all states are high, send high.
        // otherwise send low.
        // broadcast: passes along input to outputs.
        // start of loop is sending `low` to broadcast.

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

        // convert parsed lines to nodes
        let mut nodes = map
            .iter()
            .map(|(module, (prefix, targets))| {
                let node = match prefix {
                    Some('%') => Node::FlipFlop {
                        state: false,
                        targets: targets.clone(),
                    },
                    Some('&') => Node::Conjunction {
                        input_states: map
                            .iter()
                            .filter(|(_, (_, v_targets))| v_targets.contains(module))
                            .map(|(k, (_, _))| (*k, false))
                            .collect::<Vec<_>>(),
                        targets: targets.clone(),
                    },
                    None => Node::Broadcast {
                        targets: targets.clone(),
                    },
                    _ => unreachable!(),
                };

                (*module, node)
            })
            .collect::<AHashMap<_, _>>();

        let mut count_low = 0;
        let mut count_high = 0;

        let mut queue = VecDeque::new();
        for _iteration in 0..1000 {
            let should_print = false; // iteration == 0;

            queue.push_back(("button", "broadcaster", false));
            should_print.then(|| println!("button -low-> broadcaster"));

            while let Some((from, to, signal)) = queue.pop_front() {
                if signal {
                    count_high += 1;
                } else {
                    count_low += 1;
                }

                let Some(node) = nodes.get_mut(to) else {
                    continue;
                };
                match node {
                    Node::FlipFlop { state, targets } => {
                        if !signal {
                            // flip if low
                            *state = !*state;

                            for target in targets {
                                should_print.then(|| {
                                    println!(
                                        "{to} -{}-> {}",
                                        if *state { "high" } else { "low" },
                                        target
                                    );
                                });
                                queue.push_back((to, *target, *state));
                            }
                        }
                    }
                    Node::Conjunction {
                        input_states,
                        targets,
                    } => {
                        input_states.iter_mut().for_each(|(k, v)| {
                            if *k == from {
                                *v = signal
                            }
                        });
                        // input_states.insert(from, signal);

                        let to_send = !input_states.iter().all(|(_, v)| *v);

                        for target in targets {
                            should_print.then(|| {
                                println!(
                                    "{to} -{}-> {}",
                                    if to_send { "high" } else { "low" },
                                    target
                                );
                            });
                            queue.push_back((to, *target, to_send));
                        }
                    }
                    Node::Broadcast { targets } => {
                        for target in targets {
                            should_print.then(|| {
                                println!(
                                    "{to} -{}-> {}",
                                    if signal { "high" } else { "low" },
                                    target
                                );
                            });
                            queue.push_back((to, *target, signal));
                        }
                    }
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
            // println!("no rx");
            return 0;
        }

        // convert parsed lines to nodes
        let mut nodes = map
            .iter()
            .map(|(module, (prefix, targets))| {
                let node = match prefix {
                    Some('%') => Node::FlipFlop {
                        state: false,
                        targets: targets.clone(),
                    },
                    Some('&') => Node::Conjunction {
                        input_states: map
                            .iter()
                            .filter(|(_, (_, v_targets))| v_targets.contains(module))
                            .map(|(k, (_, _))| (*k, false))
                            .collect::<Vec<_>>(),
                        targets: targets.clone(),
                    },
                    None => Node::Broadcast {
                        targets: targets.clone(),
                    },
                    _ => unreachable!(),
                };

                (*module, node)
            })
            .collect::<AHashMap<_, _>>();

        let mut queue = VecDeque::new();
        let mut states = vec![];
        const SAMPLE_SIZE: usize = 100_000;
        // assume we get the loops in 1m iterations
        for iteration in 0..SAMPLE_SIZE {
            let should_print = false;

            queue.push_back(("button", "broadcaster", false));
            should_print.then(|| println!("button -low-> broadcaster"));

            while let Some((from, to, signal)) = queue.pop_front() {
                if to == "rx" && !signal {
                    return iteration + 1;
                }

                let Some(node) = nodes.get_mut(to) else {
                    continue;
                };
                match node {
                    Node::FlipFlop { state, targets } => {
                        if !signal {
                            // flip if low
                            *state = !*state;

                            for target in targets {
                                should_print.then(|| {
                                    println!(
                                        "{to} -{}-> {target}",
                                        if *state { "high" } else { "low" },
                                    );
                                });
                                queue.push_back((to, *target, *state));
                            }
                        }
                    }
                    Node::Conjunction {
                        input_states,
                        targets,
                    } => {
                        input_states.iter_mut().for_each(|(k, v)| {
                            if *k == from {
                                *v = signal
                            }
                        });
                        // input_states.insert(from, signal);

                        let to_send = !input_states.iter().all(|(_, v)| *v);

                        for target in targets {
                            should_print.then(|| {
                                println!(
                                    "{to} -{}-> {target}",
                                    if to_send { "high" } else { "low" },
                                );
                            });
                            queue.push_back((to, *target, to_send));
                        }
                    }
                    Node::Broadcast { targets } => {
                        for target in targets {
                            should_print.then(|| {
                                println!(
                                    "{to} -{}-> {target}",
                                    if signal { "high" } else { "low" },
                                );
                            });
                            queue.push_back((to, *target, signal));
                        }
                    }
                }
            }

            // NOTE: this order is unstable, uses compile-time rng!
            let nodes_values = nodes
                .iter()
                .map(|(_, v)| (v.get_state()))
                .collect::<Vec<_>>();

            // println!("{iteration:06}: {nodes_values:?}");
            states.push(nodes_values);

            // for (idx, i) in nodes_values.iter().enumerate() {
            //     let prev = previous_states.insert((idx, *i), iteration);
            //     if let Some(prev) = prev {
            //         println!("loop detected: {} -> {}", prev, iteration);
            //     }
            // }
        }

        // we've exited the loop and collected our samples
        let len = states.first().unwrap().len();

        // println!("looking for loops!");
        let mut loops = vec![];
        'len: for i in 0..len {
            // println!("checking idx {i}");

            // check if this is a loop
            for loop_len in 1..(SAMPLE_SIZE / 3) {
                let mut state_chunks = states.chunks(loop_len);
                let first_chunk_of_states = state_chunks.next().unwrap();
                let is_loop = state_chunks.all(|chunk_of_states| {
                    first_chunk_of_states
                        .iter()
                        .zip(chunk_of_states.iter())
                        .all(|(a, b)| a[i] == b[i])
                });

                if is_loop {
                    // println!("loop found: {loop_len}");
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node<'i> {
    FlipFlop {
        state: bool,
        targets: Vec<&'i str>,
    },
    Conjunction {
        input_states: Vec<(&'i str, bool)>,
        targets: Vec<&'i str>,
    },
    Broadcast {
        targets: Vec<&'i str>,
    },
}

impl Node<'_> {
    fn get_state(&self) -> u32 {
        match self {
            Node::FlipFlop { state, .. } => {
                if *state {
                    1
                } else {
                    0
                }
            }
            Node::Conjunction { input_states, .. } => input_states
                .iter()
                .enumerate()
                .fold(0, |acc, (i, (_, state))| {
                    acc | if *state { 1 << i } else { 0 }
                }),
            Node::Broadcast { .. } => 0,
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
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(238920142622879, output);
}

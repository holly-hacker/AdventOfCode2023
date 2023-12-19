use std::ops::RangeInclusive;

use ahash::AHashMap;

use crate::utils::fast_parse_int;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 19;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let (workflows, ratings) = input.split_once("\n\n").unwrap();

        let workflows = workflows
            .split('\n')
            .map(|l| {
                let (name, rest) = l.split_once('{').unwrap();
                let mut rules = rest.trim_end_matches('}').split(',').collect::<Vec<_>>();
                let fallback_rule = match rules.pop() {
                    None => panic!(),
                    Some("A") => Output::Accepted,
                    Some("R") => Output::Rejected,
                    Some(r) => Output::Rule(r.to_string()),
                };
                let rules = rules
                    .iter()
                    .map(|r| {
                        let input = match &r[0..1] {
                            "x" => Input::X,
                            "m" => Input::M,
                            "a" => Input::A,
                            "s" => Input::S,
                            _ => panic!(),
                        };
                        let is_less_than = r.contains('<');

                        let (operand, output) = r[2..].split_once(':').unwrap();

                        Rule {
                            input,
                            is_less_than,
                            value: fast_parse_int(operand),
                            output: match output {
                                "A" => Output::Accepted,
                                "R" => Output::Rejected,
                                r => Output::Rule(r.to_string()),
                            },
                        }
                    })
                    .collect::<Vec<_>>();

                (
                    name.to_string(),
                    Workflow {
                        rules,
                        fallback_rule,
                    },
                )
            })
            .collect::<AHashMap<_, _>>();

        let inputs = ratings
            .split('\n')
            .map(|r| {
                let mut inputs = r[1..][..r.len() - 2]
                    .split(',')
                    .map(|s| s.split_once('=').unwrap().1)
                    .map(fast_parse_int);

                (
                    inputs.next().unwrap(),
                    inputs.next().unwrap(),
                    inputs.next().unwrap(),
                    inputs.next().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        inputs
            .iter()
            .filter(|input| {
                let mut workflow_name = "in";
                loop {
                    let workflow = &workflows[workflow_name];
                    let mut matching_rule = None;
                    for r in &workflow.rules {
                        let value = match r.input {
                            Input::X => input.0,
                            Input::M => input.1,
                            Input::A => input.2,
                            Input::S => input.3,
                        };
                        if (r.is_less_than && value < r.value)
                            || (!r.is_less_than && value > r.value)
                        {
                            matching_rule = Some(&r.output);
                            break;
                        }
                    }
                    let matching_rule = match matching_rule {
                        Some(x) => x,
                        None => &workflow.fallback_rule,
                    };
                    match matching_rule {
                        Output::Accepted => return true,
                        Output::Rejected => return false,
                        Output::Rule(r) => {
                            workflow_name = r;
                        }
                    }
                }
            })
            .map(|i| i.0 + i.1 + i.2 + i.3)
            .sum()
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let workflows = input
            .split_once("\n\n")
            .unwrap()
            .0
            .split('\n')
            .map(|l| {
                let (name, rest) = l.split_once('{').unwrap();
                let mut rules = rest.trim_end_matches('}').split(',').collect::<Vec<_>>();
                let fallback_rule = match rules.pop() {
                    None => panic!(),
                    Some("A") => Output::Accepted,
                    Some("R") => Output::Rejected,
                    Some(r) => Output::Rule(r.to_string()),
                };
                let rules = rules
                    .iter()
                    .map(|r| {
                        let input = match &r[0..1] {
                            "x" => Input::X,
                            "m" => Input::M,
                            "a" => Input::A,
                            "s" => Input::S,
                            _ => panic!(),
                        };
                        let is_less_than = r.contains('<');

                        let (operand, output) = r[2..].split_once(':').unwrap();

                        Rule {
                            input,
                            is_less_than,
                            value: fast_parse_int(operand),
                            output: match output {
                                "A" => Output::Accepted,
                                "R" => Output::Rejected,
                                r => Output::Rule(r.to_string()),
                            },
                        }
                    })
                    .collect::<Vec<_>>();

                (
                    name.to_string(),
                    Workflow {
                        rules,
                        fallback_rule,
                    },
                )
            })
            .collect::<AHashMap<_, _>>();

        // recursively go down.
        // pass ranges?
        // within a workflow, you can still branch. need a function for a single step?
        // may not need cache, tbd.
        let range = [1..=4000, 1..=4000, 1..=4000, 1..=4000];

        do_gold_math(&workflows, "in", range)
    }
}

fn do_gold_math(
    workflows: &AHashMap<String, Workflow>,
    start: &str,
    mut ranges: [RangeInclusive<usize>; 4],
) -> usize {
    let workflow = &workflows[start];

    let mut acc = 0;

    for r in &workflow.rules {
        let index = match r.input {
            Input::X => 0,
            Input::M => 1,
            Input::A => 2,
            Input::S => 3,
        };
        let (range_split, new_range) = if r.is_less_than {
            (
                *ranges[index].start()..=(r.value - 1),
                r.value..=*ranges[index].end(),
            )
        } else {
            (
                (r.value + 1)..=*ranges[index].end(),
                *ranges[index].start()..=r.value,
            )
        };

        let mut ranges_split = ranges.clone();
        ranges_split[index] = range_split.clone();

        ranges[index] = new_range;

        match &r.output {
            Output::Accepted => {
                acc += (ranges_split[0].end() - ranges_split[0].start() + 1)
                    * (ranges_split[1].end() - ranges_split[1].start() + 1)
                    * (ranges_split[2].end() - ranges_split[2].start() + 1)
                    * (ranges_split[3].end() - ranges_split[3].start() + 1);
            }
            Output::Rejected => {
                acc += 0;
            }
            Output::Rule(r_output) => {
                acc += do_gold_math(workflows, r_output, ranges_split);
            }
        }
    }

    // match final rule
    let final_rule = &workflow.fallback_rule;
    match final_rule {
        Output::Accepted => {
            acc += (ranges[0].end() - ranges[0].start() + 1)
                * (ranges[1].end() - ranges[1].start() + 1)
                * (ranges[2].end() - ranges[2].start() + 1)
                * (ranges[3].end() - ranges[3].start() + 1);
        }
        Output::Rejected => {
            acc += 0;
        }
        Output::Rule(r_output) => {
            acc += do_gold_math(workflows, r_output, ranges);
        }
    }

    acc
}

#[derive(Debug)]
pub struct Workflow {
    rules: Vec<Rule>,
    fallback_rule: Output,
}

#[derive(Debug)]
pub struct Rule {
    input: Input,
    is_less_than: bool,
    value: usize,
    output: Output,
}

#[derive(Debug)]
pub enum Input {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Accepted,
    Rejected,
    Rule(String),
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(19114, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(386787, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(167409079868000, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(131029523269531, output);
}

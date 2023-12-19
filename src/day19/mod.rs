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
                let (rules, fallback_rule) = rest.rsplit_once(',').unwrap();
                let rules = rules
                    .split(',')
                    .map(|r| {
                        let (operand, output) = r[2..].split_once(':').unwrap();

                        Rule {
                            input: match &r.as_bytes()[0] {
                                b'x' => Input::X,
                                b'm' => Input::M,
                                b'a' => Input::A,
                                b's' => Input::S,
                                _ => panic!(),
                            },
                            is_less_than: r.as_bytes()[1] == b'<',
                            operand: fast_parse_int(operand) as u16,
                            output: match output {
                                "A" => Output::Accepted,
                                "R" => Output::Rejected,
                                r => Output::Rule(r),
                            },
                        }
                    })
                    .collect::<Vec<_>>();

                let fallback = match fallback_rule.trim_end_matches('}') {
                    "A" => Output::Accepted,
                    "R" => Output::Rejected,
                    r => Output::Rule(r),
                };

                (name, Workflow { rules, fallback })
            })
            .collect::<AHashMap<_, _>>();

        ratings
            .split('\n')
            .map(|r| {
                let mut inputs = r[1..r.len() - 1]
                    .split(',')
                    .map(|s| s.split_once('=').unwrap().1)
                    .map(fast_parse_int);

                (
                    inputs.next().unwrap() as u16,
                    inputs.next().unwrap() as u16,
                    inputs.next().unwrap() as u16,
                    inputs.next().unwrap() as u16,
                )
            })
            .filter(|input| {
                let mut workflow_name = "in";
                loop {
                    let workflow = &workflows[workflow_name];

                    let next_rule = workflow
                        .rules
                        .iter()
                        .find_map(|r| {
                            let input = match r.input {
                                Input::X => input.0,
                                Input::M => input.1,
                                Input::A => input.2,
                                Input::S => input.3,
                            };

                            if (r.is_less_than && input < r.operand)
                                || (!r.is_less_than && input > r.operand)
                            {
                                Some(&r.output)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(&workflow.fallback);

                    match next_rule {
                        Output::Accepted => return true,
                        Output::Rejected => return false,
                        Output::Rule(r) => {
                            workflow_name = r;
                        }
                    }
                }
            })
            .map(|i| (i.0 + i.1 + i.2 + i.3) as usize)
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
                let (rules, fallback_rule) = rest.rsplit_once(',').unwrap();
                let rules = rules
                    .split(',')
                    .map(|r| {
                        let (operand, output) = r[2..].split_once(':').unwrap();

                        Rule {
                            input: match &r.as_bytes()[0] {
                                b'x' => Input::X,
                                b'm' => Input::M,
                                b'a' => Input::A,
                                b's' => Input::S,
                                _ => panic!(),
                            },
                            is_less_than: r.as_bytes()[1] == b'<',
                            operand: fast_parse_int(operand) as u16,
                            output: match output {
                                "A" => Output::Accepted,
                                "R" => Output::Rejected,
                                r => Output::Rule(r),
                            },
                        }
                    })
                    .collect::<Vec<_>>();

                let fallback = match fallback_rule.trim_end_matches('}') {
                    "A" => Output::Accepted,
                    "R" => Output::Rejected,
                    r => Output::Rule(r),
                };

                (name, Workflow { rules, fallback })
            })
            .collect::<AHashMap<_, _>>();

        let range = [1..=4000, 1..=4000, 1..=4000, 1..=4000];

        do_gold_math(&workflows, "in", range)
    }
}

fn do_gold_math(
    workflows: &AHashMap<&str, Workflow>,
    start: &str,
    ranges: [RangeInclusive<u16>; 4],
) -> usize {
    let workflow = &workflows[start];

    let (acc, ranges) = workflow
        .rules
        .iter()
        .fold((0usize, ranges), |(acc, mut ranges), rule| {
            let index = match rule.input {
                Input::X => 0,
                Input::M => 1,
                Input::A => 2,
                Input::S => 3,
            };
            let (true_range, false_range) = if rule.is_less_than {
                (
                    *ranges[index].start()..=(rule.operand - 1),
                    rule.operand..=*ranges[index].end(),
                )
            } else {
                (
                    (rule.operand + 1)..=*ranges[index].end(),
                    *ranges[index].start()..=rule.operand,
                )
            };

            let mut r2 = ranges.clone();
            r2[index] = true_range.clone();

            let new_acc = match &rule.output {
                Output::Accepted => r2[0].len() * r2[1].len() * r2[2].len() * r2[3].len(),
                Output::Rejected => 0,
                Output::Rule(r_output) => do_gold_math(workflows, r_output, r2),
            };

            ranges[index] = false_range;
            (acc + new_acc, ranges)
        });

    acc + match &workflow.fallback {
        Output::Accepted => ranges[0].len() * ranges[1].len() * ranges[2].len() * ranges[3].len(),
        Output::Rejected => 0,
        Output::Rule(r_output) => do_gold_math(workflows, r_output, ranges),
    }
}

#[derive(Debug)]
pub struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    fallback: Output<'a>,
}

#[derive(Debug)]
pub struct Rule<'a> {
    input: Input,
    is_less_than: bool,
    operand: u16,
    output: Output<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum Input {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output<'a> {
    Accepted,
    Rejected,
    Rule(&'a str),
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

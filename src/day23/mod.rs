use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, VecDeque},
};

use ahash::{AHashMap, AHashSet};
use petgraph::prelude::*;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 23;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();

        // find the longest path from (1,0) to end

        let start = (1, 0);
        let end = (width - 2, height - 1);

        let mut longest_path = 0;
        let mut queue = VecDeque::new();
        queue.push_back((start, 0, (0, 1)));
        while let Some(next) = queue.pop_front() {
            let ((x, y), steps, direction) = next;
            if (x, y) == end {
                longest_path = longest_path.max(steps);
                continue;
            }

            let next_steps = steps + 1;
            if direction != (1, 0) && x > 0 && matches!(grid[y][x - 1], '.' | '<') {
                queue.push_back(((x - 1, y), next_steps, (-1, 0)));
            }
            if direction != (-1, 0) && x < width - 1 && matches!(grid[y][x + 1], '.' | '>') {
                queue.push_back(((x + 1, y), next_steps, (1, 0)));
            }
            if direction != (0, 1) && y > 0 && matches!(grid[y - 1][x], '.' | '^') {
                queue.push_back(((x, y - 1), next_steps, (0, -1)));
            }
            if direction != (0, -1) && y < height - 1 && matches!(grid[y + 1][x], '.' | 'v') {
                queue.push_back(((x, y + 1), next_steps, (0, 1)));
            }
        }

        longest_path
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        let grid = input
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();

        // create graph
        let mut graph = UnGraph::<(usize, usize), usize>::default();

        // find the longest path from (1,0) to end

        let start = (1, 0);
        let end = (width - 2, height - 1);

        let first_node = graph.add_node(start);
        let end_node = graph.add_node(end);

        let mut node_lookup = AHashMap::new();
        node_lookup.insert(start, first_node);
        node_lookup.insert(end, end_node);

        // search the graph for all intersections
        {
            let mut queue = VecDeque::new();
            queue.push_back((first_node, (0isize, 1isize)));
            while let Some(next) = queue.pop_front() {
                let (node, mut direction) = next;
                let (start_x, start_y) = *graph.node_weight(node).unwrap();
                let mut x = ((start_x as isize) + direction.0) as usize;
                let mut y = ((start_y as isize) + direction.1) as usize;

                // find the next intersection
                let mut steps = 0;
                loop {
                    let mut left = false;
                    let mut right = false;
                    let mut up = false;
                    let mut down = false;
                    if direction != (1, 0) && x > 0 && !matches!(grid[y][x - 1], '#') {
                        left = true;
                    }
                    if direction != (-1, 0) && x < width - 1 && !matches!(grid[y][x + 1], '#') {
                        right = true;
                    }
                    if direction != (0, 1) && y > 0 && !matches!(grid[y - 1][x], '#') {
                        up = true;
                    }
                    if direction != (0, -1) && y < height - 1 && !matches!(grid[y + 1][x], '#') {
                        down = true;
                    }

                    let neighbours = [left, right, up, down]
                        .map(|x| x as usize)
                        .into_iter()
                        .sum::<usize>();

                    if neighbours == 1 {
                        // no intersection
                        direction = if left {
                            (-1, 0)
                        } else if right {
                            (1, 0)
                        } else if up {
                            (0, -1)
                        } else if down {
                            (0, 1)
                        } else {
                            panic!("no direction found");
                        };
                        x = ((x as isize) + direction.0) as usize;
                        y = ((y as isize) + direction.1) as usize;
                        steps += 1;
                        continue;
                    } else {
                        // intersection
                        break;
                    }
                }

                // we have 2 neighbours, this is an intersection

                let intersection = (x, y);
                let mut newly_inserted_node = false;
                let next_node = node_lookup.get(&intersection).copied().unwrap_or_else(|| {
                    newly_inserted_node = true;
                    let node = graph.add_node(intersection);
                    node_lookup.insert(intersection, node);
                    node
                });
                if graph.find_edge(node, next_node).is_none() {
                    graph.add_edge(node, next_node, steps + 1);
                }

                if intersection == end {
                    continue;
                }

                // queue next intersection search, IFF we haven't visited it before
                if newly_inserted_node {
                    if direction != (1, 0) && x > 0 && !matches!(grid[y][x - 1], '#') {
                        queue.push_back((next_node, (-1, 0)));
                    }
                    if direction != (-1, 0) && x < width - 1 && !matches!(grid[y][x + 1], '#') {
                        queue.push_back((next_node, (1, 0)));
                    }
                    if direction != (0, 1) && y > 0 && !matches!(grid[y - 1][x], '#') {
                        queue.push_back((next_node, (0, -1)));
                    }
                    if direction != (0, -1) && y < height - 1 && !matches!(grid[y + 1][x], '#') {
                        queue.push_back((next_node, (0, 1)));
                    }
                }
            }
        }

        // println!("Graph created: {graph:?}");
        // println!("{:?}", petgraph::dot::Dot::with_config(&graph, &[]));

        // find the longest path from start to end
        let mut queue = BinaryHeap::new();
        let first_item = QueueItem {
            position: first_node,
            steps: 0,
            visited: AHashSet::from([first_node]),
        };
        queue.push(first_item);

        let mut max_steps = 0;
        while let Some(QueueItem {
            position,
            steps,
            visited,
        }) = queue.pop()
        {
            let neighbors = graph.neighbors(position);

            for neighbor in neighbors {
                let mut visited = visited.clone();
                if !visited.insert(neighbor) {
                    continue;
                }

                let edge = graph.find_edge(position, neighbor).unwrap();
                let edge_weight = graph.edge_weight(edge).unwrap();

                let steps = steps + edge_weight;

                if neighbor == end_node {
                    if max_steps < steps {
                        println!("new max steps: {}", steps);
                        // dbg!(visited);
                        max_steps = steps;
                    }
                    continue;
                }

                queue.push(QueueItem {
                    position: neighbor,
                    steps,
                    visited,
                });
            }
        }

        max_steps
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct QueueItem {
    position: NodeIndex<petgraph::graph::DefaultIx>,
    steps: usize,
    visited: AHashSet<NodeIndex<petgraph::graph::DefaultIx>>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(94, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(2186, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(154, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_ne!(6222, output);
    assert_ne!(5162, output);
    assert_ne!(6318, output);
    assert_ne!(6490, output);
}

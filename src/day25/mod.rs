use ahash::AHashMap;
use petgraph::{algo, prelude::*};

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 25;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let mut graph = UnGraph::<&str, ()>::default();
        let mut graph_nodes = AHashMap::new();

        // parse input
        input.lines().for_each(|line| {
            let (wire, targets) = line.split_once(": ").unwrap();

            let wire_idx = *graph_nodes
                .entry(wire)
                .or_insert_with(|| graph.add_node(wire));

            for target in targets.split(' ') {
                let target_idx = *graph_nodes
                    .entry(target)
                    .or_insert_with(|| graph.add_node(target));

                graph.add_edge(wire_idx, target_idx, ());
            }
        });

        // for each node, find the max distance to any other node
        let max_distances = graph_nodes
            .iter()
            .map(|(node, node_id)| {
                let x = algo::dijkstra(&graph, *node_id, None, |_| 1);
                (*node, *x.values().max().unwrap())
            })
            .collect::<Vec<_>>();

        // find the nodes that have the lowest max distance. these are likely to connect the 2
        // subgraphs together
        let lowest_max_distance = max_distances.iter().map(|(_, v)| *v).min().unwrap();
        let lowest_max_distance = max_distances
            .into_iter()
            .filter(|(_, v)| *v == lowest_max_distance)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        // for all the candidates we found, get their neighbours
        let to_loop = lowest_max_distance
            .into_iter()
            .map(|k| {
                (
                    graph_nodes[k],
                    graph.neighbors(graph_nodes[k]).collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<(_, _)>>();

        // take a combination of 3 candidates, ...
        for i in 0..to_loop.len() {
            for j in (i + 1)..to_loop.len() {
                for k in (j + 1)..to_loop.len() {
                    let (node_i_idx, node_i_targets) = &to_loop[i];
                    let (node_j_idx, node_j_targets) = &to_loop[j];
                    let (node_k_idx, node_k_targets) = &to_loop[k];

                    // ... remove an edge for each, ...
                    for i_target in node_i_targets {
                        let i_edge = graph.find_edge(*node_i_idx, *i_target);
                        debug_assert!(i_edge.is_some());
                        graph.remove_edge(i_edge.unwrap());

                        for j_target in node_j_targets {
                            let Some(j_edge) = graph.find_edge(*node_j_idx, *j_target) else {
                                continue;
                            };
                            graph.remove_edge(j_edge);

                            for k_target in node_k_targets {
                                let Some(k_edge) = graph.find_edge(*node_k_idx, *k_target) else {
                                    continue;
                                };
                                graph.remove_edge(k_edge);

                                // ... and see if we now have 2 subgraphs
                                if algo::connected_components(&graph) == 2 {
                                    let group_1_size =
                                        algo::dijkstra(&graph, *node_i_idx, None, |_| 1).len();
                                    let group_2_size = graph.node_count() - group_1_size;
                                    return group_1_size * group_2_size;
                                }

                                graph.add_edge(*k_target, *node_k_idx, ());
                            }
                            graph.add_edge(*j_target, *node_j_idx, ());
                        }
                        graph.add_edge(*i_target, *node_i_idx, ());
                    }
                }
            }
        }

        unreachable!("no solution found")
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(54, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(550080, output);
}

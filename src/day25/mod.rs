use ahash::AHashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 25;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        let input = input
            .lines()
            .map(|l| {
                let (wire, targets) = l.split_once(": ").unwrap();
                let targets = targets.split(' ').collect::<Vec<_>>();
                (wire, targets)
            })
            .collect::<Vec<_>>();

        let mut graph_nodes = AHashMap::new();
        let mut graph = petgraph::graph::UnGraph::<&str, ()>::new_undirected();

        // add nodes
        for line in &input {
            let (wire, targets) = line;
            if !graph_nodes.contains_key(wire) {
                graph_nodes.insert(wire, graph.add_node(wire));
            }

            for target in targets {
                if !graph_nodes.contains_key(target) {
                    graph_nodes.insert(target, graph.add_node(target));
                }
            }
        }

        // add edges
        for line in &input {
            let (wire, targets) = line;
            for target in targets {
                let target_idx = graph_nodes.get(target).unwrap();
                graph.add_edge(*graph_nodes.get(wire).unwrap(), *target_idx, ());
            }
        }

        let max_distances = graph_nodes
            .iter()
            .map(|(node, node_id)| {
                let x = petgraph::algo::dijkstra(&graph, *node_id, None, |_| 1);
                (node, *x.values().max().unwrap())
            })
            .collect::<Vec<_>>();

        let lowest_max_distance = max_distances.iter().map(|(_, v)| v).min().unwrap();
        let lowest_max_distance = max_distances
            .iter()
            .filter(|(_, v)| v == lowest_max_distance)
            .map(|(k, _)| k)
            .collect::<Vec<_>>();

        // dbg!(&lowest_max_distance);

        let to_loop = lowest_max_distance
            .into_iter()
            .map(|k| {
                (
                    graph_nodes[*k],
                    graph.neighbors(graph_nodes[*k]).collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<(_, _)>>();

        // dbg!(&to_loop);

        // let group_lens = to_loop.iter().enumerate().flat_map(|(index, item)| {
        //     // todo
        // });

        for i in (0..to_loop.len()).rev() {
            for j in ((i + 1)..to_loop.len()).rev() {
                for k in ((j + 1)..to_loop.len()).rev() {
                    let (node_i_idx, node_i_targets) = &to_loop[i];
                    let (node_j_idx, node_j_targets) = &to_loop[j];
                    let (node_k_idx, node_k_targets) = &to_loop[k];

                    for i_target in node_i_targets {
                        let i_edge = graph.find_edge(*node_i_idx, *i_target);
                        debug_assert!(i_edge.is_some());
                        graph.remove_edge(i_edge.unwrap());

                        for j_target in node_j_targets {
                            let j_edge = graph.find_edge(*node_j_idx, *j_target);
                            let Some(j_edge) = j_edge else {
                                continue;
                            };
                            graph.remove_edge(j_edge);
                            for k_target in node_k_targets {
                                let k_edge = graph.find_edge(*node_k_idx, *k_target);

                                let Some(k_edge) = k_edge else {
                                    continue;
                                };
                                graph.remove_edge(k_edge);

                                if petgraph::algo::connected_components(&graph) == 2 {
                                    // println!("Found a loop!");
                                    // println!(
                                    //     "Found a loop, removed {}-{}, {}-{}, {}-{}",
                                    //     graph.node_weight(*node_i_idx).unwrap(),
                                    //     graph.node_weight(*i_target).unwrap(),
                                    //     graph.node_weight(*node_j_idx).unwrap(),
                                    //     graph.node_weight(*j_target).unwrap(),
                                    //     graph.node_weight(*node_k_idx).unwrap(),
                                    //     graph.node_weight(*k_target).unwrap(),
                                    // );

                                    // kinda lazy lol
                                    let len =
                                        petgraph::algo::dijkstra(&graph, *node_i_idx, None, |_| 1)
                                            .len();
                                    return len * (graph.node_count() - len);
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

        0 // todo!()
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

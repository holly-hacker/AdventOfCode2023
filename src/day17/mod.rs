use std::collections::BinaryHeap;

use ahash::AHashMap;

use super::*;

pub struct Day;

impl SolutionSilver<usize> for Day {
    const DAY: u32 = 17;
    const INPUT_SAMPLE: &'static str = include_str!("input_sample.txt");
    const INPUT_REAL: &'static str = include_str!("input_real.txt");

    fn calculate_silver(input: &str) -> usize {
        const MAX_FLOW: usize = 3;

        let grid = input.as_bytes().split(|b| *b == b'\n').collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();

        let mut heap = BinaryHeap::new();
        heap.push(PositionWithDirection {
            position: (1, 0),
            direction: (1, 0),
            flow_count: 1,
            heat_loss: grid[0][1] as usize - b'0' as usize,
        });
        heap.push(PositionWithDirection {
            position: (0, 1),
            direction: (0, 1),
            flow_count: 1,
            heat_loss: grid[1][0] as usize - b'0' as usize,
        });

        let mut visited = AHashMap::new();

        let mut best_result = usize::MAX;
        while let Some(pos) = heap.pop() {
            if pos.heat_loss >= best_result {
                continue;
            }

            if let Some(visited_heat_loss) =
                visited.get(&(pos.position, pos.direction, pos.flow_count))
            {
                if *visited_heat_loss <= pos.heat_loss {
                    continue;
                }
            }

            visited.insert((pos.position, pos.direction, pos.flow_count), pos.heat_loss);

            if pos.position.0 == width - 1 && pos.position.1 == height - 1 {
                // found the end!
                best_result = std::cmp::min(best_result, pos.heat_loss);
                continue;
            }

            if pos.flow_count < MAX_FLOW {
                let next_x = pos.position.0 as isize + pos.direction.0;
                let next_y = pos.position.1 as isize + pos.direction.1;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < width
                    && (next_y as usize) < height
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    let next_pos = PositionWithDirection {
                        position: (next_x, next_y),
                        direction: pos.direction,
                        flow_count: pos.flow_count + 1,
                        heat_loss: pos.heat_loss + grid[next_y][next_x] as usize - b'0' as usize,
                    };
                    heap.push(next_pos);
                }
            }

            // split left and right
            // (0, 1) -> (-1, 0), (1, 0)
            // (1, 0) -> (0, -1), (0, 1)
            let left = (-pos.direction.1, pos.direction.0);
            let right = (pos.direction.1, -pos.direction.0);

            // left
            {
                let next_x = pos.position.0 as isize + left.0;
                let next_y = pos.position.1 as isize + left.1;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < width
                    && (next_y as usize) < height
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    let next_pos = PositionWithDirection {
                        position: (next_x, next_y),
                        direction: left,
                        flow_count: 1,
                        heat_loss: pos.heat_loss + grid[next_y][next_x] as usize - b'0' as usize,
                    };
                    heap.push(next_pos);
                }
            }

            // right
            {
                let next_x = pos.position.0 as isize + right.0;
                let next_y = pos.position.1 as isize + right.1;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < width
                    && (next_y as usize) < height
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    let next_pos = PositionWithDirection {
                        position: (next_x, next_y),
                        direction: right,
                        flow_count: 1,
                        heat_loss: pos.heat_loss + grid[next_y][next_x] as usize - b'0' as usize,
                    };
                    heap.push(next_pos);
                }
            }
        }

        best_result
    }
}

impl SolutionGold<usize, usize> for Day {
    fn calculate_gold(input: &str) -> usize {
        const MIN_FLOW: usize = 4;
        const MAX_FLOW: usize = 10;

        let grid = input.as_bytes().split(|b| *b == b'\n').collect::<Vec<_>>();
        let width = grid[0].len();
        let height = grid.len();

        let mut heap = BinaryHeap::new();
        heap.push(PositionWithDirection {
            position: (1, 0),
            direction: (1, 0),
            flow_count: 1,
            heat_loss: grid[0][1] as usize - b'0' as usize,
        });
        heap.push(PositionWithDirection {
            position: (0, 1),
            direction: (0, 1),
            flow_count: 1,
            heat_loss: grid[1][0] as usize - b'0' as usize,
        });

        let mut visited = AHashMap::new();

        let mut best_result = usize::MAX;
        while let Some(pos) = heap.pop() {
            if pos.heat_loss >= best_result {
                continue;
            }

            if let Some(visited_heat_loss) =
                visited.get(&(pos.position, pos.direction, pos.flow_count))
            {
                if *visited_heat_loss <= pos.heat_loss {
                    continue;
                }
            }

            visited.insert((pos.position, pos.direction, pos.flow_count), pos.heat_loss);

            if pos.position.0 == width - 1 && pos.position.1 == height - 1 {
                // found the end!
                best_result = std::cmp::min(best_result, pos.heat_loss);
                continue;
            }

            // go straight
            if pos.flow_count < MAX_FLOW {
                let next_x = pos.position.0 as isize + pos.direction.0;
                let next_y = pos.position.1 as isize + pos.direction.1;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < width
                    && (next_y as usize) < height
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    let next_pos = PositionWithDirection {
                        position: (next_x, next_y),
                        direction: pos.direction,
                        flow_count: pos.flow_count + 1,
                        heat_loss: pos.heat_loss + grid[next_y][next_x] as usize - b'0' as usize,
                    };
                    heap.push(next_pos);
                }
            }

            // split left and right
            // (0, 1) -> (-1, 0), (1, 0)
            // (1, 0) -> (0, -1), (0, 1)
            let left = (-pos.direction.1, pos.direction.0);
            let right = (pos.direction.1, -pos.direction.0);

            // left
            if pos.flow_count >= MIN_FLOW {
                let next_x = pos.position.0 as isize + left.0;
                let next_y = pos.position.1 as isize + left.1;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < width
                    && (next_y as usize) < height
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    let next_pos = PositionWithDirection {
                        position: (next_x, next_y),
                        direction: left,
                        flow_count: 1,
                        heat_loss: pos.heat_loss + grid[next_y][next_x] as usize - b'0' as usize,
                    };
                    heap.push(next_pos);
                }
            }

            // right
            if pos.flow_count >= MIN_FLOW {
                let next_x = pos.position.0 as isize + right.0;
                let next_y = pos.position.1 as isize + right.1;
                if next_x >= 0
                    && next_y >= 0
                    && (next_x as usize) < width
                    && (next_y as usize) < height
                {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
                    let next_pos = PositionWithDirection {
                        position: (next_x, next_y),
                        direction: right,
                        flow_count: 1,
                        heat_loss: pos.heat_loss + grid[next_y][next_x] as usize - b'0' as usize,
                    };
                    heap.push(next_pos);
                }
            }
        }

        best_result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionWithDirection {
    position: (usize, usize),
    direction: (isize, isize),
    flow_count: usize,
    heat_loss: usize,
}

impl PartialOrd for PositionWithDirection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PositionWithDirection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

#[test]
fn test_silver_sample() {
    let output = Day::calculate_silver(Day::INPUT_SAMPLE);
    assert_eq!(102, output);
}

#[test]
fn test_silver_real() {
    let output = Day::calculate_silver(Day::INPUT_REAL);
    assert_eq!(668, output);
}

#[test]
fn test_gold_sample() {
    let output = Day::calculate_gold(Day::INPUT_SAMPLE_GOLD);
    assert_eq!(94, output);
}

#[test]
fn test_gold_real() {
    let output = Day::calculate_gold(Day::INPUT_REAL);
    assert_eq!(788, output);
}

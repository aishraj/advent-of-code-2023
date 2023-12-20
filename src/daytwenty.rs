use std::collections::{BTreeMap, VecDeque};
use std::str::FromStr;

use itertools::Itertools;

/// Based on my reading of other people's solutions on Reddit
/// My original solution was a mess of BTreeMaps and Vecs

struct Circuit {
    graph: BTreeMap<String, Vec<String>>,
    flops: BTreeMap<String, bool>,
    conjs: BTreeMap<String, BTreeMap<String, bool>>,
}

impl FromStr for Circuit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (flops, mut conjs, graph): (BTreeMap<_, _>, BTreeMap<_, _>, BTreeMap<_, _>) = s
            .lines()
            .map(|line| {
                let parts = line.split("->").map(|s| s.trim().to_string()).collect_vec();
                let mut source = parts.first().unwrap().clone();
                let dests: Vec<String> =
                    parts[1].split(',').map(|s| s.trim().to_string()).collect();

                let is_flop = if source.starts_with('%') {
                    source.remove(0);
                    true
                } else {
                    false
                };

                let is_conj = if source.starts_with('&') {
                    source.remove(0);
                    true
                } else {
                    false
                };

                (source, dests, is_flop, is_conj)
            })
            .fold(
                (BTreeMap::new(), BTreeMap::new(), BTreeMap::new()),
                |(mut flops, mut conjs, mut graph), (source, dests, is_flop, is_conj)| {
                    if is_flop {
                        flops.insert(source.clone(), false);
                    }
                    if is_conj {
                        conjs.insert(source.clone(), BTreeMap::new());
                    }
                    graph.insert(source, dests);
                    (flops, conjs, graph)
                },
            );

        for (source, dests) in &graph {
            for dest in dests {
                if conjs.contains_key(dest) {
                    conjs.get_mut(dest).unwrap().insert(source.clone(), false);
                }
            }
        }

        Ok(Circuit {
            graph,
            flops,
            conjs,
        })
    }
}

impl Circuit {
    fn propagate_pulse(
        &mut self,
        sender: &str,
        receiver: &str,
        pulse: bool,
    ) -> Vec<(String, String, bool)> {
        let mut next_pulse = pulse;

        if let Some(flip_flop) = self.flops.get_mut(receiver) {
            if pulse {
                return vec![];
            }
            *flip_flop = !*flip_flop;
            next_pulse = *flip_flop;
        } else if let Some(conjunction) = self.conjs.get_mut(receiver) {
            conjunction.insert(sender.to_string(), pulse);
            next_pulse = !conjunction.values().all(|&x| x);
        } else if !self.graph.contains_key(receiver) {
            return vec![];
        }

        let default_receiver = vec![];
        let receivers = self.graph.get(receiver).unwrap_or(&default_receiver);

        receivers
            .iter()
            .map(|new_receiver| (receiver.to_string(), new_receiver.clone(), next_pulse))
            .collect()
    }

    fn simulate(&mut self, start_node: String, connection: String) -> (i32, i32) {
        let mut queue = VecDeque::new();
        queue.push_back((start_node, connection, false));

        let (mut low_pulse_count, mut high_pulse_count) = (0, 0);

        while let Some((sender, receiver, pulse)) = queue.pop_front() {
            if pulse {
                high_pulse_count += 1;
            } else {
                low_pulse_count += 1;
            }

            let new_nodes = self.propagate_pulse(&sender, &receiver, pulse);
            queue.extend(new_nodes);
        }

        (low_pulse_count, high_pulse_count)
    }
}

pub fn solve_part_one(input: &str) -> i64 {
    let mut circuit = Circuit::from_str(input).unwrap();
    let (mut total_high_pulse_count, mut total_low_pulse_count) = (0, 0);
    for _ in 0..1000 {
        let (high_pulse_count, low_pulse_count) =
            circuit.simulate("button".to_string(), "broadcaster".to_string());
        total_high_pulse_count += high_pulse_count;
        total_low_pulse_count += low_pulse_count;
    }

    (total_low_pulse_count * total_high_pulse_count).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20_1_real() {
        let data = std::fs::read_to_string("input/20_real.txt").unwrap();
        assert_eq!(solve_part_one(&data), 787056720);
    }
}

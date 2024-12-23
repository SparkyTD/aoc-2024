use std::fmt::{Display};
use ahash::{AHashMap, AHashSet};
use crate::utils::solution::{solution, Solution};

#[derive(Default)]
pub struct LanParty;

type Graph = AHashMap<String, AHashSet<String>>;

impl Solution for LanParty {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut graph: Graph = Graph::new();
        let mut vertices = AHashSet::new();
        for mut line in input.lines().map(|l| l.split('-')) {
            let left = line.next().unwrap();
            let right = line.next().unwrap();

            graph.entry(left.to_string()).or_insert(AHashSet::new()).insert(right.to_string());
            graph.entry(right.to_string()).or_insert(AHashSet::new()).insert(left.to_string());

            vertices.insert(left.to_string());
            vertices.insert(right.to_string());
        }

        // Part 1
        let mut triplets = AHashSet::new();
        for (node, connections) in &graph {
            if !node.starts_with("t") {
                continue;
            }
            for conn in connections {
                for conn2 in graph.get(conn).unwrap() {
                    if graph.get(node).unwrap().contains(conn2) {
                        let mut current_triplet = vec![node.clone(), conn.clone(), conn2.clone()];
                        current_triplet.sort();

                        let first = current_triplet.get(0).unwrap().clone();
                        let second = current_triplet.get(1).unwrap().clone();
                        let third = current_triplet.get(2).unwrap().clone();

                        triplets.insert((first, second, third));
                    }
                }
            }
        }

        // Part 2
        let mut cliques: Vec<AHashSet<String>> = Vec::new();

        let mut sorted_vertices = vertices.into_iter().collect::<Vec<_>>();
        sorted_vertices.sort();

        for vertex in sorted_vertices {
            let neighbors = graph.get(&vertex).unwrap_or(&AHashSet::new()).clone();
            let clique_to_join = cliques
                .iter_mut().find(|c| c.intersection(&neighbors).count() == c.len());
            if let Some(clique) = clique_to_join {
                clique.insert(vertex);
            } else {
                let clique = AHashSet::from([vertex]);
                cliques.push(clique);
            }
        }

        let mut max_clique_length = 0;
        let mut max_clique = None;
        for clique in cliques {
            if clique.len() > max_clique_length {
                max_clique_length = clique.len();
                max_clique = Some(clique);
            }
        }

        let mut max_clique = max_clique.unwrap().into_iter().collect::<Vec<String>>();
        max_clique.sort();

        solution!(triplets.len(), max_clique.join(","))
    }
}
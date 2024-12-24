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

        // Find all possible cliques
        for vertex in sorted_vertices {
            let neighbors = graph.get(&vertex).unwrap_or(&AHashSet::new()).clone();
            // println!("Processing vertex '{}' with neighbors {:?}", vertex.bright_purple().bold(), neighbors);

            let clique_to_join = cliques
                .iter_mut().find(|c| c.intersection(&neighbors).count() == c.len());
            if let Some(clique) = clique_to_join {
                // println!("   Will join clique: {:?}", clique);
                clique.insert(vertex);
            } else {
                // println!("   No clique to join, creating new one");
                let clique = AHashSet::from([vertex]);
                cliques.push(clique);
            }
        }

        // Loop through all cliques (big and small), check which one could
        // ingest the most neighboring nodes to become bigger
        // println!("All cliques: {:?}", cliques);
        let mut clique_extensions = AHashMap::new();
        for i in 0..cliques.len() {
            let clique = &mut cliques[i];
            let mut valid_candidates = Vec::new();
            for (node, connection) in &graph {
                if clique.contains(node) {
                    continue;
                }

                // Check if adding this node to the clique is valid
                let mut temp_clique = clique.clone();
                temp_clique.extend(valid_candidates.clone());
                if connection.intersection(&temp_clique).count() == temp_clique.len() {
                    valid_candidates.push(node.clone());
                }
            }

            clique_extensions.insert(i, valid_candidates);
        }

        let mut max_extension_length = 0;
        let mut max_extension = None;
        for (clique_index, extension) in clique_extensions {
            let clique = &cliques[clique_index];
            let new_length = clique.len() + extension.len();
            if new_length > max_extension_length {
                max_extension_length = new_length;
                max_extension = Some((clique_index, extension));
            }
        }

        let max_extension = max_extension.unwrap();
        let clique = &mut cliques[max_extension.0];
        clique.extend(max_extension.1);

        // Select a clique with the highest node count
        let mut max_clique_length = 0;
        let mut max_cliques = Vec::new();
        for clique in cliques {
            if clique.len() > max_clique_length {
                max_cliques.clear();
                max_clique_length = clique.len();
                max_cliques.clear();
                max_cliques.push(clique);
            } else if clique.len() == max_clique_length {
                max_cliques.push(clique);
            }
        }

        let mut max_cliques = max_cliques.into_iter();
        let mut max_clique = max_cliques.next().unwrap().into_iter().collect::<Vec<String>>();
        max_clique.sort();

        solution!(triplets.len(), max_clique.join(","))
    }
}
use std::collections::{HashSet};
use std::fmt::Display;
use crate::utils::matrix::Matrix;
use crate::utils::solution::{solution, Solution};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Edge {
    start: (usize, usize),
    end: (usize, usize),
}

impl Edge {
    pub fn from(start_x: usize, start_y: usize, orientation: Orientation) -> Self {
        Self {
            start: (start_x, start_y),
            end: match orientation {
                Orientation::Horizontal => (start_x + 1, start_y),
                Orientation::Vertical => (start_x, start_y + 1),
            },
        }
    }

    pub fn orientation(&self) -> Orientation {
        if self.start.0 == self.end.0 && self.start.1 != self.end.1 {
            Orientation::Vertical
        } else if self.start.1 == self.end.1 && self.start.0 != self.end.0 {
            Orientation::Horizontal
        } else {
            panic!("Invalid edge!")
        }
    }

    pub fn try_merge(&self, other: &Self) -> Option<Self> {
        if self.orientation() != other.orientation() {
            return None;
        }

        if self.end == other.start {
            Some(Self { start: self.start, end: other.end })
        } else if self.start == other.end {
            Some(Self { start: other.start, end: self.end })
        } else if self.start == other.start {
            Some(Self { start: self.end, end: other.end })
        } else if self.end == other.end {
            Some(Self { start: other.start, end: self.start })
        } else {
            None
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<(usize, usize)> {
        if self.orientation() == other.orientation() {
            return None;
        }

        if self.start == other.start || self.end == other.start || self.start == other.end || self.end == other.end {
            return None;
        }

        let self_start_x = self.start.0.min(self.end.0);
        let self_start_y = self.start.1.min(self.end.1);
        let self_end_x = self.start.0.max(self.end.0);
        let self_end_y = self.start.1.max(self.end.1);

        let other_start_x = other.start.0.min(other.end.0);
        let other_start_y = other.start.1.min(other.end.1);
        let other_end_x = other.start.0.max(other.end.0);
        let other_end_y = other.start.1.max(other.end.1);

        if self.orientation() == Orientation::Vertical && other_start_x < self_start_x && other_end_x > self_start_x {
            if other_start_y > self_start_y && other_end_y < self_end_y {
                return Some((self_start_x, other_start_y));
            }
        }

        if self.orientation() == Orientation::Horizontal && other_start_y < self_start_y && other_end_y > self_start_y {
            if other_start_x > self_start_x && other_end_x < self_end_x {
                return Some((other_start_x, self_start_y));
            }
        }

        None
    }
}

fn extract_edges(matrix: &Matrix<bool>) -> Vec<Edge> {
    let mut edges = Vec::new();

    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            let val = *matrix.get(x, y).unwrap();
            if !val {
                continue;
            }

            if x == 0 || !*matrix.get(x - 1, y).unwrap() {
                edges.push(Edge::from(x, y, Orientation::Vertical));
            }
            if x == matrix.width() - 1 || !*matrix.get(x + 1, y).unwrap() {
                edges.push(Edge::from(x + 1, y, Orientation::Vertical));
            }
            if y == 0 || !*matrix.get(x, y - 1).unwrap() {
                edges.push(Edge::from(x, y, Orientation::Horizontal));
            }
            if y == matrix.height() - 1 || !*matrix.get(x, y + 1).unwrap() {
                edges.push(Edge::from(x, y + 1, Orientation::Horizontal));
            }
        }
    }

    edges
}

fn simplify_edges(edges: &Vec<Edge>) -> usize {
    let mut edges = edges.clone();
    let mut new_edges = Vec::new();
    let mut merged_parts = HashSet::new();

    // Merge all straight loops
    loop {
        let mut has_merged_any = false;
        for i in 0..edges.len() {
            let mut edge_1 = *edges.get(i).unwrap();
            if merged_parts.contains(&edge_1.clone()) {
                continue;
            }
            for j in i + 1..edges.len() {
                let edge_2 = edges.get(j).unwrap();
                if merged_parts.contains(&edge_1.clone()) || merged_parts.contains(edge_2) {
                    continue;
                }
                if let Some(new_edge) = edge_1.try_merge(edge_2) {
                    merged_parts.insert(edge_1.clone());
                    merged_parts.insert(edge_2.clone());
                    edge_1 = new_edge;
                    has_merged_any = true;
                }
            }
            new_edges.push(edge_1);
        }

        if !has_merged_any {
            break;
        }

        edges = new_edges.clone();
        new_edges.clear();
    }

    edges = new_edges;

    // Break up intersections
    let mut intersection_count = 0;
    for i in 0..edges.len() {
        let edge_1 = *edges.get(i).unwrap();
        for j in 0..edges.len() {
            let edge_2 = edges.get(j).unwrap();
            if let Some(_) = edge_1.intersect(&edge_2) {
                intersection_count += 1;
                break;
            }
        }
    }

    edges.len() + intersection_count
}

#[derive(Default)]
pub struct GardenGroups;

impl Solution for GardenGroups {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let matrix = Matrix::<char>::from_text(input.as_str());

        let mut filled_cells: HashSet<(usize, usize)> = HashSet::new();
        let mut total_price_p1 = 0;
        let mut total_price_p2 = 0;

        for x in 0..matrix.height() {
            for y in 0..matrix.width() {
                if filled_cells.contains(&(x, y)) {
                    continue;
                }

                let mut region_area: HashSet<(usize, usize)> = HashSet::new();

                let flood_matrix = matrix.flood_eq(x, y, |x: &usize, y: &usize| {
                    region_area.insert((*x, *y));
                });

                filled_cells.extend(&region_area);

                let edges = extract_edges(&flood_matrix);
                total_price_p1 += edges.len() * region_area.len();

                let simplified_edges = simplify_edges(&edges);
                total_price_p2 += simplified_edges * region_area.len();
            }
        }

        solution!(total_price_p1, total_price_p2)
    }
}
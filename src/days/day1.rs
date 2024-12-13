use std::fmt::Display;
use crate::utils::solution::{solution, Solution};

#[derive(Debug, Clone)]
struct Node {
    value: u32,
    count: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u32) -> Node {
        Self {
            value,
            count: 1,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: u32) {
        let new_node = Box::new(Node::new(value));
        if value > self.value {
            Self::insert_over(&mut self.right, new_node)
        } else if value < self.value {
            Self::insert_over(&mut self.left, new_node)
        } else {
            self.count += 1;
        }
    }

    fn insert_over(subtree: &mut Option<Box<Node>>, node: Box<Node>) {
        if let Some(subtree) = subtree {
            subtree.insert(node.value);
        } else {
            subtree.replace(node);
        }
    }

    fn flatten_into_vec(&self, vec: &mut Vec<u32>) {
        if let Some(left_subtree) = &self.left {
            left_subtree.flatten_into_vec(vec);
        }

        for _ in 0..self.count {
            vec.push(self.value);
        }

        if let Some(right_subtree) = &self.right {
            right_subtree.flatten_into_vec(vec);
        }
    }

    fn find_count_of(&self, value: u32) -> u32 {
        if self.value == value {
            self.count
        } else if value < self.value {
            if let Some(left_subtree) = &self.left {
                left_subtree.find_count_of(value)
            } else {
                0
            }
        } else if value > self.value {
            if let Some(right_subtree) = &self.right {
                right_subtree.find_count_of(value)
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct HistorianHysteria;

impl Solution for HistorianHysteria {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut left_tree: Option<Box<Node>> = None;
        let mut right_tree: Option<Box<Node>> = None;

        // Read all values into a BST
        for line in input.lines() {
            let mut split = line.split_whitespace();
            let left = split.next().unwrap().parse::<u32>().unwrap();
            let right = split.next().unwrap().parse::<u32>().unwrap();

            for (value, &mut ref mut tree) in [left, right]
                .iter()
                .zip([&mut left_tree, &mut right_tree].iter_mut())
            {
                Node::insert_over(tree, Box::new(Node::new(*value)));
            }
        }

        // Flatten BSTs into sorted vectors
        let mut left_sorted: Vec<u32> = vec![];
        let left_tree = left_tree.unwrap();
        left_tree.flatten_into_vec(&mut left_sorted);
        left_sorted.reverse();

        let mut right_sorted: Vec<u32> = vec![];
        let right_tree = right_tree.unwrap();
        right_tree.flatten_into_vec(&mut right_sorted);
        right_sorted.reverse();

        // Sum up all the differences
        let mut total_differences = 0;
        let mut similarity_score = 0;

        assert_eq!(left_sorted.len(), right_sorted.len(), "The two lists are somehow not the same length!");

        for (left, right) in left_sorted.iter().zip(right_sorted.iter()) {
            let count_in_right = right_tree.find_count_of(*left);
            similarity_score += *left * count_in_right;
            total_differences += right.max(left) - left.min(right);
        }

        solution!(total_differences, similarity_score)
    }
}
#[derive(Debug, Clone)]
struct Node {
    value: u64,
    count: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(value: u64) -> Node {
        Self {
            value,
            count: 1,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn insert(&mut self, value: u64) {
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

    fn flatten_into_vec(&self, vec: &mut Vec<u64>) {
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
}

pub fn day_1() {
    let input = include_str!("../data/day1.txt");

    let mut left_tree: Option<Box<Node>> = None;
    let mut right_tree: Option<Box<Node>> = None;

    // Read all values into a BST
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let left = split.next().unwrap().parse::<u64>().unwrap();
        let right = split.next().unwrap().parse::<u64>().unwrap();

        for (value, &mut ref mut tree) in [left, right]
            .iter()
            .zip([&mut left_tree, &mut right_tree].iter_mut())
        {
            Node::insert_over(tree, Box::new(Node::new(*value)));
        }
    }

    // Flatten BSTs into sorted vectors
    let mut left_sorted: Vec<u64> = vec![];
    left_tree.unwrap().flatten_into_vec(&mut left_sorted);
    left_sorted.reverse();

    let mut right_sorted: Vec<u64> = vec![];
    right_tree.unwrap().flatten_into_vec(&mut right_sorted);
    right_sorted.reverse();

    // Sum up all the differences
    let mut total_differences = 0;
    assert_eq!(left_sorted.len(), right_sorted.len(), "The two lists are somehow not the same length!");

    for (mut left, mut right) in left_sorted.iter().zip(right_sorted.iter()) {
        if left > right {
            (left, right) = (right, left);
        }

        total_differences += right - left;
    }

    println!("Total differences: {}", total_differences);
}

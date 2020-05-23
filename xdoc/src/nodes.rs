use std::cmp::Ordering;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use crate::node::Node;

#[derive(Debug, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Nodes(VecDeque<Node>);

impl Clone for Nodes {
    fn clone(&self) -> Nodes {
        let mut result = VecDeque::new();
        for node in self.0.iter() {
            result.push_back(node.clone())
        }
        Nodes(result)
    }
}

impl Default for Nodes {
    fn default() -> Nodes {
        Nodes(VecDeque::new())
    }
}

impl PartialEq for Nodes {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for i in 0..self.0.len() {
            if self.0.get(i).unwrap().eq(other.0.get(i).unwrap()) {
                return false;
            }
        }
        true
    }
}

impl Nodes {
    fn size_le(&self, other: &Self) -> bool {
        self.0.len() < other.0.len()
    }

    fn size_gt(&self, other: &Self) -> bool {
        self.0.len() > other.0.len()
    }
}

impl PartialOrd for Nodes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.size_le(other) {
            return Some(Ordering::Less);
        } else if self.size_gt(other) {
            return Some(Ordering::Greater);
        }
        for i in 0..self.0.len() {
            if self.0.get(i).unwrap() < other.0.get(i).unwrap() {
                return Some(Ordering::Less);
            } else if self.0.get(i).unwrap() > other.0.get(i).unwrap() {
                return Some(Ordering::Greater);
            }
        }
        Some(Ordering::Equal)
    }

    fn lt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Less || ordering == Ordering::Equal;
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        if let Some(ordering) = self.partial_cmp(other) {
            return ordering == Ordering::Greater || ordering == Ordering::Equal;
        }
        false
    }
}

impl Hash for Nodes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for node in self.0.iter() {
            node.hash(state);
        }
    }
}

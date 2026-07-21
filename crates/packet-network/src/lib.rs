use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::graph::{NodeIndex, UnGraph};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    Core,
    Edge,
    Exchange,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub kind: NodeKind,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    pub id: String,
    pub capacity_mbps: f64,
    pub latency_ms: f64,
}

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("duplicate node id: {0}")]
    DuplicateNode(String),
    #[error("unknown node id: {0}")]
    UnknownNode(String),
    #[error("non-positive capacity: {0}")]
    NonPositiveCapacity(f64),
}

pub struct Network {
    graph: UnGraph<Node, Link>,
    index: HashMap<String, NodeIndex>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            graph: UnGraph::new_undirected(),
            index: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), NetworkError> {
        if self.index.contains_key(&node.id) {
            return Err(NetworkError::DuplicateNode(node.id));
        }
        let id = node.id.clone();
        let idx = self.graph.add_node(node);
        self.index.insert(id, idx);
        Ok(())
    }

    pub fn add_link(&mut self, from_id: &str, to_id: &str, link: Link) -> Result<(), NetworkError> {
        if link.capacity_mbps <= 0.0 {
            return Err(NetworkError::NonPositiveCapacity(link.capacity_mbps));
        }
        let from = *self
            .index
            .get(from_id)
            .ok_or_else(|| NetworkError::UnknownNode(from_id.to_string()))?;
        let to = *self
            .index
            .get(to_id)
            .ok_or_else(|| NetworkError::UnknownNode(to_id.to_string()))?;
        self.graph.add_edge(from, to, link);
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn link_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn degree(&self, id: &str) -> Option<usize> {
        self.index.get(id).map(|&idx| self.graph.edges(idx).count())
    }

    pub fn is_connected(&self, a: &str, b: &str) -> bool {
        match (self.index.get(a), self.index.get(b)) {
            (Some(&ai), Some(&bi)) => self.find_path(ai, bi, &HashSet::new()).is_some(),
            _ => false,
        }
    }

    pub fn has_diverse_path(&self, a: &str, b: &str) -> bool {
        let (ai, bi) = match (self.index.get(a), self.index.get(b)) {
            (Some(&ai), Some(&bi)) => (ai, bi),
            _ => return false,
        };
        if ai == bi {
            return false;
        }
        let path1 = match self.find_path(ai, bi, &HashSet::new()) {
            Some(path) => path,
            None => return false,
        };
        let intermediates: HashSet<NodeIndex> =
            path1.into_iter().filter(|&n| n != ai && n != bi).collect();
        self.find_path(ai, bi, &intermediates).is_some()
    }

    pub fn incident_capacity_mbps(&self, id: &str) -> f64 {
        self.index.get(id).map_or(0.0, |&idx| {
            self.graph
                .edges(idx)
                .map(|edge| edge.weight().capacity_mbps)
                .sum()
        })
    }

    fn find_path(
        &self,
        start: NodeIndex,
        goal: NodeIndex,
        excluded: &HashSet<NodeIndex>,
    ) -> Option<Vec<NodeIndex>> {
        if excluded.contains(&start) || excluded.contains(&goal) {
            return None;
        }
        if start == goal {
            return Some(vec![start]);
        }

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut pred: HashMap<NodeIndex, NodeIndex> = HashMap::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            for neighbor in self.graph.neighbors(node) {
                if excluded.contains(&neighbor) || visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                pred.insert(neighbor, node);
                if neighbor == goal {
                    let mut path = vec![goal];
                    let mut cur = goal;
                    while let Some(&p) = pred.get(&cur) {
                        path.push(p);
                        cur = p;
                        if p == start {
                            break;
                        }
                    }
                    path.reverse();
                    return Some(path);
                }
                queue.push_back(neighbor);
            }
        }

        None
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str, kind: NodeKind) -> Node {
        Node {
            id: id.to_string(),
            name: format!("node-{id}"),
            kind,
        }
    }

    fn link(id: &str, capacity_mbps: f64) -> Link {
        Link {
            id: id.to_string(),
            capacity_mbps,
            latency_ms: 1.0,
        }
    }

    #[test]
    fn build_and_counts() {
        let mut net = Network::new();
        net.add_node(node("a", NodeKind::Core)).unwrap();
        net.add_node(node("b", NodeKind::Edge)).unwrap();
        net.add_node(node("c", NodeKind::Exchange)).unwrap();
        net.add_link("a", "b", link("l1", 100.0)).unwrap();
        net.add_link("b", "c", link("l2", 200.0)).unwrap();

        assert_eq!(net.node_count(), 3);
        assert_eq!(net.link_count(), 2);
        assert_eq!(net.degree("b"), Some(2));
        assert_eq!(net.degree("a"), Some(1));
        assert_eq!(net.degree("missing"), None);
    }

    #[test]
    fn connectivity_true_and_false() {
        let mut net = Network::new();
        for id in ["a", "b", "c", "d"] {
            net.add_node(node(id, NodeKind::Core)).unwrap();
        }
        net.add_link("a", "b", link("l1", 10.0)).unwrap();
        net.add_link("b", "c", link("l2", 10.0)).unwrap();

        assert!(net.is_connected("a", "c"));
        assert!(!net.is_connected("a", "d"));
    }

    #[test]
    fn incident_capacity_sums() {
        let mut net = Network::new();
        net.add_node(node("a", NodeKind::Core)).unwrap();
        net.add_node(node("b", NodeKind::Core)).unwrap();
        net.add_node(node("c", NodeKind::Core)).unwrap();
        net.add_link("a", "b", link("l1", 100.0)).unwrap();
        net.add_link("a", "c", link("l2", 50.0)).unwrap();

        assert_eq!(net.incident_capacity_mbps("a"), 150.0);
        assert_eq!(net.incident_capacity_mbps("b"), 100.0);
        assert_eq!(net.incident_capacity_mbps("missing"), 0.0);
    }

    #[test]
    fn diverse_path_true_on_ring() {
        let mut net = Network::new();
        for id in ["a", "b", "c", "d"] {
            net.add_node(node(id, NodeKind::Core)).unwrap();
        }
        net.add_link("a", "b", link("l1", 10.0)).unwrap();
        net.add_link("b", "c", link("l2", 10.0)).unwrap();
        net.add_link("c", "d", link("l3", 10.0)).unwrap();
        net.add_link("d", "a", link("l4", 10.0)).unwrap();

        assert!(net.has_diverse_path("a", "c"));
    }

    #[test]
    fn diverse_path_false_on_chain() {
        let mut net = Network::new();
        for id in ["a", "b", "c", "d"] {
            net.add_node(node(id, NodeKind::Core)).unwrap();
        }
        net.add_link("a", "b", link("l1", 10.0)).unwrap();
        net.add_link("b", "c", link("l2", 10.0)).unwrap();
        net.add_link("c", "d", link("l3", 10.0)).unwrap();

        assert!(!net.has_diverse_path("a", "d"));
    }

    #[test]
    fn add_node_rejects_duplicate() {
        let mut net = Network::new();
        net.add_node(node("a", NodeKind::Core)).unwrap();
        let err = net.add_node(node("a", NodeKind::Edge)).unwrap_err();
        assert!(matches!(err, NetworkError::DuplicateNode(_)));
    }

    #[test]
    fn add_link_rejects_bad_inputs() {
        let mut net = Network::new();
        net.add_node(node("a", NodeKind::Core)).unwrap();
        net.add_node(node("b", NodeKind::Core)).unwrap();

        let bad_capacity = net.add_link("a", "b", link("l1", 0.0)).unwrap_err();
        assert!(matches!(bad_capacity, NetworkError::NonPositiveCapacity(_)));

        let unknown = net.add_link("a", "z", link("l2", 10.0)).unwrap_err();
        assert!(matches!(unknown, NetworkError::UnknownNode(_)));
    }
}

use std::collections::{HashMap, HashSet, VecDeque};

/// An unweighted directed (cyclic or acyclic) graph.
pub struct DependencyGraph {
    graph: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    /// Constructs a new, empty `DependencyGraph`.
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    /// Adds an edge from vertex `a` to vertex `b` to the `DependencyGraph`.
    pub fn add_edge(&mut self, a: String, b: String) {
        if let Some(x) = self.graph.get_mut(&a) {
            x.insert(b);
        } else {
            self.graph.insert(a, HashSet::from([b]));
        }
    }

    /// Returns the first cycle found in the `DependencyGraph`.
    /// If no cycle exists, then `None` is returned.
    pub fn find_cycle(&self) -> Option<Vec<String>> {
        // A cycle exists as the presence of a back edge indicates a cycle in a directed graph
        if let Some(edge) = self.find_back_edge() {
            let predecessors = self
                .find_shortest_path(edge.0.clone(), edge.1.clone())
                .unwrap();

            return Some(DependencyGraph::reconstruct_cycle_path(
                edge.0,
                edge.1,
                predecessors,
            ));
        }

        // No cycle exists
        None
    }

    /// Returns the first back edge found in the `DependencyGraph` using DFS.
    /// If no back edge exists, then `None` is returned.
    fn find_back_edge(&self) -> Option<(String, String)> {
        let mut discovered: HashSet<String> = HashSet::new();
        let mut finished: HashSet<String> = HashSet::new();

        for vertex in self.graph.keys() {
            if discovered.contains(vertex) && finished.contains(vertex) {
                continue;
            }

            if let Some(edge) = self.dfs_visit(vertex.clone(), &mut discovered, &mut finished) {
                return Some((edge.0, edge.1));
            }
        }

        None
    }

    /// Returns the first back edge found while analysing a vertex and its children.
    /// If no back edge exists, then `None` is returned.
    fn dfs_visit(
        &self,
        vertex: String,
        discovered: &mut HashSet<String>,
        finished: &mut HashSet<String>,
    ) -> Option<(String, String)> {
        discovered.insert(vertex.clone());

        if let Some(children) = &self.graph.get(&vertex) {
            for child in children.iter() {
                if discovered.contains(child) {
                    return Some((child.clone(), vertex));
                }

                if !finished.contains(child) {
                    if let Some(edge) = self.dfs_visit(child.to_string(), discovered, finished) {
                        return Some((edge.0, edge.1));
                    }
                }
            }
        }

        discovered.remove(&vertex);
        finished.insert(vertex);

        None
    }

    /// Returns the predecessors of the shortest path using BFS.
    /// The predecessors can then be used to reconstruct the path.
    fn find_shortest_path(&self, start: String, end: String) -> Option<HashMap<String, String>> {
        let mut queue: VecDeque<String> = VecDeque::from([start.clone()]);
        let mut visited: HashSet<String> = HashSet::from([start]);
        let mut predecessors: HashMap<String, String> = HashMap::new();

        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();

            if vertex == end {
                return Some(predecessors);
            }

            if let Some(children) = &self.graph.get(&vertex) {
                for child in children.iter() {
                    if visited.contains(child) {
                        continue;
                    }

                    queue.push_back(child.clone());
                    visited.insert(child.clone());
                    predecessors.insert(child.to_string(), vertex.clone());

                    if child == &end {
                        return Some(predecessors);
                    }
                }
            }
        }

        None
    }

    /// Returns the reconstructed cycle path from vertex `start` to vertex `end`.
    /// The vertex `start` will be present twice, at the beginning and at the end of the result.
    fn reconstruct_cycle_path(
        start: String,
        end: String,
        predecessors: HashMap<String, String>,
    ) -> Vec<String> {
        let mut path: Vec<String> = Vec::from([end.clone()]);
        let mut crawl = end;

        while let Some(predecessor) = predecessors.get(&crawl) {
            path.push(predecessor.to_string());
            crawl = predecessor.to_string();
        }

        path.reverse();
        path.push(start);

        path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contains_no_cycle_when_empty() {
        let dependency_graph = DependencyGraph::new();
        let cycle = dependency_graph.find_cycle();

        assert_eq!(cycle, None);
    }

    #[test]
    fn contains_no_cycle_when_no_back_edge() {
        let mut dependency_graph = DependencyGraph::new();

        dependency_graph.add_edge(String::from("A"), String::from("B"));
        dependency_graph.add_edge(String::from("B"), String::from("C"));
        dependency_graph.add_edge(String::from("C"), String::from("D"));
        dependency_graph.add_edge(String::from("A"), String::from("D"));

        let cycle = dependency_graph.find_cycle();

        assert_eq!(cycle, None);
    }

    #[test]
    fn contains_cycle_when_loop() {
        let mut dependency_graph = DependencyGraph::new();

        dependency_graph.add_edge(String::from("A"), String::from("A"));

        let cycle = dependency_graph.find_cycle().unwrap();

        assert_eq!(cycle, vec![String::from("A"), String::from("A")]);
    }

    #[test]
    fn contains_cycle_when_back_edge() {
        let mut dependency_graph = DependencyGraph::new();

        dependency_graph.add_edge(String::from("A"), String::from("B"));
        dependency_graph.add_edge(String::from("B"), String::from("C"));
        dependency_graph.add_edge(String::from("C"), String::from("B"));
        dependency_graph.add_edge(String::from("C"), String::from("D"));

        let cycle = dependency_graph.find_cycle().unwrap();

        assert_eq!(cycle.len(), 3);
    }
}

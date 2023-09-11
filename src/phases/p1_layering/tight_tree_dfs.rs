use std::collections::{ HashSet, hash_set::Iter };

use petgraph::{stable_graph::{NodeIndex, StableDiGraph, EdgeIndex}, Direction::{Incoming, Outgoing}, visit::EdgeRef};

use super::{InitialRanks, Vertex, Edge, traits::Slack};

#[derive(Debug)]
pub(super) struct TightTreeDFS {
    vertices: HashSet<NodeIndex>,
    pub(super) edges: HashSet<EdgeIndex>,
}

impl TightTreeDFS {
    pub(super) fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }

    pub(super) fn contains_vertex(&self, vertex: &NodeIndex) -> bool {
        self.vertices.contains(&(*vertex).into())
    }

    pub(super) fn contains_edge(&self, edge: EdgeIndex) -> bool {
        self.edges.contains(&edge)
    }

    pub(super) fn vertices(&self) -> Iter<'_, NodeIndex> {
        self.vertices.iter()
    }

    /// Returns true if exactly one vertex is a member of the tree.
    pub(super) fn is_incident_edge(&self, edge: &EdgeIndex, graph: &StableDiGraph<Vertex, Edge>) -> bool {
        let (tail, head)  = graph.edge_endpoints(*edge).unwrap();
        self.vertices.contains(&tail) ^ self.vertices.contains(&head)
    }

    pub(super) fn tight_tree(&mut self, ranked: &InitialRanks, vertex: NodeIndex, visited: &mut HashSet<EdgeIndex>) -> usize {
        // start from topmost nodes.
        // then for each topmost node add nodes to tree until done. Then continue with next node until no more nodes are found.
        let mut node_count = 1;
        if !self.vertices.contains(&vertex) {
            self.vertices.insert(vertex);
        }

        for edge in ranked.graph.edges_directed(vertex, Incoming).chain(ranked.graph.edges_directed(vertex, Outgoing)) {
            let (tail, head) = (edge.source(), edge.target());
            let edge = edge.id();
            // find out if the other vertex of the edge is the head or the tail
            let other = if tail == vertex { head } else { tail };

            if !visited.contains(&edge) {
                visited.insert(edge);
                if self.edges.contains(&edge) {
                    node_count += self.tight_tree(ranked, other, visited);
                } else if  ranked.slack(edge) == 0 {
                    //self.vertices.insert(other);
                    self.edges.insert(edge);
                    node_count += self.tight_tree(ranked, other, visited);
                }
            }
        }

        node_count
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::phases::p1_layering::{tight_tree_dfs::TightTreeDFS, tests::{Builder, GraphBuilder, EXAMPLE_GRAPH, UnlayeredGraphBuilder}};


        #[test]
        fn test_dfs_start_from_root() {
            let mut dfs = TightTreeDFS::new();
            let initial_ranks = Builder::<UnlayeredGraphBuilder>::from_edges(&EXAMPLE_GRAPH).build().init_rank();
            let number_of_nodes = initial_ranks.graph.node_count();
            dfs.tight_tree(&initial_ranks, 0.into(), &mut HashSet::new());
            assert_eq!(dfs.edges.len(), number_of_nodes - 1);
            assert_eq!(dfs.vertices.len(), number_of_nodes);
        }

        #[test]
        fn test_dfs_start_not_from_root() {
            let mut dfs = TightTreeDFS::new();
            let initial_ranks = Builder::<UnlayeredGraphBuilder>::from_edges(&EXAMPLE_GRAPH).build().init_rank();
            let number_of_nodes = initial_ranks .graph.node_count();
            dfs.tight_tree(&initial_ranks, 4.into(), &mut HashSet::new());
            assert_eq!(dfs.edges.len(), number_of_nodes - 1);
            assert_eq!(dfs.vertices.len(), number_of_nodes);
        }

}
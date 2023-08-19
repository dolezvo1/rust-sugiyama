mod rank;
mod tree;
#[cfg(test)]
mod tests;

use std::collections::{HashSet, HashMap, VecDeque};

use petgraph::Direction::{*, self};
use petgraph::stable_graph::{StableDiGraph, NodeIndex};
use petgraph::visit::{EdgeRef, IntoNeighborsDirected};

use crate::util::layers::Layers;

use self::rank::Ranks;
use self::tree::{TreeSubgraph, TighTreeDFS};

use super::p2_reduce_crossings::ProperLayeredGraph;

pub(crate) fn start_layering<T: Default>(graph: StableDiGraph<Option<T>, usize>) -> UnlayeredGraph<T> {
    UnlayeredGraph { graph }
}

// create from input graph
pub(crate) struct UnlayeredGraph<T: Default> {
    graph: StableDiGraph<Option<T>, usize>
}

impl<T: Default> UnlayeredGraph<T> {
    pub(crate) fn initial_ranking(self, minimum_length: usize) -> TightTreeBuilder<T> {
        let mut scanned = HashSet::<(NodeIndex, NodeIndex)>::new();
        let mut ranks = HashMap::<NodeIndex, isize>::new();

        // Sort nodes topologically so we don't need to verify that we've assigned
        // a rank to all incoming neighbors
        // assume graphs contain no circles for now
        for v in petgraph::algo::toposort(&self.graph, None).unwrap() {
            self.graph.neighbors_directed(v, Incoming).for_each(|u| assert!(scanned.contains(&(u, v))));
            
            let rank = self.graph.neighbors_directed(v, Incoming)
                                 .filter_map(|n| ranks.get(&n).and_then(|r| Some(r + 1)))
                                 .max()
                                 .unwrap_or(0);

            for n in self.graph.neighbors_directed(v, Outgoing) {
                scanned.insert((v, n));
            }

            ranks.insert(v, rank);
        }

        let ranks = Ranks::new(ranks, &self.graph, minimum_length);
        TightTreeBuilder { graph: self.graph, ranks }
    }
}

pub(crate) struct TightTreeBuilder<T: Default> {
    graph: StableDiGraph<Option<T>, usize>,
    ranks: Ranks,
}

impl<T: Default> TightTreeBuilder<T> {
    #[cfg(test)]
    fn new(graph: StableDiGraph<Option<T>, usize>, ranks: Ranks) -> Self {
        Self { graph, ranks }
    }

    pub(crate) fn make_tight(mut self) -> FeasibleTreeBuilder<T> {
        // take a random edge to start the tree from
        // split edges into sets consisting of tree and non tree edges.
        // in the beginning, all edges are non tree edges, and they are added
        // with each call to dfs.

        // build a new graph which is a tree. 
        // Remember only edges which where part of the original graph
        // each time we add an edge to the tree, we remove it from the graph
        let num_nodes = self.graph.node_count();
        let mut nodes = self.graph.node_indices().into_iter();
        let mut dfs = TighTreeDFS::new();
        
        while dfs.build_tight_tree(&self.graph, &self.ranks, nodes.next().unwrap(), &mut HashSet::new()) < num_nodes {
            let (tail, head) = self.find_non_tight_edge(&dfs);
            let mut delta = self.ranks.slack(tail, head);

            if dfs.contains_vertex(&head) {
                delta = -delta;
            }

            self.ranks.tighten_edge(&dfs, delta)
        }

        // remove all edges which are contained in tree from graph
        dfs.make_edges_disjoint(&mut self.graph);

        FeasibleTreeBuilder { graph: self.graph, ranks: self.ranks, tree: dfs.into_tree_subgraph() }
    }
    
    fn find_non_tight_edge(&self, tree: &TighTreeDFS) -> (NodeIndex, NodeIndex) {
        self.graph.edge_indices()
            .filter_map(|e| self.graph.edge_endpoints(e))
            .filter(|(tail, head)| !tree.contains_edge(*tail, *head) && tree.is_incident_edge(tail, head))
            .min_by(|a, b| self.ranks.slack(a.0, a.1).cmp(&self.ranks.slack(b.0, b.1))).unwrap()
    }
}


pub(crate) struct FeasibleTreeBuilder<T: Default> {
    graph: StableDiGraph<Option<T>, usize>,
    ranks: Ranks,
    tree: StableDiGraph<Option<T>, usize>,
}

impl<T: Default> FeasibleTreeBuilder<T> {
    pub(crate) fn init_cutvalues(self) -> FeasibleTree<T> {
        // assumes all edges have a weight of one
        let mut cut_values = HashMap::<(NodeIndex, NodeIndex), isize>::new();
        let mut queue = self.leaves();

        // traverse tree inward via breadth first starting from leaves
        while let Some(vertex) = queue.pop_front() {
            // terminate early if all cutvalues are known
            if cut_values.len() == self.tree.edge_count() {
                println!("done early");
                break;
            }
            let (mut cut_values_incoming, mut missing_cut_values_incoming) = 
                self.get_neighborhood_info(vertex, &mut cut_values, Incoming); 
            let (mut cut_values_outgoing, mut missing_cut_values_outgoing) = 
                self.get_neighborhood_info(vertex, &mut cut_values, Outgoing); 
            let (mut incoming, mut outgoing) = (Direction::Incoming, Direction::Outgoing);

            // if we can't calculate cut value yet, or the value is already known
            if missing_cut_values_incoming.len() > 1 || missing_cut_values_outgoing.len() > 1 || 
                missing_cut_values_incoming.len() == 0 && missing_cut_values_outgoing.len() == 0 {
                continue;
            } 

            // switch direction, if vertex is tail component of edge
            let edge = if missing_cut_values_outgoing.len() == 1 {
                std::mem::swap(&mut cut_values_incoming, &mut cut_values_outgoing);
                std::mem::swap(&mut missing_cut_values_incoming, &mut missing_cut_values_outgoing);
                std::mem::swap(&mut incoming, &mut outgoing);
                (vertex, missing_cut_values_incoming[0])
            } else {
                (missing_cut_values_incoming[0], vertex)
            };

            let cut_value = 1 + self.graph.neighbors_directed(vertex, incoming).count() as isize - 
                cut_values_incoming.iter().sum::<isize>() + cut_values_incoming.len() as isize - 
                self.graph.neighbors_directed(vertex, outgoing).count() as isize + 
                cut_values_outgoing.iter().sum::<isize>() - cut_values_outgoing.len() as isize;
            
            cut_values.insert(edge, cut_value);
            // continue traversing tree in direction of edge whose vertex was missing before
            queue.push_back(missing_cut_values_incoming[0]);
        }

        FeasibleTree { graph: self.graph, tree: self.tree, ranks: self.ranks, cut_values }
    }

    fn get_neighborhood_info(
        &self, 
        vertex: NodeIndex, 
        cut_values: &mut HashMap<(NodeIndex, NodeIndex), isize>, 
        direction: Direction
    ) -> (Vec<isize>, Vec<NodeIndex>) {
        let mut cuts = Vec::new(); 
        let mut missing = Vec::new();
        for edge in self.tree.edges_directed(vertex, direction) {
            let (tail, head) = (edge.source(), edge.target());
            if let Some(cut_value) = cut_values.get(&(tail, head)) {
                cuts.push(*cut_value);
            } else {
                missing.push(if tail == vertex { head } else { tail });
            }
        }
        (cuts, missing)
    }

    fn leaves(&self) -> VecDeque<NodeIndex> {
        self.tree.node_indices().filter(|v| self.tree.neighbors_undirected(*v).count() < 2).collect::<VecDeque<_>>()
    }
}

pub(crate) struct FeasibleTree<T: Default> {
    graph: StableDiGraph<Option<T>, usize>,
    tree: StableDiGraph<Option<T>, usize>,
    ranks: Ranks,
    pub cut_values: HashMap<(NodeIndex, NodeIndex), isize>,
}

impl<T: Default> FeasibleTree<T> {
    fn rank(mut self) -> ProperLayeredGraph<T> {
        while let Some(edge) = self.leave_edge() {
            // swap edges and calculate cut value
        }

        // TODO maybe destcructure this or turn it into Layers.
        self.ranks.normalize();
        // don't balance ranks since we want maximum width to 
        // give indication about number of parallel processes running
        ProperLayeredGraph::new(Layers::new_empty(), self.graph)
    }

    fn leave_edge(&self) -> Option<(NodeIndex, NodeIndex)> {
        for (edge, cut_value) in self.cut_values.iter() {
            if cut_value < &0 {
                return Some(*edge);
            }
        }
        None
    }
}
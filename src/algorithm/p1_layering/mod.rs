// TODOS: Keep non graph edges during rank() procedure in vecdeque to be able to cyclically search through them
mod cut_values;
mod low_lim;
mod ranking;
#[cfg(test)]
pub(crate) mod tests;

use petgraph::Direction::Incoming;
use petgraph::stable_graph::{StableDiGraph, EdgeIndex};
use petgraph::visit::IntoNodeIdentifiers;

use self::cut_values::update_cutvalues;
use self::low_lim::update_low_lim;
use self::ranking::{feasible_tree, update_ranks};

use super::{Vertex, Edge, slack};

pub(super) fn rank(graph: &mut StableDiGraph<Vertex, Edge>, minimum_length: i32) {
    feasible_tree(graph, minimum_length);
    while let Some(removed_edge) = leave_edge(graph) {
        // swap edges and calculate cut value
        let swap_edge = enter_edge(graph, removed_edge, minimum_length);
        exchange(graph, removed_edge, swap_edge, minimum_length);
    }

    // don't balance ranks since we want maximum width to 
    // give indication about number of parallel processes running
    normalize(graph);
}

pub(super) fn move_roots_to_top(graph: &mut StableDiGraph<Vertex, Edge>) {
    for v in graph.node_indices().collect::<Vec<_>>() {
        if graph.neighbors_directed(v, Incoming).count() == 0 {
            graph[v].rank = 0;
        }
    }
}

fn leave_edge(graph: &StableDiGraph<Vertex, Edge>) -> Option<EdgeIndex> {
    for edge in graph.edge_indices() {
        if let Some(cut_value) = graph[edge].cut_value {
            if cut_value < 0 {
                return Some(edge);
            }
        }
    }
    None
}

fn enter_edge(graph: &mut StableDiGraph<Vertex, Edge>, edge: EdgeIndex, minimum_length: i32) -> EdgeIndex {
    // find a non-tree edge to replace e.
    // remove e from tree
    // consider all edges going from head to tail component.
    // choose edge with minimum slack.
    let (mut u, mut v) = graph.edge_endpoints(edge).map(|(t, h)| (graph[t], graph[h])).unwrap();
    let is_root_in_head = u.lim < v.lim;
    if !is_root_in_head {
        std::mem::swap(&mut u, &mut v); 
    }

    graph.edge_indices()
        .filter(|e| !graph[*e].is_tree_edge && is_head_to_tail(graph, *e, u, is_root_in_head))
        .min_by(|e1, e2| slack(graph,*e1, minimum_length).cmp(&slack(graph, *e2, minimum_length)))
        .unwrap()
}

fn exchange(graph: &mut StableDiGraph<Vertex, Edge>, removed_edge: EdgeIndex, swap_edge: EdgeIndex, minimum_length: i32) {
    // swap edges 
    graph[removed_edge].is_tree_edge = false;
    graph[swap_edge].is_tree_edge = true;

    // update the graph
    let least_common_ancestor = update_cutvalues(graph, removed_edge, swap_edge);
    update_low_lim(graph, least_common_ancestor);
    update_ranks(graph, minimum_length);
}

fn normalize(graph: &mut StableDiGraph<Vertex, Edge>) {
    let min_rank = graph.node_identifiers().map(|v| graph[v].rank).min().unwrap();
    for v in graph.node_weights_mut() {
        v.rank -= min_rank;
    }
}

fn is_head_to_tail(graph: &StableDiGraph<Vertex, Edge>, edge: EdgeIndex, u: Vertex, is_root_in_head: bool) -> bool {
    // edge needs to go from head to tail. e.g. tail neads to be in head component, and head in tail component
    let (tail, head) = graph.edge_endpoints(edge).map(|(t, h)| (graph[t], graph[h])).unwrap();
    // check if head is in tail component
    is_root_in_head == (u.low <= head.lim && head.lim <= u.lim) &&
    // check if tail is in head component
    is_root_in_head != (u.low <= tail.lim && tail.lim <= u.lim)
}
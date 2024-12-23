use std::collections::HashMap;

use algorithm::{Edge, Vertex};
use configure::CoordinatesBuilder;

use log::info;
use petgraph::stable_graph::StableDiGraph;

mod algorithm;
pub mod configure;
mod util;

type Layout = (Vec<(usize, (isize, isize))>, usize, usize);
type Layouts<T> = Vec<(Vec<(T, (isize, isize))>, usize, usize)>;
type RawGraph<'a> = (&'a [u32], &'a [(u32, u32)]);

/// Creates a graph layout from edges, which are given as a `&[(u32, u32)]`.
///
/// It returns a [CoordinatesBuilder] which can be used to configure the
/// layout.
pub fn from_edges(edges: &[(u32, u32)]) -> CoordinatesBuilder<&[(u32, u32)]> {
    info!(target: "initializing", "Creating new layout from edges, containing {} edges", edges.len());
    let graph = StableDiGraph::from_edges(edges);
    CoordinatesBuilder::new(graph)
}

/// Creates a graph layout from a preexisting `StableDiGraph<V, E>`.
///
/// It returns a [CoordinatesBuilder] which can be used to configure the
/// layout.
pub fn from_graph<V, E>(graph: &StableDiGraph<V, E>) -> CoordinatesBuilder<StableDiGraph<V, E>> {
    info!(target: "initializing", 
        "Creating new layout from existing graph, containing {} vertices and {} edges.", 
        graph.node_count(), 
        graph.edge_count());

    let graph = graph.map(|id, _| Vertex::new(id.index()), |_, _| Edge::default());
    CoordinatesBuilder::new(graph)
}

/// Creates a graph layot from `&[u32]` (vertices)
/// and `&[(u32, u32)]` (edges).
///
/// It returns a [CoordinatesBuilder] which can be used to configure the
/// layout.
///
/// # Panics
///
/// Panics if `edges` contain vertices which are not contained in `vertices`
pub fn from_vertices_and_edges<'a>(
    vertices: &'a [u32],
    edges: &'a [(u32, u32)],
) -> CoordinatesBuilder<RawGraph<'a>> {
    info!(target: "initializing", 
        "Creating new layout from existing graph, containing {} vertices and {} edges.", 
        vertices.len(), 
        edges.len());

    let mut graph = StableDiGraph::new();
    let mut id_map = HashMap::new();
    for v in vertices {
        let id = graph.add_node(Vertex::new(*v as usize));
        id_map.insert(*v, id);
    }

    for (tail, head) in edges {
        graph.add_edge(
            *id_map.get(tail).unwrap(),
            *id_map.get(head).unwrap(),
            Edge::default(),
        );
    }

    CoordinatesBuilder::new(graph)
}

#[cfg(test)]
mod benchmark {
    use super::from_edges;

    #[test]
    fn r_100() {
        let edges = graph_generator::RandomLayout::new(100)
            .build_edges()
            .into_iter()
            .map(|(r, l)| (r as u32, l as u32))
            .collect::<Vec<(u32, u32)>>();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!("Random 100 edges: {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn r_1000() {
        let edges = graph_generator::RandomLayout::new(1000)
            .build_edges()
            .into_iter()
            .map(|(r, l)| (r as u32, l as u32))
            .collect::<Vec<(u32, u32)>>();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!("Random 1000 edges: {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn r_2000() {
        let edges = graph_generator::RandomLayout::new(2000).build_edges();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!("Random 2000 edges: {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn r_4000() {
        let edges = graph_generator::RandomLayout::new(4000).build_edges();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!("Random 4000 edges: {}ms", start.elapsed().as_millis());
    }

    #[test]
    fn l_1000_2() {
        let n = 1000;
        let e = 2;
        let edges = graph_generator::GraphLayout::new_from_num_nodes(n, e).build_edges();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!(
            "{n} nodes, {e} edges per node: {}ms",
            start.elapsed().as_millis()
        );
    }

    #[test]
    fn l_2000_2() {
        let n = 2000;
        let e = 2;
        let edges = graph_generator::GraphLayout::new_from_num_nodes(n, e).build_edges();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!(
            "{n} nodes, {e} edges per node: {}ms",
            start.elapsed().as_millis()
        );
    }

    #[test]
    fn l_4000_2() {
        let n = 4000;
        let e = 2;
        let edges = graph_generator::GraphLayout::new_from_num_nodes(n, e).build_edges();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!(
            "{n} nodes, {e} edges per node: {}ms",
            start.elapsed().as_millis()
        );
    }

    #[test]
    fn l_8000_2() {
        let n = 8000;
        let e = 2;
        let edges = graph_generator::GraphLayout::new_from_num_nodes(n, e).build_edges();
        let start = std::time::Instant::now();
        let _ = from_edges(&edges).build();
        println!(
            "{n} nodes, {e} edges per node: {}ms",
            start.elapsed().as_millis()
        );
    }
}

#[cfg(test)]
mod check_visuals {

    use crate::from_vertices_and_edges;

    use super::from_edges;

    #[test]
    fn test_no_dummies() {
        let vertices = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let edges = [
            (1, 2),
            (1, 3),
            (2, 5),
            (2, 16),
            (4, 5),
            (4, 6),
            (4, 7),
            (6, 17),
            (6, 3),
            (6, 18),
            (8, 3),
            (8, 9),
            (8, 10),
            (9, 16),
            (9, 7),
            (9, 19),
            (11, 7),
            (11, 12),
            (11, 13),
            (12, 18),
            (12, 10),
            (12, 20),
            (14, 10),
            (14, 15),
            (15, 19),
            (15, 13),
        ];
        let _ = from_vertices_and_edges(&vertices, &edges)
            .dummy_vertices(true)
            .build();
    }
    #[test]
    fn verify_looks_good() {
        // NOTE: This test might fail eventually, since the order of lements in a row canot be guaranteed;
        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 5),
            (3, 6),
            (3, 7),
            (3, 8),
            (4, 5),
            (4, 6),
            (4, 7),
            (4, 8),
            (5, 9),
            (6, 9),
            (7, 9),
            (8, 9),
        ];
        let (layout, width, height) = &mut from_edges(&edges).build()[0];
        layout.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(*width, 4);
        assert_eq!(*height, 6);
        println!("{:?}", layout);
    }

    #[test]
    fn root_vertices_on_top_disabled() {
        let edges = [(1, 0), (2, 1), (3, 0), (4, 0)];
        let layout = from_edges(&edges).build();
        for (id, (_, y)) in layout[0].0.clone() {
            if id == 2 {
                assert_eq!(y, 0);
            } else if id == 3 || id == 4 || id == 1 {
                assert_eq!(y, -10);
            } else {
                assert_eq!(y, -20)
            }
        }
    }

    #[test]
    fn check_coords_2() {
        let edges = [
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 4),
            (4, 5),
            (5, 6),
            (2, 6),
            (3, 6),
            (3, 7),
            (3, 8),
            (3, 9),
        ];
        let layout = from_edges(&edges).build();
        println!("{:?}", layout);
    }

    #[test]
    fn hlrs_ping() {
        let _nodes = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
        ];
        let edges = [
            (1, 2),
            (1, 4),
            (1, 5),
            (1, 3),
            (2, 4),
            (2, 5),
            (3, 9),
            (3, 10),
            (3, 8),
            (4, 6),
            (4, 9),
            (4, 8),
            (5, 6),
            (5, 10),
            (5, 8),
            (6, 7),
            (7, 9),
            (7, 10),
            (8, 14),
            (8, 15),
            (8, 13),
            (9, 11),
            (9, 14),
            (9, 13),
            (10, 11),
            (10, 15),
            (10, 13),
            (11, 12),
            (12, 14),
            (12, 15),
            (13, 18),
            (13, 19),
            (13, 20),
            (14, 16),
            (14, 18),
            (14, 20),
            (15, 16),
            (15, 19),
            (15, 20),
            (16, 17),
            (17, 18),
            (17, 19),
            (18, 21),
            (19, 21),
        ]
        .into_iter()
        .map(|(t, h)| (t - 1, h - 1))
        .collect::<Vec<_>>();

        let layout = from_edges(&edges)
            .layering_type(crate::configure::RankingType::Up)
            .build();
        println!("{layout:?}");
    }

    #[test]
    fn run_algo_empty_graph() {
        use super::from_edges;
        let edges = [];
        let g = from_edges(&edges).build();
        assert!(g.is_empty());
    }

    #[test]
    fn run_algo_with_duplicate_edges() {
        let edges = [
            (1, 2),
            (2, 5),
            (2, 6),
            (2, 3),
            (3, 4),
            (4, 3),
            (4, 8),
            (8, 4),
            (8, 7),
            (3, 7),
            (6, 7),
            (7, 6),
            (5, 6),
            (5, 1),
        ];

        let layout = from_edges(&edges).build();
        println!("{layout:?}");
    }
}

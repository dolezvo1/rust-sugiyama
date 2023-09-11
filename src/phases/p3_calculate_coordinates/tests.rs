use petgraph::stable_graph::{StableDiGraph, NodeIndex};

use super::{Vertex, Edge, MinimalCrossings, VDir, HDir};

pub(crate) fn create_test_layout() -> MinimalCrossings {

    let edges: [(u32, u32); 30] = [(0, 2), (0, 6), (0, 18), (1, 16), (1, 17), 
                    (3, 8), (16, 8), (4, 8), (17, 19), (18, 20), (5, 8), (5, 9), (6, 8), (6, 21),
                    (7, 10), (7, 11), (7, 12), (19, 23), (20, 24), (21, 12), (9, 22), (9, 25),
                    (10, 13), (10, 14), (11, 14), (22, 13), (23, 15), (24, 15), (12, 15), (25, 15)];

    let mut graph = StableDiGraph::<Vertex, Edge>::from_edges(&edges);
    let layers: Vec<Vec<NodeIndex>> = [
        vec![0, 1],
        vec![2, 3, 16, 4, 17, 18, 5, 6],
        vec![7, 8, 19, 20, 21, 9],
        vec![10, 11, 22, 23, 24, 12, 25],
        vec![13, 14, 15],
    ].into_iter().map(|row| row.into_iter().map(|id| id.into()).collect())
    .collect();
    
    for (rank, row) in layers.iter().enumerate() {
        for (pos, v) in row.iter().enumerate() {
            let weight = &mut graph[*v];
            if v.index() < 16 {
                *weight = Vertex::new(0, *v, rank, pos, false);
            } else {
                *weight = Vertex::new(0, *v, rank, pos, true);
            }
        }
    }


    MinimalCrossings { layers, graph }
}

#[test]
fn type_1() {
    let mc = create_test_layout();
    let c = mc.mark_type_1_conflicts();
    assert!(c.graph[c.find_edge(6.into(), 8.into()).unwrap()].has_type_1_conflict);
    assert!(c.graph[c.find_edge(7.into(), 12.into()).unwrap()].has_type_1_conflict);
    assert!(c.graph[c.find_edge(5.into(), 8.into()).unwrap()].has_type_1_conflict);
    assert!(c.graph[c.find_edge(9.into(), 22.into()).unwrap()].has_type_1_conflict);
}

#[test]
fn alignment_down_right() {
    let mc = create_test_layout().mark_type_1_conflicts().create_vertical_alignments(VDir::Down, HDir::Right); 
    // verify roots
    assert_eq!(mc[NodeIndex::from(0)].root, 0.into());
    assert_eq!(mc[NodeIndex::from(1)].root, 1.into());
    assert_eq!(mc[NodeIndex::from(2)].root, 0.into());
    assert_eq!(mc[NodeIndex::from(3)].root, 3.into());
    assert_eq!(mc[NodeIndex::from(4)].root, 4.into());
    assert_eq!(mc[NodeIndex::from(5)].root, 5.into());
    assert_eq!(mc[NodeIndex::from(6)].root, 6.into());
    assert_eq!(mc[NodeIndex::from(7)].root, 7.into());
    assert_eq!(mc[NodeIndex::from(8)].root, 4.into());
    assert_eq!(mc[NodeIndex::from(9)].root, 9.into());
    assert_eq!(mc[NodeIndex::from(10)].root, 7.into());
    assert_eq!(mc[NodeIndex::from(11)].root, 11.into());
    assert_eq!(mc[NodeIndex::from(12)].root, 6.into());
    assert_eq!(mc[NodeIndex::from(13)].root, 7.into());
    assert_eq!(mc[NodeIndex::from(14)].root, 11.into());
    assert_eq!(mc[NodeIndex::from(15)].root, 18.into());
    assert_eq!(mc[NodeIndex::from(16)].root, 1.into());
    assert_eq!(mc[NodeIndex::from(17)].root, 17.into());
    assert_eq!(mc[NodeIndex::from(18)].root, 18.into());
    assert_eq!(mc[NodeIndex::from(19)].root, 17.into());
    assert_eq!(mc[NodeIndex::from(20)].root, 18.into());
    assert_eq!(mc[NodeIndex::from(21)].root, 6.into());
    assert_eq!(mc[NodeIndex::from(22)].root, 22.into());
    assert_eq!(mc[NodeIndex::from(23)].root, 17.into());
    assert_eq!(mc[NodeIndex::from(24)].root, 18.into());
    assert_eq!(mc[NodeIndex::from(25)].root, 9.into());
    
    // verify alignments
    assert_eq!(mc[NodeIndex::from(0)].align, 2.into());
    assert_eq!(mc[NodeIndex::from(1)].align, 16.into());
    assert_eq!(mc[NodeIndex::from(2)].align, 0.into());
    assert_eq!(mc[NodeIndex::from(3)].align, 3.into());
    assert_eq!(mc[NodeIndex::from(4)].align, 8.into());
    assert_eq!(mc[NodeIndex::from(5)].align, 5.into());
    assert_eq!(mc[NodeIndex::from(6)].align, 21.into());
    assert_eq!(mc[NodeIndex::from(7)].align, 10.into());
    assert_eq!(mc[NodeIndex::from(8)].align, 4.into());
    assert_eq!(mc[NodeIndex::from(9)].align, 25.into());
    assert_eq!(mc[NodeIndex::from(10)].align, 13.into());
    assert_eq!(mc[NodeIndex::from(11)].align, 14.into());
    assert_eq!(mc[NodeIndex::from(12)].align, 6.into());
    assert_eq!(mc[NodeIndex::from(13)].align, 7.into());
    assert_eq!(mc[NodeIndex::from(14)].align, 11.into());
    assert_eq!(mc[NodeIndex::from(15)].align, 18.into());
    assert_eq!(mc[NodeIndex::from(16)].align, 1.into());
    assert_eq!(mc[NodeIndex::from(17)].align, 19.into());
    assert_eq!(mc[NodeIndex::from(18)].align, 20.into());
    assert_eq!(mc[NodeIndex::from(19)].align, 23.into());
    assert_eq!(mc[NodeIndex::from(20)].align, 24.into());
    assert_eq!(mc[NodeIndex::from(21)].align, 12.into());
    assert_eq!(mc[NodeIndex::from(22)].align, 22.into());
    assert_eq!(mc[NodeIndex::from(23)].align, 17.into());
    assert_eq!(mc[NodeIndex::from(24)].align, 15.into());
    assert_eq!(mc[NodeIndex::from(25)].align, 9.into());
}

#[test]
fn alignment_down_left() {
    let mc = create_test_layout().mark_type_1_conflicts().create_vertical_alignments(VDir::Down, HDir::Left); 

    // block root 0
    for n in [0, 6] { assert_eq!(mc[NodeIndex::from(n)].root, 0.into()); }
    // block root 1
    for n in [1] { assert_eq!(mc[NodeIndex::from(n)].root, 1.into()); }
    // block root 2
    for n in [2] { assert_eq!(mc[NodeIndex::from(n)].root, 2.into()); }
    // block root 3
    for n in [3] { assert_eq!(mc[NodeIndex::from(n)].root, 3.into()); }
    // block root 16
    for n in [16] { assert_eq!(mc[NodeIndex::from(n)].root, 16.into()); }
    // block root 4
    for n in [4, 8] { assert_eq!(mc[NodeIndex::from(n)].root, 4.into()); }
    // block root 17
    for n in [17, 19, 23] { assert_eq!(mc[NodeIndex::from(n)].root, 17.into()); }
    // block root 18
    for n in [18, 20, 24] { assert_eq!(mc[NodeIndex::from(n)].root, 18.into()); }
    // block root 5
    for n in [5, 9, 25] { assert_eq!(mc[NodeIndex::from(n)].root, 5.into()); }
    // block root 7
    for n in [7, 11, 14] { assert_eq!(mc[NodeIndex::from(n)].root, 7.into()); }
    // block root 21
    for n in [21, 12, 15] { assert_eq!(mc[NodeIndex::from(n)].root, 21.into()); }
    // block root 10
    for n in [10, 13] { assert_eq!(mc[NodeIndex::from(n)].root, 10.into()); }
    // block root 22
    for n in [22] { assert_eq!(mc[NodeIndex::from(n)].root, 22.into()); }
}

#[test]
fn alignment_up_right() {
    let mc = create_test_layout().mark_type_1_conflicts().create_vertical_alignments(VDir::Up, HDir::Right); 

    for n in [13, 10] { assert_eq!(mc[NodeIndex::from(n)].root, 13.into()) }
    for n in [14, 11, 7] { assert_eq!(mc[NodeIndex::from(n)].root, 14.into()) }
    for n in [15, 23, 19, 17] { assert_eq!(mc[NodeIndex::from(n)].root, 15.into()) }
    for n in [22] { assert_eq!(mc[NodeIndex::from(n)].root, 22.into()) }
    for n in [24, 20, 18, 0] { assert_eq!(mc[NodeIndex::from(n)].root, 24.into()) }
    for n in [12, 21] { assert_eq!(mc[NodeIndex::from(n)].root, 12.into()) }
    for n in [25, 9, 5] { assert_eq!(mc[NodeIndex::from(n)].root, 25.into()) }
    for n in [8, 3] { assert_eq!(mc[NodeIndex::from(n)].root, 8.into()) }
    for n in [2] { assert_eq!(mc[NodeIndex::from(n)].root, 2.into()) }
    for n in [16] { assert_eq!(mc[NodeIndex::from(n)].root, 16.into()) }
    for n in [4] { assert_eq!(mc[NodeIndex::from(n)].root, 4.into()) }
    for n in [6] { assert_eq!(mc[NodeIndex::from(n)].root, 6.into()) }
    for n in [1] { assert_eq!(mc[NodeIndex::from(n)].root, 1.into()) }
}

#[test]
fn alignment_up_left() {
    let mc = create_test_layout().mark_type_1_conflicts().create_vertical_alignments(VDir::Up, HDir::Left); 

    for n in [15, 25, 9] { assert_eq!(mc[NodeIndex::from(n)].root, 15.into()) }
    for n in [14] { assert_eq!(mc[NodeIndex::from(n)].root, 14.into()) }
    for n in [13, 22] { assert_eq!(mc[NodeIndex::from(n)].root, 13.into()) }
    for n in [12, 21, 6] { assert_eq!(mc[NodeIndex::from(n)].root, 12.into()) }
    for n in [24, 20, 18] { assert_eq!(mc[NodeIndex::from(n)].root, 24.into()) }
    for n in [23, 19, 17, 1] { assert_eq!(mc[NodeIndex::from(n)].root, 23.into()) }
    for n in [11, 7] { assert_eq!(mc[NodeIndex::from(n)].root, 11.into()) }
    for n in [10] { assert_eq!(mc[NodeIndex::from(n)].root, 10.into()) }
    for n in [4, 8] { assert_eq!(mc[NodeIndex::from(n)].root, 8.into()) }
    for n in [0] { assert_eq!(mc[NodeIndex::from(n)].root, 0.into()) }
    for n in [2] { assert_eq!(mc[NodeIndex::from(n)].root, 2.into()) }
    for n in [3] { assert_eq!(mc[NodeIndex::from(n)].root, 3.into()) }
    for n in [16] { assert_eq!(mc[NodeIndex::from(n)].root, 16.into()) }
}
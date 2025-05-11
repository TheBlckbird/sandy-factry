use petgraph::{Graph, graph::NodeIndex};

pub fn get_or_create_node<N, M: PartialEq + Copy, E>(
    graph: &mut Graph<(N, M), E>,
    node: (N, &M),
) -> NodeIndex {
    graph
        .node_indices()
        .find(|&node_index| node.1 == &graph[node_index].1)
        .unwrap_or_else(|| graph.add_node((node.0, *node.1)))
}

pub fn add_edge_if_not_exists<N, M, E>(
    graph: &mut Graph<(N, M), E>,
    a: NodeIndex,
    b: NodeIndex,
    weight: E,
) {
    if graph.find_edge(a, b).is_none() {
        graph.add_edge(a, b, weight);
    }
}

use petgraph::{Graph, graph::NodeIndex};

pub fn get_or_create_node<N, E: PartialEq + Copy>(
    graph: &mut Graph<(N, E), ()>,
    node: (N, &E),
) -> NodeIndex {
    graph
        .node_indices()
        .find(|&node_index| node.1 == &graph[node_index].1)
        .unwrap_or_else(|| graph.add_node((node.0, *node.1)))
}

pub fn add_edge_if_not_exists<N, E>(graph: &mut Graph<(N, E), ()>, a: NodeIndex, b: NodeIndex) {
    if graph.find_edge(a, b).is_none() {
        graph.add_edge(a, b, ());
    }
}

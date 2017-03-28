use super::*;

#[test]
fn test_description_matrix() {
    let mut graph = AdjacencyMatrixGraph::new();

    let a = graph.create_vertex(1);
    let b = graph.create_vertex(2);
    let c = graph.create_vertex(3);

    graph.add_directed_edge(a, b, Some(1.0));
    graph.add_directed_edge(a, c, Some(1.0));
    graph.add_directed_edge(b, c, Some(1.0));

    let expected_str = " N 1.00 1.00 \n N N 1.00 \n N N N \n";
    assert_eq!(format!("{}", graph), expected_str.to_string());
}

#[test]
fn test_description_list() {
    let mut graph = AdjacencyListGraph::new();

    let a = graph.create_vertex(1);
    let b = graph.create_vertex(2);
    let c = graph.create_vertex(3);

    graph.add_directed_edge(a, b, Some(1.0));
    graph.add_directed_edge(a, c, Some(1.0));
    graph.add_directed_edge(b, c, Some(1.0));

    let expexted_str = format!("{}: ->{}(1.00) ->{}(1.00)\n{}: ->{}(1.00)\n",
                               a, b, c, b, c);
    assert_eq!(format!("{}", graph), expexted_str);
}

macro_rules! test_edge {
    ($graph_type: ident) => {
        let mut graph = $graph_type::new();
        
        let a = graph.create_vertex(1);
        let b = graph.create_vertex(2);
        let c = graph.create_vertex(3);

        graph.add_directed_edge(a, b, Some(1.0));
        graph.add_directed_edge(a, c, Some(1.0));
        graph.add_directed_edge(b, c, Some(1.0));

        let edge_ab = Edge { from: a, to: b, weight: Some(1.0) };
        let edge_ac = Edge { from: a, to: c, weight: Some(1.0) };
        let edge_bc = Edge { from: b, to: c, weight: Some(1.0) };
        
        let edges_from_a = graph.edges_from_src(a);
        assert_eq!(edges_from_a.len(), 2);
        assert_eq!(edges_from_a[0], edge_ab);
        assert_eq!(edges_from_a[1], edge_ac);

        let edges_from_b = graph.edges_from_src(b);
        assert_eq!(edges_from_b.len(), 1);
        assert_eq!(edges_from_b[0], edge_bc);

        let edges = graph.edges();
        assert_eq!(edges.len(), 3);
        assert_eq!(edges, [edge_ab, edge_ac, edge_bc]);
    }
}

#[test]
fn test_edge_matrix() {
    test_edge!(AdjacencyListGraph);
}

#[test]
fn test_edge_list() {
    test_edge!(AdjacencyListGraph);
}

macro_rules! test_edges_undirected {
    ($graph_type: ident) => {
        let mut graph = $graph_type::new();
        let a = graph.create_vertex(1);
        let b = graph.create_vertex(2);
        let c = graph.create_vertex(3);

        graph.add_undirected_edge(a, b, Some(1.0));
        graph.add_undirected_edge(a, c, Some(1.0));
        graph.add_undirected_edge(b, c, Some(1.0));

        let edge_ab = Edge { from: a, to: b, weight: Some(1.0) };
        let edge_ac = Edge { from: a, to: c, weight: Some(1.0) };
        let edge_bc = Edge { from: b, to: c, weight: Some(1.0) };
        let edge_ba = Edge { from: b, to: a, weight: Some(1.0) };
        let edge_ca = Edge { from: c, to: a, weight: Some(1.0) };
        let edge_cb = Edge { from: c, to: b, weight: Some(1.0) };

        let edges_from_a = graph.edges_from_src(a);
        assert_eq!(edges_from_a.len(), 2);
        assert_eq!(edges_from_a[0], edge_ab);
        assert_eq!(edges_from_a[1], edge_ac);

        let edges_from_c = graph.edges_from_src(c);
        assert_eq!(edges_from_c.len(), 2);
        assert_eq!(edges_from_c[0], edge_ca);
        assert_eq!(edges_from_c[1], edge_cb);

        let edges_from_b = graph.edges_from_src(b);
        assert_eq!(edges_from_b.len(), 2);
        assert_eq!(edges_from_b[0], edge_ba);
        assert_eq!(edges_from_b[1], edge_bc);

        let edges = graph.edges();
        assert_eq!(edges.len(), 6);
        assert_eq!(edges, [edge_ab, edge_ac, edge_ba, edge_bc, edge_ca, edge_cb]);
    }
}

#[test]
fn test_edges_undirected_matrix() {
    test_edges_undirected!(AdjacencyMatrixGraph);
}

#[test]
fn test_edges_undirected_list() {
    test_edges_undirected!(AdjacencyListGraph);
}

macro_rules! test_bfs {
    ($graph_type: ident) => {
        let mut graph = $graph_type::new();
        let a = graph.create_vertex(1);
        let b = graph.create_vertex(2);
        let c = graph.create_vertex(3);
        let d = graph.create_vertex(4);
        let e = graph.create_vertex(5);

        graph.add_directed_edge(a, b, Some(1.0));
        graph.add_directed_edge(a, d, Some(1.0));
        graph.add_directed_edge(b, c, Some(1.0));
        graph.add_directed_edge(c, d, Some(1.0));
        graph.add_directed_edge(d, b, Some(1.0));
        graph.add_directed_edge(d, e, Some(1.0));

        let bfs_traversed = graph.bfs_traverse(a);

        assert_eq!(bfs_traversed, [a, b, d, c, e]);
    } 
}

#[test]
fn test_bfs_matrix() {
    test_bfs!(AdjacencyMatrixGraph);
}

#[test]
fn test_bfs_list() {
    test_bfs!(AdjacencyListGraph);
}

macro_rules! test_dfs {
    ($graph_type: ident) => {
        let mut graph = $graph_type::new();
        let a = graph.create_vertex(1);
        let b = graph.create_vertex(2);
        let c = graph.create_vertex(3);
        let d = graph.create_vertex(4);
        let e = graph.create_vertex(5);

        graph.add_directed_edge(a, b, Some(1.0));
        graph.add_directed_edge(a, d, Some(1.0));
        graph.add_directed_edge(b, c, Some(1.0));
        graph.add_directed_edge(c, d, Some(1.0));
        graph.add_directed_edge(d, b, Some(1.0));
        graph.add_directed_edge(d, e, Some(1.0));

        let dfs_traversed = graph.dfs_traverse(a);

        assert_eq!(dfs_traversed, [a, b, c, d, e]);
    }
}

#[test]
fn test_dfs_matrix() {
    test_dfs!(AdjacencyMatrixGraph);
}

#[test]
fn test_dfs_list() {
    test_dfs!(AdjacencyListGraph);
}

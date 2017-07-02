extern crate stash;

use std::ops::{Index, IndexMut};
use std::collections::HashSet;
use std::collections::hash_set::Iter as HashSetIter;

use stash::Stash;
use stash::stash::Iter as StashIter;

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Node(usize);

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Edge(usize);

type EdgeList = HashSet<Edge>;
type EdgeListIter<'a> = HashSetIter<'a, Edge>;

type NodeData<N> = (EdgeList, N);

type EdgeData<E> = ((Node, Node), E);

#[derive(Clone)]
pub struct Graph<N, E> {
    nodes: Stash<NodeData<N>>,
    edges: Stash<EdgeData<E>>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph {
            nodes: Stash::new(),
            edges: Stash::new(),
        }
    }

    pub fn with_capacity(nb_nodes: usize, nb_edges: usize) -> Graph<N, E> {
        Graph {
            nodes: Stash::with_capacity(nb_nodes),
            edges: Stash::with_capacity(nb_edges),
        }
    }

    pub fn reserve(&mut self, nb_nodes: usize, nb_edges: usize) {
        self.reserve_nodes(nb_nodes);
        self.reserve_edges(nb_edges);
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }

    pub fn add_node(&mut self, node_weight: N) -> Node {
        Node(self.nodes.put((EdgeList::new(), node_weight)))
    }

    pub fn remove_node(&mut self, node: Node) {
        let (edges, _) = self.nodes.take(node.0).unwrap();

        for edge in edges {
            self.edges.take(edge.0).unwrap();
        }
    }

    pub fn reserve_nodes(&mut self, nb_nodes: usize) {
        self.nodes.reserve(nb_nodes);
    }

    pub fn nodes(&self) -> NodeIter<N> {
        NodeIter(self.nodes.iter())
    }

    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn node_weight(&self, node: Node) -> Option<&N> {
        self.nodes.get(node.0).map(|&(_, ref w)| w)
    }

    fn node_weight_mut(&mut self, node: Node) -> Option<&mut N> {
        self.nodes.get_mut(node.0).map(|&mut (_, ref mut w)| w)
    }

    pub fn add_edge(&mut self, source_node: Node, target_node: Node, edge_weight: E) -> Edge {
        let adjacency = (source_node, target_node);
        let edge = Edge(self.edges.put((adjacency, edge_weight)));

        self.nodes.get_mut(source_node.0).unwrap().0.insert(edge);
        self.nodes.get_mut(target_node.0).unwrap().0.insert(edge);

        edge
    }

    pub fn remove_edge(&mut self, edge: Edge) {
        let ((source_node, target_node), _) = self.edges.take(edge.0).unwrap();

        self.nodes.get_mut(source_node.0).unwrap().0.remove(&edge);
        self.nodes.get_mut(target_node.0).unwrap().0.remove(&edge);
    }

    pub fn reserve_edges(&mut self, nb_edges: usize) {
        self.edges.reserve(nb_edges);
    }

    pub fn edges(&self) -> EdgeIter<E> {
        EdgeIter(self.edges.iter())
    }

    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    fn edge_weight(&self, edge: Edge) -> Option<&E> {
        self.edges.get(edge.0).map(|&(_, ref w)| w)
    }

    fn edge_weight_mut(&mut self, edge: Edge) -> Option<&mut E> {
        self.edges.get_mut(edge.0).map(|&mut (_, ref mut w)| w)
    }

    pub fn adjacent_nodes(&self, node: Node, direction: Direction) -> AdjacentNodes<E> {
        AdjacentNodes {
            neighbors: self.neighbors(node, direction),
        }
    }

    pub fn incoming_nodes(&self, node: Node) -> AdjacentNodes<E> {
        self.adjacent_nodes(node, Direction::Incoming)
    }

    pub fn outgoing_nodes(&self, node: Node) -> AdjacentNodes<E> {
        self.adjacent_nodes(node, Direction::Outgoing)
    }

    pub fn adjacent_edges(&self, node: Node, direction: Direction) -> AdjacentEdges<E> {
        AdjacentEdges {
            neighbors: self.neighbors(node, direction),
        }
    }

    pub fn incoming_edges(&self, node: Node) -> AdjacentEdges<E> {
        self.adjacent_edges(node, Direction::Incoming)
    }

    pub fn outgoing_edges(&self, node: Node) -> AdjacentEdges<E> {
        self.adjacent_edges(node, Direction::Outgoing)
    }

    fn neighbors(&self, node: Node, direction: Direction) -> Neighbors<E> {
        let &(ref edges, _) = self.nodes.get(node.0).unwrap();

        Neighbors {
            direction: direction,
            edges: &self.edges,
            iter: edges.iter(),
            node: node,
        }
    }

    pub fn edge_nodes(&self, edge: Edge) -> (Node, Node) {
        self.edges.get(edge.0).map(|&(adj, _)| adj).unwrap()
    }

    pub fn edge_source(&self, edge: Edge) -> Node {
        self.edge_nodes(edge).0
    }

    pub fn edge_target(&self, edge: Edge) -> Node {
        self.edge_nodes(edge).1
    }
}

impl<N, E> Index<Node> for Graph<N, E> {
    type Output = N;

    fn index(&self, node: Node) -> &N {
        self.node_weight(node).unwrap()
    }
}

impl<N, E> IndexMut<Node> for Graph<N, E> {
    fn index_mut(&mut self, node: Node) -> &mut N {
        self.node_weight_mut(node).unwrap()
    }
}

pub struct NodeIter<'a, N: 'a>(StashIter<'a, NodeData<N>, usize>);

impl<'a, N> Iterator for NodeIter<'a, N> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(i, _)| Node(i))
    }
}

impl<N, E> Index<Edge> for Graph<N, E> {
    type Output = E;

    fn index(&self, edge: Edge) -> &E {
        self.edge_weight(edge).unwrap()
    }
}

impl<N, E> IndexMut<Edge> for Graph<N, E> {
    fn index_mut(&mut self, edge: Edge) -> &mut E {
        self.edge_weight_mut(edge).unwrap()
    }
}

pub struct EdgeIter<'a, E: 'a>(StashIter<'a, EdgeData<E>, usize>);

impl<'a, E> Iterator for EdgeIter<'a, E> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(i, _)| Edge(i))
    }
}

struct Neighbors<'a, E: 'a> {
    direction: Direction,
    edges: &'a Stash<EdgeData<E>>,
    iter: EdgeListIter<'a>,
    node: Node,
}

impl<'a, E> Iterator for Neighbors<'a, E> {
    type Item = (Edge, Node);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(&e) = self.iter.next() {
            // note: using unwrap should be safe here
            let &((source_node, target_node), _) = self.edges.get(e.0).unwrap();

            let other_node = match self.direction {
                Direction::Incoming => source_node,
                Direction::Outgoing => target_node,
            };

            if self.node != other_node {
                return Some((e, other_node));
            }
        }

        None
    }
}

pub struct AdjacentNodes<'a, E: 'a> {
    neighbors: Neighbors<'a, E>,
}

impl<'a, E> Iterator for AdjacentNodes<'a, E> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.neighbors.next().map(|(_, n)| n)
    }
}

pub struct AdjacentEdges<'a, E: 'a> {
    neighbors: Neighbors<'a, E>,
}

impl<'a, E> Iterator for AdjacentEdges<'a, E> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        self.neighbors.next().map(|(e, _)| e)
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Incoming,
    Outgoing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add_nodes() {
        let mut g = Graph::<(), ()>::new();

        let _ = g.add_node(());
        let _ = g.add_node(());

        assert_eq!(g.nb_nodes(), 2);
    }

    #[test]
    fn it_can_get_nodes() {
        let mut g = Graph::<&str, ()>::new();

        let a = g.add_node("a");
        let b = g.add_node("b");

        assert_eq!(g[a], "a");
        assert_eq!(g[b], "b");
    }

    #[test]
    fn it_can_remove_nodes() {
        let mut g = Graph::<&str, ()>::new();

        let a = g.add_node("a");
        let b = g.add_node("b");

        g.remove_node(a);

        assert_eq!(g[b], "b");
        assert_eq!(g.nb_nodes(), 1);
    }

    #[test]
    fn it_can_add_edges() {
        let mut g = Graph::<(), ()>::new();

        let a = g.add_node(());
        let b = g.add_node(());

        let _ = g.add_edge(a, b, ());

        assert_eq!(g.nb_edges(), 1);
    }

    #[test]
    fn it_can_get_edges() {
        let mut g = Graph::<&str, &str>::new();

        let a = g.add_node("a");
        let b = g.add_node("b");

        let ab = g.add_edge(a, b, "ab");

        assert_eq!(g[ab], "ab");
    }

    #[test]
    fn it_can_remove_edges() {
        let mut g = Graph::<(), ()>::new();

        let a = g.add_node(());
        let b = g.add_node(());

        let ab = g.add_edge(a, b, ());
        let _ = g.add_edge(b, a, ());

        g.remove_edge(ab);

        assert_eq!(g.nb_edges(), 1);
    }

    #[test]
    fn it_removes_edges_when_removing_nodes() {
        let mut g = Graph::<&str, ()>::new();

        let a = g.add_node("a");
        let b = g.add_node("b");

        let _ = g.add_edge(a, b, ());

        g.remove_node(a);

        assert_eq!(g.nb_edges(), 0);
    }

    #[test]
    fn it_can_iterate_on_nodes() {
        let mut g = Graph::<&str, &str>::new();

        let b = g.add_node("b");
        let d = g.add_node("d");
        let a = g.add_node("a");
        let c = g.add_node("c");

        let nodes = {
            let mut nodes: Vec<_> = g.nodes().collect();
            nodes.sort();
            nodes
        };

        let expected_nodes = {
            let mut nodes = vec![a, b, c, d];
            nodes.sort();
            nodes
        };

        assert_eq!(nodes, expected_nodes);
    }

    #[test]
    fn it_can_iterate_on_edges() {
        let mut g = Graph::<&str, &str>::new();

        let b = g.add_node("b");
        let d = g.add_node("d");
        let c = g.add_node("c");
        let a = g.add_node("a");

        let cd = g.add_edge(c, d, "cd");
        let ab = g.add_edge(a, b, "ab");
        let ca = g.add_edge(c, a, "ca");
        let ad = g.add_edge(a, d, "ad");
        let bc = g.add_edge(b, c, "bc");

        let edges = {
            let mut edges: Vec<_> = g.edges().collect();
            edges.sort();
            edges
        };

        let expected_edges = {
            let mut edges = vec![cd, ab, ca, ad, bc];
            edges.sort();
            edges
        };

        assert_eq!(edges, expected_edges);
    }

    #[test]
    fn it_can_get_neighbors() {
        let mut g = Graph::<&str, &str>::new();

        let a = g.add_node("a");
        let b = g.add_node("b");
        let c = g.add_node("c");
        let d = g.add_node("d");

        let ab = g.add_edge(a, b, "ab");
        let bc = g.add_edge(b, c, "bc");
        let cd = g.add_edge(c, d, "cd");
        let ad = g.add_edge(a, d, "ad");
        let ca = g.add_edge(c, a, "ca");

        let g = g; // freeze for safety

        macro_rules! adjacent_edges {
            ($n:ident, $d:tt $(, $v:expr)*) => {
                let adjacent_edges = {
                    let mut adjacent_edges: Vec<_> = g.adjacent_edges($n, Direction::$d).collect();
                    adjacent_edges.sort();
                    adjacent_edges
                };

                let expected_neighbors = {
                    let mut adjacent_edges = vec![$( $v, )*];
                    adjacent_edges.sort();
                    adjacent_edges
                };

                assert_eq!(adjacent_edges, expected_neighbors);
            }
        }

        adjacent_edges!(a, Outgoing, ab, ad);
        adjacent_edges!(a, Incoming, ca);
        adjacent_edges!(b, Outgoing, bc);
        adjacent_edges!(b, Incoming, ab);
        adjacent_edges!(c, Outgoing, ca, cd);
        adjacent_edges!(c, Incoming, bc);
        adjacent_edges!(d, Outgoing);
        adjacent_edges!(d, Incoming, ad, cd);
    }
}

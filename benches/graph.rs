#![feature(test)]

extern crate test;

extern crate petgraph;
extern crate stash_graph;

use petgraph::Graph as BestGraph;
use petgraph::graphmap::DiGraphMap as AdjacencyListGraph;
use stash_graph::Graph as StashGraph;

macro_rules! benchs {
    ($g:ident, $m:ident) => {
        mod $m {
            use super::*;

            #[bench]
            fn add_1000_nodes(b: &mut test::Bencher) {
                b.iter(|| {
                    let mut g = $g::<(), ()>::new();

                    for _ in 0..1000 {
                        let _ = g.add_node(());
                    }
                });
            }

            #[bench]
            fn add_1000_edges_to_self(b: &mut test::Bencher) {
                let mut g = $g::<(), ()>::new();
                let nodes: Vec<_> = (0..1000).map(|_| g.add_node(())).collect();
                let g = g;

                b.iter(|| {
                    let mut g = g.clone();

                    for &node in nodes.iter() {
                        let _ = g.add_edge(node, node, ());
                    }
                });
            }

            #[bench]
            fn add_5_edges_for_each_of_1000_nodes(b: &mut test::Bencher) {
                let mut g = $g::<(), ()>::new();
                let nodes: Vec<_> = (0..1000).map(|_| g.add_node(())).collect();
                let g = g;

                let edges_to_add: Vec<_> = nodes.iter()
                    .enumerate()
                    .map(|(i, &node)| {
                        let edges: Vec<_> = (0..5)
                            .map(|j| (i + j + 1) % nodes.len())
                            .map(|j| (node, nodes[j]))
                            .collect();

                        edges
                    })
                    .flat_map(|e| e)
                    .collect();

                b.iter(|| {
                    let mut g = g.clone();

                    for &(source, target) in edges_to_add.iter() {
                        let _ = g.add_edge(source, target, ());
                    }
                });
            }

            // FIXME: does not match GraphMap's interface
            // #[bench]
            // fn remove_1000_edges(b: &mut test::Bencher) {
            //     let mut g = $g::<(), ()>::new();
            //     let nodes: Vec<_> = (0..1000).map(|_| g.add_node(())).collect();
            //     let edges: Vec<_> = nodes.iter().map(|&n| g.add_edge(n, n, ())).collect();
            //     let g = g;

            //     b.iter(|| {
            //         let mut g = g.clone();

            //         for &edge in edges.iter() {
            //             g.remove_edge(edge);
            //         }
            //     });
            // }
        }
    }
}

benchs!(BestGraph, best);
benchs!(AdjacencyListGraph, adj_list);
benchs!(StashGraph, stash);

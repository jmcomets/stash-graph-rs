#![feature(test)]

extern crate test;
extern crate petgraph;
extern crate stash_graph;

use petgraph::Graph as BaseGraph;
use stash_graph::Graph as StashGraph;

#[bench]
fn add_1000_nodes_theirs(b: &mut test::Bencher) {
    b.iter(|| {
        let mut g = BaseGraph::<(), ()>::new();

        for _ in 0..1000 {
            let _ = g.add_node(());
        }
    });
}

#[bench]
fn add_1000_nodes_mine(b: &mut test::Bencher) {
    b.iter(|| {
        let mut g = StashGraph::<(), ()>::new();
        g.reserve_nodes(1000);

        for _ in 0..1000 {
            let _ = g.add_node(());
        }
    });
}

#[bench]
fn push_1000_node_datas_to_a_vec(b: &mut test::Bencher) {
    use std::collections::HashSet;

    b.iter(|| {
        let mut v = Vec::<(HashSet<usize>, ())>::new();
        v.reserve(1000);

        for _ in 0..1000 {
            v.push((HashSet::new(), ()));
        }
    });
}

#[bench]
fn push_1000_optional_node_datas_to_a_vec(b: &mut test::Bencher) {
    use std::collections::HashSet;

    b.iter(|| {
        let mut v = Vec::<(Option<HashSet<usize>>, ())>::new();
        v.reserve(1000);

        for _ in 0..1000 {
            v.push((None, ()));
        }
    });
}

#[bench]
fn creating_1000_hash_sets(b: &mut test::Bencher) {
    use std::collections::HashSet;

    b.iter(|| {
        for _ in 0..1000 {
            let _ = HashSet::<()>::new();
        }
    });
}

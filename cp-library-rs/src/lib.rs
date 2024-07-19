#![allow(non_snake_case)]
#![allow(dead_code)]

/// 代数的構造
pub mod algebraic_structure {
    pub mod affine1d;
    pub mod affine2d;
    pub mod extmonoid;
    pub mod extmonoid_mod;
    pub mod monoid;
    pub mod monoid_examples;
    pub mod monoid_mod;
}

/// データ構造
pub mod data_structure {
    pub mod acc2d;
    pub mod acc2d_cyclic;
    pub mod bit;
    pub mod bit_2d;
    pub mod bitset;
    pub mod dual_segment_tree;
    pub mod dynamic_segment_tree;
    pub mod indexedset;
    pub mod lazy_segment_tree;
    pub mod mex_set;
    pub mod multiset;
    pub mod multiset_splay_tree;
    pub mod rollback_unionfind;
    pub mod segment_tree;
    pub mod segment_tree_2d;
    pub mod segment_tree_mutable;
    pub mod segmented_sieve;
    pub mod sparse_table;
    pub mod union_find;
    pub mod weighted_union_find;
}

/// 幾何
pub mod geometry {
    pub mod vec2;
}

/// グラフ
pub mod graph {
    pub mod centroid;
    pub mod dijkstra;
    pub mod euler_tour;
    pub mod ford_fulkerson;
    pub mod lca_doubling;
    pub mod loop_detection;
    pub mod loop_detection_fold;
    pub mod namori;
    pub mod rerooting;
    pub mod scc;
    pub mod simple_graph;
}

/// 線形代数
pub mod linear_algrebra {
    pub mod matrix_exp;
}

/// 数論
pub mod number_theory {
    pub mod comb;
    pub mod comb_no_mod;
    pub mod crt;
    pub mod factorize;
    pub mod factorize_fast;
    pub mod factors_all;
    pub mod frac;
    pub mod miller_rabin_test;
    pub mod modint;
    pub mod modint_comb;
    pub mod modint_for_rollinghash;
    pub mod modint_traits;
    pub mod pollard_rho_algorithm;
    pub mod powmod;
}

/// 文字列
pub mod string {
    pub mod lcs;
    pub mod rolling_hash;
    pub mod suffix_array;
}

/// 木
pub mod tree {
    pub mod trie;
}

/// ツール
pub mod utils {
    pub mod consts;
    pub mod coordinate_compression;
    pub mod enum_pairs;
    pub mod grid;
    pub mod iterutil;
    pub mod lineartime_merging;
    pub mod run_length;
    pub mod yesno;
    pub mod zigzag;
}

// macro
mod cfor;
mod chmax;
mod chmin;
mod debug;
mod debug2D;
mod get;

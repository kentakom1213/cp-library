//! # `cp-library-rs`
//!
//! A competitive programming library for Rust.
//!
//! ## Modules
//!
//! - [`algebraic_structure`]: algebraic traits used by generic data structures.
//! - [`convolution`]: convolution and NTT utilities.
//! - [`data_structure`]: standard data structures such as segment trees and union-find.
//! - [`geometry`]: geometric primitives and helpers.
//! - [`graph`]: graph algorithms and tree decompositions.
//! - [`linear_algrebra`]: matrix-based linear algebra utilities.
//! - [`number_theory`]: modular arithmetic, factorization, and related algorithms.
//! - [`string`]: string algorithms such as Z-algorithm and suffix array.
//! - [`tree`]: internal tree utilities and arena allocation helpers.
//! - [`utils`]: small helpers frequently used in contests.

#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod algebraic_structure;
pub mod convolution;
pub mod data_structure;
pub mod geometry;
pub mod graph;
pub mod linear_algrebra;
pub mod number_theory;
pub mod string;
pub mod tree;
pub mod utils;

// macro
mod cfor;
mod chmax;
mod chmin;
mod debug;
mod debug2D;
mod get;
mod in_ex;

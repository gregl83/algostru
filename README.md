[![Crates.io](https://img.shields.io/crates/v/algostru.svg)](https://crates.io/crates/algostru)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/algostru/blob/master/LICENSE)
[![Build Status](https://github.com/gregl83/algostru/workflows/CI/badge.svg?branch=main)](https://github.com/gregl83/algostru/actions?query=workflow%3ACI+branch%3Amain)
# algostru

Algorithms (algo) and Data Structures (stru) using Rust

## Stability

Experimental

Demonstration of Asymptotic Notation without granular scrutiny on operation specific performance.

## Contents

This repository is comprised of a command line interface tool and library of algorithms and data structures.

### Command Line Interface (CLI)

CLI tool *might* grow into toolset to aid with algorithm and data structure implementations.

For now, there is simply a **Big-O Cheatsheet** rendered in your terminal using ASCII.

Either `cargo run` or `cargo install` should work fine from repository root.

### Library

- Arithmetic
  - Multiplication
    - [Karatsuba](/src/arithmetic/multiplication/karatsuba.rs)
- Collections
  - Multiplication
    - [Strassen](/src/collections/multiplication/strassen.rs)
  - Ordering
    - [Bubble Sort](/src/collections/ordering/bubble_sort.rs)
    - [Insertion Sort](/src/collections/ordering/insertion_sort.rs)
    - [Merge Sort](/src/collections/ordering/merge_sort.rs)
    - [Merge Sort Inversion Count](/src/collections/ordering/merge_sort_inversion_count.rs)
    - [Quick Sort](/src/collections/ordering/quick_sort.rs)
    - [Selection Sort](/src/collections/ordering/selection_sort.rs)
  - Search
    - [Closest Pair](/src/collections/search/closest_pair.rs)

## Usage

See [docs.rs/algostru](https://docs.rs/algostru/).

## References

- [Stanford Algorithms Specialization](https://www.coursera.org/specializations/algorithms)
- [Big-O Cheatsheet](https://www.bigocheatsheet.com/)

## License

[MIT](LICENSE)

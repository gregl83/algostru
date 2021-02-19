[![Crates.io](https://img.shields.io/crates/v/algostru.svg)](https://crates.io/crates/algostru)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/algostru/blob/master/LICENSE)
[![Build Status](https://github.com/gregl83/algostru/workflows/CI/badge.svg?branch=main)](https://github.com/gregl83/algostru/actions?query=workflow%3ACI+branch%3Amain)

<p align="center"><img src="/assets/algostru.png" /></p>

# algostru

Algorithms and Structures powered by Rust.

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
    - [Karatsuba](/src/arithmetic/multiplication/karatsuba.rs) *O(n^log3)*
- Collections
  - Multiplication
    - [Strassen](/src/collections/multiplication/strassen.rs) *O(n^log7)*
  - Ordering
    - [Bubble Sort](/src/collections/ordering/bubble_sort.rs) *O(n^2)*
    - [Insertion Sort](/src/collections/ordering/insertion_sort.rs) *O(n^2)*
    - [Merge Sort](/src/collections/ordering/merge_sort.rs) *O(n log(n))*
    - [Merge Sort Inversion Count](/src/collections/ordering/merge_sort_inversion_count.rs) *O(n log(n))*
    - [Quick Sort](/src/collections/ordering/quick_sort.rs) *O(n log(n))*
    - [Selection Sort](/src/collections/ordering/selection_sort.rs) *O(n^2)*
    - [Tim Sort](/src/collections/ordering/tim_sort.rs) *O(n log(n))*
  - Search
    - [Closest Pair](/src/collections/search/closest_pair.rs) *O(n log(n))*
- Tries
  - [Binary Search Tree](/src/tries/binary_search_tree.rs)
  - [Merkle Tree](/src/tries/merkle_tree.rs)
  - [Radix Tree](/src/tries/radix_tree.rs)

## Usage

See [docs.rs/algostru](https://docs.rs/algostru/).

## References

- [Stanford Algorithms Specialization](https://www.coursera.org/specializations/algorithms)
- [Mathematics for Computer Science](http://courses.csail.mit.edu/6.042/spring18/mcs.pdf)
- [Mathematics for Computer Science Video Lectures](https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-042j-mathematics-for-computer-science-fall-2010/video-lectures/)
- [Big-O Cheatsheet](https://www.bigocheatsheet.com/)

## License

[MIT](LICENSE)

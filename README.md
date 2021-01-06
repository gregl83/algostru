# algostru

Algorithms (algo) and Data Structures (stru)

## Stability

Experimental

Demonstration of Asymptotic Notation without granular scrutiny on operation specific performance.

## Contents

This repository is comprised of a command line interface tool and library of algorithms and data structures.

### Command Line Interface (CLI)

CLI tool *might* grow into toolset to aid with algorithm and data structure implementations.

For now, there is simply a Big-O Cheatsheet rendered in your terminal using ASCII.

Either `cargo run` or `cargo install` should work fine from repository root.

### Library

- Math
  - Multiplication
    - [Karatsuba](/src/math/multiplication/karatsuba.rs)
    - [Strassen](/src/math/multiplication/strassen.rs)
- Collections
  - Ordering
    - [Bubble Sort](/src/collections/ordering/bubble_sort.rs)
    - [Merge Sort](/src/collections/ordering/merge_sort.rs)
    - [Merge Sort Inversion Count](/src/collections/ordering/merge_sort_inversion_count.rs)
    - [Quick Sort](/src/collections/ordering/quick_sort.rs)

## References

- [Stanford Algorithms Specialization](https://www.coursera.org/specializations/algorithms)

## License

[MIT](LICENSE)

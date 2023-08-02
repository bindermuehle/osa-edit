# Osa Library

The Osa library is a Rust implementation of the Optimal String Alignment (OSA) algorithm, a dynamic programming algorithm that computes the minimum edit distance between two strings. The library provides an implementation of the algorithm that can handle both vectors and grids, as well as several options for customizing the cost of insertions, deletions, substitutions, and transpositions.

## Installation

To use this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
osa = "0.2.0"
```

## Usage

To use the library, you can import the necessary modules and functions as follows:

```rust
use osa::{Matrix, Osa};
use osa::grid::GridMatrix;
use osa::vec::VecMatrix;
use osa::EditType;
use osa::MatrixType;
use osa::Options;
use osa::get_osa;
use osa::get_script;
use osa::get_matrix;
```

### Types

The library defines several types:

- `Matrix`: A trait that represents a two-dimensional matrix that can be used with the OSA algorithm. It provides methods for getting and setting values in the matrix.
- `GridMatrix`: A struct that implements the `Matrix` trait and represents a two-dimensional grid of values.
- `VecMatrix`: A struct that implements the `Matrix` trait and represents a one-dimensional vector of values.
- `EditType`: An enum that represents the type of edit needed to transform one string into another. The possible values are `Insert`, `Delete`, `Sub`, `Equal`, and `Transpose`.
- `MatrixType`: An enum that represents the type of matrix to use in the OSA algorithm. The possible values are `Vec` and `Grid`.

### Functions

The library provides several functions:

- `get_osa`: Given two strings and an options struct, returns an `Osa` instance that can be used to compute the minimum edit distance between the two strings. The type parameter `T` specifies the type of matrix to use (`VecMatrix` or `GridMatrix`).
- `get_script`: Given two strings and an options struct, returns an `EditScript` that represents the minimum edit distance between the two strings.
- `get_matrix`: Given two strings and an options struct, returns the `Matrix` instance used to compute the minimum edit distance between the two strings.

### Options

The `Options` struct contains the following fields:

- `ins_cost`: The cost of an insertion operation (default is 1).
- `del_cost`: The cost of a deletion operation (default is 1).
- `sub_cost`: The cost of a substitution operation (default is 1).
- `transp_cost`: The cost of a transposition operation (default is 1).
- `equals`: A function that returns true if two characters are considered equal (default is a function that returns true if the characters are identical).

### Examples

Here is an example of how to use the library to compute the minimum edit distance between two strings:

```rust
let source = "kitten";
let target = "sitting";
let options = Options {
    ins_cost: 1,
    del_cost: 1,
    sub_cost: 1,
    transp_cost: 1,
    equals: |a, b| a == b,
};
let osa: Osa<GridMatrix> = get_osa(source, target, options);
let distance = osa.distance();
assert_eq!(distance, 3);
```

This code creates an `Osa` instance for the two strings "kitten" and "sitting", using a `GridMatrix
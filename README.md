# Crunchy

Crunchy is a generic Rust library for lattice gauge theory and related lattice-model computations.

It currently provides:

- Generic periodic cubical lattices in arbitrary dimensions
- Fields on vertices, edges, faces, and cubes
- Connectivity between lattice cells
- N-dimensional array and indexing utilities
- Reusable simulation types and Metropolis updates

## Getting started

This project currently uses unstable Rust features, so a nightly toolchain is required.

```bash
cargo +nightly test
```

Run the included 2D Ising model example with:

```bash
cargo +nightly run --example ising_model --release
```

To define a model, implement `Field`/`UpdateField` for its degrees of freedom and `CubicalFields` for its lattice field content. See [`examples/ising_model.rs`](examples/ising_model.rs) for a complete example.

## Status

Crunchy is experimental and under active development; APIs may change.

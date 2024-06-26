# Computation Graph Library

A simple computation graph library in Rust, designed for easy creation, manipulation, and evaluation of computational graphs.
This library supports basic arithmetic operations, hints for complex computations, and constraint checking.

## Features

- Define and initialize nodes in the computation graph.
- Support for basic arithmetic operations: addition and multiplication.
- Ability to set constant values for nodes.
- Hinting mechanism for complex computations like division and square roots.
- Constraint checking to ensure correctness of computations.

## Example

Here’s a basic example demonstrating how to use the library:

```rust
use computation_graph::Builder;

fn main() {
    let mut builder = Builder::new();

    // Example: f(x) = x^2 + x + 5
    let x = builder.init();
    let x_squared = builder.mul(&x, &x);
    let x_squared_plus_x = builder.add(&x_squared, &x);
    let five = builder.constant(5);
    let y = builder.add(&x_squared_plus_x, &five);

    let inputs = vec![Some(3)];
    builder.fill_nodes(inputs);
    assert!(builder.check_constraints());
}
```

Tests

    cargo test



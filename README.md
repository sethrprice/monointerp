# Introduction

This is a replica of JohannesBuchner's fast linear interpolation C library in Rust, only without the integration. 

Like numpy.interp or Matlab's interp1, it takes a query vec, an x vec (coordinates), and a y vec (values) and returns a Result containing a vec of y values that correspond to the query points. Unlike numpy.interp and interp1, it assumes the x coordinate vec is ordered, which avoids the need to run a binary search every iteration. Instead, the binary search is run once and then we assume the next coordinate is close to the original. 

As the original author says, "this makes the interpolation very fast".

It also cannot extrapolate outide the range of x and will return an error if you try.

# Usage

This is an example usage in which we create an x-axis, some y values (in this rather uninteresting case y = 2x + 1), and a some query coordinates (halfway between the x points), and find the interpolated values at those query coordinates.

```rust
let x: Vec<f64> = (0..100).map(|val| val as f64).collect();
let y: Vec<f64> = x.iter().map(|val| 2. * val + 1.).collect();
let query: Vec<f64> = x.iter().map(|val| val + 0.5).take(x.len() - 1).collect();
let interpolated_values = monointerp(query, x, y);
```

# Availability

Not currently available on crates.io or anything like that.

# Dependencies

This library relies on the following dependencies, which are already in `Cargo.toml`:

- **[num-traits](https://crates.io/crates/num-traits) 0.2.19

To install dependencies, run 

```bash
cargo build
```
# zkllvm-rslang-types

This library defines zkLLVM field type wrappers with implemented external traits.

For more info about zkLLVM visit [homepage][zkllvm].

This library is supposed to be built **only** with [`rslang`][zkllvm-rslang] compiler (fork of Rust compiler).
Installation instructions may be found [here][rust-toolchain].

Since `rslang`-defined types are used, compiling with original Rust compiler will naturally lead to an error.
The same will happened if one is trying to build for any target different from `assigner-unknown-unknown`.
Right now this will lead to an unpleasent segmentation fault, but the fix is comming up soon ([tracking issue][segfault-tracking-issue]).

## Motivation

The main reason to use type wrappers is traits: it is not allowed to implement external traits on built-in types.
Thus to avoid forking a large number of popular public crates and implement their traits at their own crates, this library was created.

To control which traits are implemented one can use crate features.
For now available: `hash`, `ord`, `iter`, `int-conversions`, `num-traits`.

Arithmetic and formatting traits are implemented by default.

## Usage

To add `zkllvm-rslang-types` to your dependencies, add this to your `Cargo.toml`:

```toml
zkllvm-rslang-types = "0.1.0"
```

If you want to add e.g. implementations of integer conversions, add this:

```toml
zkllvm-rslang-types = { version = "0.1.0", features = [ "int-conversions" ]}
```

## Example

Type wrappers fully support same operations that built-in types do.
The only difference comes across when instantiating variable from a literal:

```rust
let x: PallasBase = 1g.into();
```

Since field literals (e.g. `1g`) are used to define built-in types,
one have to explicitly convert them into type wrappers with `into`.

Alternatively one can explicitly wrap the literal:

```rust
let x = PallasBase(1g);
```

[zkllvm]: https://github.com/NilFoundation/zkllvm
[zkllvm-rslang]: https://github.com/NilFoundation/zkllvm-rslang
[rust-toolchain]: https://github.com/NilFoundation/zkllvm#rust-toolchain
[segfault-tracking-issue]: https://github.com/NilFoundation/zkllvm-rslang/issues/11

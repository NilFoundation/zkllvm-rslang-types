# zkllvm-rslang-types-pre

**Warning: This is a temporary pre-release version, created only to test uploading to [crates.io](crates.io).**

This library defines zkLLVM field type wrappers with implemented external traits.

This library is supposed to be built **only** with [`rslang`](https://github.com/NilFoundation/zkllvm-rslang) compiler (fork of Rust compiler). Visit [zkLLVM](https://github.com/NilFoundation/zkllvm) repository to get installation instructions.

Since `rslang`-defined types are used, compiling with original Rust compiler will naturally lead to an error.

## Motivation

The main reason to use type wrappers is traits: it is not allowed to implement external traits on built-in types. Thus to avoid forking a large number of popular public crates and implement their traits at their own crates, this library was created.

To control which traits are implemented one can use crate features. For now available: `hash`, `ord`, `iter`, `int-conversions`, `num-traits`.

Arithmetic and formatting traits are implemented by default.

## Usage

To add `zkllvm-rslang-types-pre` to your dependencies, add this to your `Cargo.toml`:

```
zkllvm-rslang-types-pre = { git = "https://github.com/NilFoundation/zkllvm-rslang-types-pre.git", branch = "master" }
```

If you want to add e.g. implementations of integer conversions, add this:

```
zkllvm-rslang-types-pre = { git = "https://github.com/NilFoundation/zkllvm-rslang-types-pre.git", branch = "master", features = [ "int-conversions" ]}
```

## Example

Type wrappers fully support same operations that built-in types do. The only difference comes across when instantiating variable from a literal:

```rust
let x: PallasBase = 1g.into();
```

Since field literals (e.g. `1g`) are used to define built-in types, one have to explicitly convert them into type wrappers with `into`.

Alternatively one can explicitly wrap the literal:

```rust
let x = PallasBase(1g);
```

## Building examples

To build examples correctly you need to use `-C link-dead-code`:

```
cargo +zkllvm rustc --target assigner-unknown-unknown --example arithmetics -- -C link-dead-code
```

Otherwise dead code elimination will omit most of the instructions in example functions.

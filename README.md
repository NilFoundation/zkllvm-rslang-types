# zktypes

This library defines zkLLVM type wrappers with implemented external traits.

**Warning:** building `--release` is unstable now.

## Examples

To build examples correctly you need to use `-C link-dead-code`:

```
cargo rustc --target assigner-unknown-unknown --example arithmetics -- -C link-dead-code
```

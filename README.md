# FIELD TRACKER

An easy tool to enable measuring the number of operations in your protocol

### SET UP

Add the rust code below to your cargo.toml

```rust
[dev-dependencies]
field-tracker = { git = "https://github.com/sublinearlabs/field-tracker", branch = "main" }
```

In your code
```rust
use field_tracker::Ft;
use ark_bn245::Fq;

type Fq = Ft<4, Fq>
```
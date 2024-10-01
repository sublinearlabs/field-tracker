# FIELD TRACKER

An easy tool to enable measuring the number of operations in your protocol

### SET UP

 - Add field tracker to cargo.toml

```rust
[dev-dependencies]
field-tracker = { git = "https://github.com/sublinearlabs/field-tracker", branch = "main" }
```

- Initialize the field tracker type
```rust
use field_tracker::Ft;
use ark_bn245::Fq;

type Fq = Ft<4, Fq>
```

- Call summary to get a summary of the operations.

```rust
println!("{}", Fq::summary());
```


### Example
```rust
use field_tracker::Ft;

type Fq = Ft<4, ark_bn245::Fq>

Fq::from(3) + Fq::from(5);

println!("{}", Fq::summary());
```
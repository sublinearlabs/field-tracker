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
use field_tracker::{Ft, start_tscope, end_tscope, print_summary, summary};

type Fq = Ft!(ark_bn245::Fq);
```

- Call the summary macro to return a summary

```rust
let summary = summary!();
```

- Call the print_summary macro to print a summary to the terminal

```rust
print_summary!();
```

- The start_tscope macro can be used to get more context by namespacing the operations done from where it is called until an end_tscope macro is seen. It takes a name for the namespace as an input.

```rust
start_tscope!("Sumcheck");
```

- The end_tscope macro is used to end a namespace summary. It should only be called if a start_tscope was called.
```rust
end_tscope!();
```


### Example
```rust
use field_tracker::{Ft, start_tscope, end_tscope, print_summary, summary};

type Fq = Ft!(ark_bn245::Fq);

let mut a = Fq::from(3)
let mut b = Fq::from(5);
let mut c = a + b;

start_tscope!("Layer 2");

a += c;
b += c;
d = a * b;

end_tscope!();

d -= c;

print_summary!();
```

# Getting Started

Add BIC as a dependency:

```toml
[dependencies]
bic = { path = "../bic" }
```

## Basic usage

```rust
use bic::raw_headers::HeaderConfig;

let result = HeaderConfig::new()
    .header("mylib.h")
    .process()
    .unwrap();

for item in &result.package.items {
    println!("{:?}", item);
}
```

## With include directories and defines

```rust
let result = HeaderConfig::new()
    .header("api.h")
    .include_dir("/usr/local/include")
    .define("VERSION", Some("2".into()))
    .process()
    .unwrap();
```

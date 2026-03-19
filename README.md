# BIC (bind c)

C binding and ABI extraction layer on top of [PAC](https://github.com/bresilla/pac).

Parses C headers, extracts function signatures, types, and symbols into a structured IR,
and generates Rust FFI bindings or JSON output.

## Usage

```rust
use bic::raw_headers::HeaderConfig;
use bic::emit_rust_ffi;

let result = HeaderConfig::new()
    .header("mylib.h")
    .include_dir("/usr/local/include")
    .process()
    .unwrap();

let rust_ffi = emit_rust_ffi(&result.package);
println!("{}", rust_ffi);
```

## Building

```sh
make build
make test
```

## License

Dual-licensed under Apache 2.0 or MIT (see `LICENSE-APACHE` and `LICENSE-MIT`).

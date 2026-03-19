# Code Generation

`emit_rust_ffi` converts a `BindingPackage` into Rust FFI code.

```rust
let rust_code = bic::emit_rust_ffi(&result.package);
```

## Output format

The generated code includes:
- `extern "C"` blocks with function declarations
- `#[repr(C)]` struct and union definitions
- Type aliases
- Enum definitions with explicit discriminant values

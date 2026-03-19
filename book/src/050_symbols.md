# Symbol Validation

BIC can inspect ELF and Mach-O binaries and validate extracted bindings against actual symbols.

## Inspecting symbols

```rust
let inventory = bic::inspect_symbols("libz.so").unwrap();
```

Each `SymbolEntry` contains: name, visibility, binding (Local/Global/Weak), size, and section.

## Supported formats

| Format | Extensions | Artifact types |
|--------|-----------|----------------|
| ELF | `.o`, `.a`, `.so` | Object, static library, shared library |
| Mach-O | `.o`, `.a`, `.dylib` | Object, static library, dynamic library |

Mach-O symbol names have their leading `_` prefix stripped automatically to match C identifier names from headers.

## Validating bindings

```rust
let report = bic::validate(&result.package, &inventory);

println!("matched: {}", report.matched().len());
println!("missing: {}", report.missing().len());
```

The validation report classifies each binding as Matched, Missing, Hidden, WeakMatch, or type-mismatched (NotAFunction, NotAVariable).

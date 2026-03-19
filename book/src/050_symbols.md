# Symbol Validation

BIC can inspect ELF shared libraries and validate extracted bindings against actual symbols.

## Inspecting symbols

```rust
let inventory = bic::inspect_symbols("libz.so").unwrap();
```

Each `SymbolEntry` contains: name, visibility, binding (Local/Global/Weak), size, and section.

## Validating bindings

```rust
let report = bic::validate(&result.package, &inventory);

println!("matched: {}", report.matched().len());
println!("missing: {}", report.missing().len());
```

The validation report classifies each binding as Matched, Missing, Hidden, WeakMatch, or type-mismatched (NotAFunction, NotAVariable).

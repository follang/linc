# Getting Started

This chapter shows the shortest path from a normalized source contract to
machine-readable evidence.

Read `linc` as an analysis library. It produces evidence artifacts. It does
not promise that every successful analysis is safe for final build execution
or Rust generation.

In the toolchain split:

- `parc` owns source meaning
- `linc` owns link and binary meaning
- `gerc` owns lowering and emitted build metadata

The boundary rule is strict: `linc/src/**` must not depend on `parc` or
`gerc`, and cross-package translation belongs only in tests, examples, or
external harnesses.

## Add The Crate

Use a local path dependency while developing in the workspace:

```toml
[dependencies]
linc = { path = "../linc" }
```

If you need symbol inspection or validation, enable the `symbols` feature.

## Minimal Example

```rust
use linc::{
    analyze_source_package,
    SourceDeclaration,
    SourceFunction,
    SourcePackage,
    SourceType,
};

fn main() {
    let mut source = SourcePackage::default();
    source.declarations.push(SourceDeclaration::Function(SourceFunction {
        name: "mylib_init".into(),
        parameters: vec![],
        return_type: SourceType::Int,
        variadic: false,
        source_offset: None,
    }));

    let analysis = analyze_source_package(&source);
    println!(
        "declared link inputs: {}",
        analysis.declared_link_surface.ordered_inputs.len()
    );
    println!(
        "has resolved plan: {}",
        analysis.resolved_link_plan.is_some()
    );
}
```

The preferred output contract is `LinkAnalysisPackage`.

## JSON Round Trip

`LinkAnalysisPackage` is the contract intended to be exchanged across tools.

```rust
use linc::{analyze_source_package, LinkAnalysisPackage, SourcePackage};

let analysis = analyze_source_package(&SourcePackage::default());
let json = serde_json::to_string_pretty(&analysis).unwrap();
let restored: LinkAnalysisPackage = serde_json::from_str(&json).unwrap();
assert_eq!(analysis, restored);
```

## Common Integration Pattern

The common pattern is:

1. produce a `SourcePackage` in `parc` or another frontend
2. call `analyze_source_package`
3. optionally inspect artifacts with `inspect_symbols`
4. optionally validate against those artifacts
5. pass `SourcePackage` plus `LinkAnalysisPackage` to downstream tooling

If `parc` emits a serialized source artifact, a test, example, or external
harness should decode and translate it before calling `linc`.

## First Things To Inspect

When an analysis result does not look right, inspect these fields first:

- `analysis.declared_link_surface`
- `analysis.resolved_link_plan`
- `analysis.diagnostics`
- `analysis.abi_probe`
- `analysis.validation`
- `analysis.symbol_inventories`

Those surfaces usually tell you whether the problem is source intake, ABI
probing, link metadata declaration, provider discovery, or validation.

## Library-Only Design

`linc` is intended to be consumed as a Rust library that owns only link and
binary evidence concerns.

That means:

1. call `analyze_source_package()` or other public APIs directly
2. serialize the resulting values if another tool needs artifacts
3. keep cross-package translation in tests/examples/harnesses
4. keep final generation and build policy in downstream tools rather than in
   `linc`

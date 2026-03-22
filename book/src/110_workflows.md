# End-To-End Workflows

This chapter ties the separate surfaces together into practical workflows.

## Workflow 1: Analyze A Source Contract And Save JSON

```rust
use linc::{analyze_source_package, SourcePackage};

let analysis = analyze_source_package(&SourcePackage::default());
let json = serde_json::to_string_pretty(&analysis).unwrap();
std::fs::write("link-analysis.json", json).unwrap();
```

## Workflow 2: Translate PARC Artifacts In Tests Or Harnesses

The intended cross-package architecture is artifact-based, not shared-type
based. Library code should not import `parc`; translation belongs in tests,
examples, or external harnesses.

## Workflow 3: Inspect A Native Artifact

```rust
use linc::inspect_symbols;

let inventory = inspect_symbols("build/libdemo.so")?;
let json = serde_json::to_string_pretty(&inventory).unwrap();
std::fs::write("symbols.json", json).unwrap();
```

## Workflow 4: Validate Source-Derived Bindings Against Artifacts

Validation compares a binding package against one or more inventories.

## Workflow 5: Extract Just The Link Surface

Use `analysis.declared_link_surface` and `analysis.resolved_link_plan` when a
downstream tool only wants link names, artifact inputs, framework inputs,
platform constraints, or ordering metadata.

## Workflow 6: Downstream `fol` / `gerc` Consumption

1. `parc` produces a source artifact
2. tests/examples/harnesses translate that artifact into `linc` input
3. `linc` produces `LinkAnalysisPackage`
4. downstream generation reads source and link analysis in parallel

## Workflow 7: Repo-Local Bootstrap

The raw-header bootstrap path exists for difficult fixtures and repository
self-hosting. It is not the package boundary that downstream tools should
depend on.

## Workflow 8: ABI-Sensitive Packages

For packages with important struct ABI, attach layout evidence, inspect
symbols, and validate before generation.

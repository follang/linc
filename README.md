# LINC

LINC is the link and binary evidence layer in the `parc -> linc -> gerc`
toolchain.

It owns evidence: declared native inputs, discovered artifacts, resolved link
plans, ABI probe results, and validation findings.

## What LINC Actually Exposes Today

There are two real consumer layers in the crate:

1. a preferred contract-first layer centered on `SourcePackage` and
   `LinkAnalysisPackage`
2. a still-public lower-level layer centered on `BindingPackage`,
   `linc::ir::*`, and the repo-local `raw_headers` bootstrap path

The docs should not pretend the second layer is gone. It is still public and
still exercised by tests.

## Responsibilities

- consume source-shaped input through `SourcePackage`
- analyze declared link requirements
- inspect native artifacts for symbol evidence
- resolve provider choices into `ResolvedLinkPlan`
- probe ABI-sensitive layouts
- validate declarations against observed native artifacts
- serialize evidence products

## Non-Responsibilities

- owning source parsing/preprocessing as the main public boundary
- Rust code generation
- downstream crate-specific build policy
- library-level dependency on `parc` or `gerc`

## Preferred Surface

The preferred modern entrypoints are:

- `analyze_source_package`
- `LinkAnalysisPackage`
- `inspect_symbols`
- `probe_type_layouts`
- `validate` / `validate_many`

## Still-Public Lower-Level Surface

The crate root also still exposes:

- `BindingPackage` and related IR under `linc::ir`
- `raw_headers::HeaderConfig` and raw-header bootstrap helpers
- a large set of symbol/probe/validation/support types

That low-level surface is real. It is not the first story new consumers should
build around, but it is part of what the crate currently is.

## Minimal Contract-First Example

```rust
use linc::{analyze_source_package, SourcePackage};

let source = SourcePackage::default();
let analysis = analyze_source_package(&source);
println!("{}", analysis.declared_link_surface.ordered_inputs.len());
```

## Artifact Boundary

Cross-package composition belongs in tests, examples, and external harnesses.
`linc/src/**` stays self-contained even though `linc` may read and write
serialized artifacts that other tools also understand.

## Tested Scope

The current suite covers:

- source-contract analysis
- symbol inspection on ELF, Mach-O, and COFF-like inputs
- ABI probe reports
- validation reports
- raw-header bootstrap flows
- artifact-boundary tests using upstream fixtures
- large hostile/library surfaces such as zlib, libpng, libcurl, OpenSSL, and epoll

## Build And Test

```sh
make build
make test
```

## License

Dual-licensed under Apache 2.0 or MIT.

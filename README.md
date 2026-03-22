# LINC

LINC is the link and binary evidence layer in the `parc -> linc -> gerc`
toolchain.

It does not own parsing, preprocessing, or Rust lowering. It owns evidence:
what native inputs were declared, what native artifacts were found, how they
match, and what ABI evidence was measured.

## Responsibilities

- consume normalized source contracts through `SourcePackage`
- inspect native artifacts for symbol evidence
- normalize link requirements and provider choices
- probe ABI-relevant layouts
- validate source claims against compiled reality
- serialize evidence for downstream tools

## Non-Responsibilities

- source parsing and preprocessing as a public architecture boundary
- source extraction and declaration normalization upstream of `SourcePackage`
- Rust code generation
- downstream crate-specific build logic
- any library-level dependency on `parc` or `gerc`

Cross-package composition belongs only in tests, examples, or external
harnesses. Library code in `linc/src/**` stays self-contained.

## What LINC Produces

The main output families are:

- `LinkAnalysisPackage`
  a normalized evidence bundle derived from source intent plus optional native
  inspection
- `SymbolInventory`
  exported/imported symbol evidence from ELF, Mach-O, COFF, and similar
  artifacts
- `ValidationReport`
  declaration-vs-artifact evidence, including missing and mismatched cases
- `ResolvedLinkPlan`
  normalized library/framework/artifact requirements with provider matching
- `AbiProbeReport`
  compiler-measured layout evidence for requested types

These are evidence products, not parser products and not code-generation
products.

## Core Workflow

```rust
use linc::{analyze_source_package, SourcePackage};

let source = SourcePackage::default();
let evidence = analyze_source_package(&source);
let json = serde_json::to_string_pretty(&evidence).unwrap();
```

The normal sequence is:

1. an upstream frontend produces a normalized source artifact
2. a test, example, or harness translates that artifact into `SourcePackage`
3. LINC analyzes the package
4. downstream tooling consumes the evidence on its own terms

## Artifact Boundary

The real integration boundary is serialized artifacts, not shared Rust types
across crates.

- `parc` may serialize a source artifact
- tests/examples/harnesses may translate that artifact into `linc` input
- `linc` may serialize `LinkAnalysisPackage`, `SymbolInventory`, or
  `ValidationReport`
- `gerc` or another consumer may load those artifacts through its own
  test/example code

Library code inside `linc` must not be the place where cross-package
translation lives.

## Tested Scope

The suite currently exercises:

- Linux and other ELF-oriented flows
- macOS / Mach-O inventory and validation evidence
- split-pipeline artifact tests using upstream fixtures
- difficult native surfaces including zlib, libpcap, libcurl, OpenSSL,
  SocketCAN, epoll, and libpng

The tests are the statement of supported behavior.

## Build And Test

```sh
make build
make test
```

## License

Dual-licensed under Apache 2.0 or MIT (see `LICENSE-APACHE` and
`LICENSE-MIT`).

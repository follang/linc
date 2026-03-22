# LINC Reference

LINC is the link and binary evidence layer in the `parc -> linc -> gerc`
toolchain.

It owns evidence, not parsing and not lowering.

## What LINC Is For

LINC turns normalized source intent into native evidence. It can:

- normalize declared link requirements
- inspect object, archive, and shared-library artifacts
- probe ABI-relevant layouts
- validate declarations against binary reality
- serialize the resulting evidence for downstream consumers

## What LINC Produces

The main outputs are:

- `LinkAnalysisPackage`
- `SymbolInventory`
- `ResolvedLinkPlan`
- `ValidationReport`
- `AbiProbeReport`

Those outputs are transportable artifacts. They are what downstream tooling
should rely on, not private parser or extraction state.

## Data Flow

```text
normalized source input
  -> linc analysis
  -> link/binary evidence artifacts
  -> downstream consumer
```

In practice the input is a `SourcePackage`, the analysis entrypoint is
`analyze_source_package`, and any symbol/probe/validation pass is optional
evidence layered on top.

## Ownership Boundary

LINC owns:

- the evidence model
- the link surface
- the validation story
- the ABI probe story

LINC does not own:

- parser internals
- source preprocessing internals
- Rust code generation
- library-level composition with `parc` or `gerc`

Composition across packages belongs in tests, examples, or external harnesses.

## Modules And APIs

The root APIs are:

- `analyze_source_package`
- `inspect_symbols`
- `probe_type_layouts`
- `resolve_link_plan`
- `validate`

The important modules are:

- `intake`
- `analysis`
- `link_plan`
- `probe`
- `symbols`
- `validate`
- `diagnostics`
- `error`

`raw_headers` exists as a transitional bootstrap module and is not the long
term public architecture.

## Reading Order

1. [Getting Started](./010_getting_started.md)
2. [Intake Layer](./015_intake.md)
3. [Header Processing](./020_headers.md)
4. [IR Model](./030_ir.md)
5. [Native Evidence](./095_native_evidence.md)
6. [API Contract](./100_api_contract.md)
7. [End-To-End Workflows](./110_workflows.md)

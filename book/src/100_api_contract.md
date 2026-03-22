# API Contract

This chapter defines the intended public library surface of LINC as it exists
today, not as we might wish it already looked.

## First Principle

LINC is a library crate. The intended downstream pattern is:

1. call the crate from Rust
2. obtain structured values such as `LinkAnalysisPackage`,
   `SymbolInventory`, and `ValidationReport`
3. serialize those values only when another tool or process boundary needs them

## Preferred Public Surface

The crate root is still the preferred consumer boundary, but there are two
real layers inside it:

1. preferred contract-first APIs
2. lower-level IR/bootstrap APIs that remain public

## Normative Rules For Consumers

If you are building on top of LINC:

1. prefer crate-root re-exports over deep module imports
2. use `analyze_source_package` as the normal contract-first entry point
3. treat `LinkAnalysisPackage`, `SymbolInventory`, and `ValidationReport` as
   the primary transport-level contracts
4. treat diagnostics and validation results as normal structured output, not as
   ad hoc log text
5. do not rely on exact `String` error text for durable control flow
6. do not treat extracted declarations alone as sufficient ABI proof for
   layout-sensitive generation

## Public Surface Tiers

- Tier 1: `analyze_source_package`, `inspect_symbols`, `probe_type_layouts`,
  `validate`, `validate_many`, and `LinkAnalysisPackage`
- Tier 2: `BindingPackage`, root-level IR re-exports, and modules such as
  `probe`, `symbols`, `validate`, and `raw_headers`
- Tier 3: support-oriented modules such as `diagnostics`, `error`, and
  `line_markers`

Tier 2 and Tier 3 are real and tested. They are just not the first story the
book wants new downstream users to build around.

## Explicit Non-Goals

The current contract does not yet guarantee typed operational errors across the
whole crate, full ABI completeness for all C constructs, or full cross-platform
parity across ELF, Mach-O, and Windows-native artifact formats.

It also does not guarantee that repo-local bootstrap flows are the preferred
architecture, even though they are public.

## Artifact Boundary Reminder

LINC owns evidence, not universal pipeline state. Cross-package translation
belongs only in tests/examples/harnesses.

If another chapter sounds broader than this one, treat this chapter as the
normative boundary.

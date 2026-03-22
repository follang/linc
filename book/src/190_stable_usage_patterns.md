# Stable Usage Patterns

This chapter describes the usage patterns that are most likely to remain
stable for downstream library consumers.

## Pattern 1: Prefer Root-Level Entry Points

Prefer `HeaderConfig`, `probe_type_layouts`, `inspect_symbols`, `validate`,
`validate_many`, and `analyze_source_package`.

## Pattern 2: Treat `BindingPackage` As The Primary Product

A typical flow is:

1. analyze or bootstrap source input
2. inspect `package.diagnostics`
3. optionally attach or compare native evidence
4. serialize the package only when crossing a tool boundary

## Pattern 3: Treat Diagnostics As Contractual Data

Read `package.diagnostics`, classify which diagnostic kinds are blocking for
your downstream generator, and make the decision explicit in your consumer.

## Pattern 4: Gate Artifact Consumption On `schema_version`

Gate on `schema_version`, treat `linc_version` as provenance, and keep fixture
coverage for the payload shapes you rely on.

## Pattern 5: Preserve Native Metadata Instead Of Re-Deriving It

When `package.link`, `package.layouts`, or symbol inventories are available,
prefer using them.

## Pattern 6: Keep Validation Separate From Transport Failure

Execution failure should be handled as `Err(...)`. Successful validation with
mismatches should be handled as structured evidence.

## Anti-Patterns

- matching on exact free-form error strings
- treating pretty JSON formatting as a semantic contract
- assuming every successful scan is generation-ready
- inferring link intent only from declarations when package-level link evidence
  already exists

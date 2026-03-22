# Error Taxonomy

This chapter defines the intended typed error taxonomy that later
implementation slices should converge on.

## Goal

`LincError` should be the crate-wide typed failure surface for operational
failures.

The design target is to separate configuration and scan execution failures,
preprocessing and parse failures, ABI probe failures, artifact inspection
failures, and serialization or schema failures.

Validation findings are deliberately different. They should remain structured
report output, not thrown as operational errors.

## Current Coverage

The current enum already contains variants for missing headers, preprocessor
failure, parse failure, I/O failure, serialization failure, symbol-read
failure, unsupported artifact format, and schema-version mismatch.

## Intended Category Boundaries

- configuration failure should be distinguishable from compiler failure
- consumers should be able to distinguish toolchain invocation failure from
  source parse failure
- probe failures should not be collapsed into generic scan or I/O text
- path and context should be preserved in typed artifact errors

## Validation Is Not An Error Channel

Validation mismatches should not be encoded as `LincError`.
Validation should keep returning `ValidationReport`.

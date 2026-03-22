# Failure Model

This chapter defines the intended boundary between hard failures, diagnostics,
and validation findings.

## The Three Outcome Classes

1. hard operational failure
2. successful analysis with diagnostics
3. successful validation with findings

## Consumer Rule

- `Err(...)` means the requested operation itself failed
- diagnostics mean the operation succeeded, but the returned analysis may be
  partial or lossy
- validation findings mean the operation succeeded and produced evidence that
  the native surface does not match expectations cleanly

That means a robust downstream integration should not collapse everything into
a single boolean "success" value.

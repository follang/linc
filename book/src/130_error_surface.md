# Error Surface

This chapter inventories the current public error surface of LINC.

## Current State

LINC exposes typed errors via `LincError` and structured diagnostics inside
returned data.

## Typed Error Surface Today

The clearest typed error boundary today is around the explicit workflow APIs
such as `probe_type_layouts(...)` and `inspect_symbols(...)`.

## What Consumers Should Do Right Now

Downstream users should:

- treat successful return values as stable enough to consume
- treat diagnostics in returned data structures as first-class signals
- avoid matching exact error strings for durable control flow
- wrap stringly errors at their own boundary if they need structured handling
  immediately

## What Counts As An Error vs A Diagnostic

- hard operational failures generally return an error
- partially understood source constructs often become diagnostics attached to a
  returned package
- validation findings are reported as structured match results, not thrown as
  errors

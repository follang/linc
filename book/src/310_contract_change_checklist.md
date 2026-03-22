# Contract Change Checklist

Use this checklist whenever a release includes changes to schema, public API,
or checked-in contract fixtures.

## Schema Changes

- confirm whether the change is additive, behavioral, or breaking
- keep `schema_version` unchanged for additive/defaulted changes
- bump `schema_version` only when older consumers can no longer deserialize or
  safely interpret the payload

## Public API Changes

- confirm whether the root-level API contract changed or only lower-level
  modules changed
- update crate-level docs and book chapters when recommended usage changes

## Fixture Changes

- confirm the fixture still represents a real supported or intentionally
  unsupported scenario
- confirm the corresponding regression test explains why the fixture exists

## Consumer Guidance Changes

- confirm generic library guidance stays separate from consumer-specific
  guidance
- confirm consumer guidance remains an example profile rather than universal
  crate policy

## Final Questions

Ask whether the change altered what downstream code can safely rely on, whether
fixture coverage changed to prove the new boundary, and whether the docs now
describe the same boundary.

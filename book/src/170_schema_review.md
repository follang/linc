# Schema Version Review

This chapter records the current review decision for `SCHEMA_VERSION`.

## Current Decision

The schema version remains:

```text
SCHEMA_VERSION = 1
```

That is intentional for the current phase.

## Why It Has Not Been Bumped Yet

The IR has grown substantially, but the recent changes have mostly been of this form:

- additive fields
- additive nested metadata
- serde-defaultable structures
- richer evidence attached to existing top-level containers

Those changes are important, but they do not automatically justify a schema bump unless they change the meaning of previously valid payloads in a non-defaultable way.

## Current Review Standard

A future bump to `2` should happen when one or more of these become true:

- an existing field changes meaning in a way old consumers would misread
- a representation changes in a non-defaultable way
- downstream consumers need a new explicit compatibility boundary
- the project decides the current "additive within v1" period is over and wants a stricter contract line

## Practical Consequence

Today, downstream consumers should read the current status as:

- `v1` is still the active compatibility line
- compatibility within `v1` is being hardened with fixtures and tests
- a future `v2` should be deliberate, documented, and paired with migration reasoning

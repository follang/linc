# Schema Version Review

This chapter records the current review decision for `SCHEMA_VERSION`.

## Current Decision

The schema version remains:

```text
SCHEMA_VERSION = 1
```

## Why It Has Not Been Bumped Yet

The recent changes have mostly been additive fields, additive nested metadata,
serde-defaultable structures, and richer evidence attached to existing
top-level containers.

## Current Review Standard

A future bump to `2` should happen when an existing field changes meaning in a
way old consumers would misread, a representation changes in a non-defaultable
way, or the project decides the current shape can no longer evolve safely
within `v1`.

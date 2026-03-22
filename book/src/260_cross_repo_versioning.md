# Cross-Repo Versioning

This chapter describes how producers and consumers should think about versioned
artifacts across repository boundaries.

## Artifact Keys

Use `schema_version` as the artifact gate and `linc_version` as provenance.

## Coordination Rules

Cross-repo consumers should pin the artifact shape they understand and reject
future shapes instead of guessing.

## Additive Changes

Additive changes should be documented and fixture-tested.

## Breaking Contract Changes

Breaking changes require explicit review and a schema bump when older
consumers would misread the payload.

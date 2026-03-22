# Field Stability

This chapter classifies the current `BindingPackage` artifact by stability
expectations.

## Top-Level `BindingPackage`

The top-level package fields fall into three practical groups:

- contract identity fields
- stable container fields
- evolving evidence fields

## Contract Identity Fields

| Field | Current classification | Notes |
|---|---|---|
| `schema_version` | required contract field | artifact-shape gate |
| `linc_version` | stable provenance field | producer version, not the main shape gate |
| `source_path` | useful provenance field | helpful, but not the primary artifact anchor |

## Stable Container Fields

The major package sections downstream tools can reasonably depend on existing
are `target`, `inputs`, `macros`, `layouts`, `link`, `items`, and
`diagnostics`.

## Practical Rule For Downstream Consumers

Rely on top-level package sections and documented meanings, treat nested
metadata as additive/defaultable unless explicitly documented otherwise, and
use `schema_version` as the hard artifact boundary.

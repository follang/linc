# JSON Artifacts

This chapter describes how `linc` treats serialized JSON artifacts.

The important framing is architectural:

- JSON is an artifact format
- it is not a promise to preserve old pipeline shapes forever
- the only shapes that matter are the ones currently documented and tested

`linc` does not carry a backward-compatibility burden for discarded designs.

## First Principle

Consumers may depend on documented field names, documented field meanings,
`schema_version`, and documented defaulting behavior when tests rely on it.

Consumers must not depend on whitespace, pretty-print layout, incidental field
ordering, or undocumented fields that happen to be present today.

## Main Serialized Artifacts

The main JSON-bearing values are:

- `LinkAnalysisPackage`
- `SymbolInventory`
- `ValidationReport`
- `ResolvedLinkPlan`

## Version Fields

`schema_version` is the artifact gate. `linc_version` identifies the producing
build and is useful for provenance.

## Change Policy

- do not preserve obsolete artifact envelopes just because they existed earlier
- do not keep old field layouts alive unless current tests still need them
- do make semantic changes explicit in docs and fixtures
- do update `schema_version` when consumers would otherwise misread the
  artifact

## Maintenance Rule

When artifact shapes change:

1. update the docs
2. update or replace the relevant fixtures
3. update consumers in tests/examples/harnesses
4. do not carry a dead compatibility layer just to keep an obsolete shape
   deserializable

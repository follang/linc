# Release Checklist

Use this checklist before cutting a release candidate.

## Build And Test

- run `make build`
- run `make test`

## Contract Surfaces

- confirm the documented JSON artifact shapes remain consumable by the current
  schema version
- confirm `ValidationReport` fixture coverage still matches current structured
  fields

## Documentation

- confirm README wording matches tested behavior
- confirm the book reflects current API entry points and platform scope

## Consumer Boundary

- confirm the generic library contract stays primary
- confirm cross-package composition is still described as tests/examples/
  harness work, not crate-to-crate library coupling

## Release Decision

Do not treat "builds successfully" as sufficient. The code, docs, and fixtures
all need to match the same boundary.

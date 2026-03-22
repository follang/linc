# `fol` Integration Guide

This chapter describes the intended producer/consumer contract between LINC
and `fol`.

## Division Of Responsibility

- LINC extracts declarations, metadata, diagnostics, layouts, and native
  evidence
- `fol` consumes that evidence to generate bindings and apply policy

## What `fol` Should Expect

- `BindingPackage` as the primary declaration and metadata contract
- `BindingPackage.diagnostics` as explicit extraction warnings and
  partial-fidelity signals
- `BindingPackage.layouts` when ABI-sensitive types need compiler-probed
  evidence
- `BindingPackage.link` as the normalized native dependency surface
- `SymbolInventory` and `ValidationReport` when native artifact matching matters

## Recommended End-To-End Flow

1. run LINC header scanning or source analysis
2. inspect `BindingPackage.diagnostics`
3. require layout probes for ABI-sensitive types
4. inspect native artifacts and run validation when linkable symbols matter
5. pass the resulting structured values to `fol`
6. let `fol` decide what to generate, reject, or gate behind policy

## Contract Boundaries

- `schema_version` is the wire-compatibility gate
- `linc_version` is producer provenance
- `BindingPackage` is the declaration and metadata contract
- `ValidationReport` is evidence, not an exception channel
- diagnostics are part of the data contract, not incidental logs

## Minimal Durable Contract

The shortest durable `fol` contract is:

1. a serialized `BindingPackage`
2. optional `SymbolInventory` values
3. optional `ValidationReport`
4. explicit consumer policy over diagnostics, layout evidence, and link
   evidence

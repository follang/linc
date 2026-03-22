# `fol` Minimal Contract

This chapter defines the smallest durable contract that `fol` can rely on.

## Minimal Required Inputs

- a serialized `BindingPackage`
- diagnostics

## Minimal Required Semantics

The minimal contract lets `fol` inspect declarations and policy-gate on
diagnostics.

## What The Minimal Contract Does Not Promise

It does not promise layout evidence, link resolution, or validation.

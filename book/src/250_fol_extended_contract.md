# `fol` Extended Contract

This chapter defines the optional evidence that makes `fol` more confident.

## Extended Optional Inputs

- `layouts`
- `link`
- `SymbolInventory`
- `ValidationReport`
- macros

## Why This Contract Is Optional

Some generation tasks only need declarations. ABI-sensitive or publication
quality workflows usually need more evidence.

## Consumer Rule

Use the extended contract when the downstream decision really depends on
layout, link, or validation evidence.

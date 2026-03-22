# Origin Filtering

By default, LINC does not blindly keep every declaration found after
preprocessing. It uses source-origin information to keep the extracted surface
focused on the headers you asked for.

This behavior is one of the reasons scans stay usable on real systems with
deep transitive header trees.

## The Problem Filtering Solves

A normal header often pulls in:

- C runtime headers
- platform SDK headers
- project-local support headers
- unrelated transitive declarations

If all of that were kept by default, a scan of one library header could
explode into a large, noisy package dominated by system declarations.

## How Origin Tracking Works

The C preprocessor emits line markers such as:

```text
# 42 "/usr/include/stdio.h" 3
```

LINC parses those markers into a `FileOriginMap`. That map is then used to
classify declaration offsets.

Current origin classes are:

| Origin | Meaning |
|---|---|
| `Entry` | From an entry header explicitly requested by the user |
| `UserInclude` | From a non-system header included by an entry header |
| `System` | From a system header |
| `Unknown` | The origin could not be determined reliably |

## Default Behavior

The default `OriginFilter` keeps entry-header declarations and user-include
declarations, and excludes system-header declarations.

This is usually the right tradeoff for evidence generation because it
preserves the API surface while avoiding C runtime clutter.

## Disable Filtering Entirely

Disable filtering when you want the complete preprocessed declaration world.
That is useful for debugging missing declarations or validating whether a
declaration really exists after preprocessing.

## Custom Filters

Custom filters are useful when system declarations are intentionally part of
the bindable contract.

## Practical Advice

If a declaration seems to be missing:

1. rerun with `.no_origin_filter()`
2. inspect the preprocessing report
3. confirm the declaration was not removed by macro conditions
4. confirm the declaration still maps cleanly to a known origin

Most missing-item surprises come from one of those four causes.

## Why Filtering Happens After Extraction

LINC first extracts from the parsed translation unit and then filters by
origin.

That design has two benefits:

- extraction logic sees the same full parse tree the compiler saw
- filtering policy stays configurable and separate from parsing

It also means you can inspect the same source through multiple origin policies
without changing preprocessing itself.

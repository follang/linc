# Origin Filtering

By default, BIC filters out declarations from system headers (like `stdio.h`), keeping only declarations from your entry headers and their non-system includes.

## How it works

The preprocessor emits line markers (`# linenum "file" flags`). BIC parses these to build a `FileOriginMap` that classifies each byte range as:

| Origin | Description |
|--------|-------------|
| `Entry` | From one of the user's entry headers |
| `UserInclude` | Included from entry headers, not a system header |
| `System` | System header (flag 3 in line markers) |
| `Unknown` | Origin could not be determined |

## Customizing the filter

```rust
use bic::OriginFilter;

// Include everything
let result = HeaderConfig::new()
    .header("mylib.h")
    .no_origin_filter()
    .process()
    .unwrap();

// Custom filter: include system headers too
let result = HeaderConfig::new()
    .header("mylib.h")
    .origin_filter(OriginFilter {
        include_entry: true,
        include_user: true,
        include_system: true,
    })
    .process()
    .unwrap();
```

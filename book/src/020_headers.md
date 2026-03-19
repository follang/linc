# Header Processing

`HeaderConfig` drives the full pipeline from C headers to extracted bindings.

## Configuration

| Method | Description |
|--------|-------------|
| `header(path)` | Add an entry header to parse |
| `include_dir(path)` | Add an include search path (`-I`) |
| `define(name, value)` | Add a preprocessor define (`-D`) |
| `compiler(cmd)` | Override the C compiler (default: `gcc`) |
| `flavor(f)` | Set the C dialect: `GnuC11`, `ClangC11`, `StdC11` |
| `origin_filter(f)` | Set custom origin filter |
| `no_origin_filter()` | Disable origin filtering (include everything) |

## Process

Calling `.process()` runs the preprocessor, parses the output with PAC, extracts the IR, and applies origin filtering.

```rust
let result = HeaderConfig::new()
    .header("zlib.h")
    .include_dir("/usr/include")
    .process()
    .unwrap();
```

The result contains:
- `package`: the extracted `BindingPackage`
- `report`: preprocessing metadata (command, args, preprocessed source)

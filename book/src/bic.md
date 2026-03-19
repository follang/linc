# BIC Reference

BIC (bind c) extracts C binding and ABI information from C headers using the PAC parser.

It produces a structured intermediate representation (IR) of functions, types, enums, records, and variables found in C headers, and can generate Rust FFI bindings or JSON output.

## Pipeline

```
C headers → preprocessor → PAC parser → BIC extractor → IR → codegen / JSON
```

## Features

- Parse C11 headers (GNU and Clang extensions supported)
- Origin filtering to exclude system header declarations
- Rust FFI code generation
- JSON serialization of the binding IR
- ELF symbol inspection and validation

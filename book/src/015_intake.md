# Intake Layer

The intake layer is LINC's frontend-neutral source contract.

It defines what LINC needs from an upstream frontend without coupling to any
specific parser AST or source extraction implementation.

## SourcePackage

The primary intake type is `SourcePackage`.

An upstream frontend such as `parc` produces this after scanning and
extracting source-level information.

```rust
use linc::{
    analyze_source_package,
    SourceDeclaration,
    SourceFunction,
    SourcePackage,
    SourceType,
};

let mut source = SourcePackage::default();
source.source_path = Some("mylib.h".into());
source.declarations.push(SourceDeclaration::Function(SourceFunction {
    name: "init".into(),
    parameters: vec![],
    return_type: SourceType::Int,
    variadic: false,
    source_offset: None,
}));

let analysis = analyze_source_package(&source);
assert!(analysis.resolved_link_plan.is_some() || analysis.diagnostics.len() >= 0);
```

## Declaration Types

The intake layer supports these declaration kinds:

- `SourceFunction` for function declarations
- `SourceRecord` for struct/union declarations
- `SourceEnum` for enum declarations with variants
- `SourceTypeAlias` for typedef and alias declarations
- `SourceVariable` for external variable declarations

Records may be opaque when `fields` is `None`.

## Type Model

`SourceType` is a simplified, language-neutral type representation. It is not
a full lossless C type system.

It covers:

- primitive types such as `Void`, `Bool`, `Char`, `Int`, `UInt`, and `Long`
- pointers and const pointers
- arrays
- function pointers
- references to typedefs, records, and enums
- `Const` and `Volatile` wrappers

## Intake Contract

The intended intake path is:

1. produce `SourcePackage`
2. call `analyze_source_package`
3. consume `LinkAnalysisPackage`

Any adapter code that converts a serialized source artifact into `SourcePackage`
belongs in tests, examples, or an external harness.

## Design Principles

1. LINC core logic should say "analyze this normalized source surface", not
   "parse this"
2. adapter code is separate from core analysis logic
3. `parc` may be used in tests, but not as a library-level dependency of LINC
4. another frontend should be able to replace `parc` without rewriting LINC

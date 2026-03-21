# Intake Layer

The intake layer is LINC's frontend-neutral input contract. It defines what
LINC needs from a frontend without coupling to any specific parser AST.

## SourcePackage

The primary intake type is `SourcePackage`. A frontend such as `parc`
produces this after scanning and extracting source-level information.

```rust
use linc::{analyze_source_package, SourceDeclaration, SourceFunction, SourcePackage, SourceType};

let mut src = SourcePackage::default();
src.source_path = Some("mylib.h".into());
src.declarations.push(SourceDeclaration::Function(SourceFunction {
    name: "init".into(),
    parameters: vec![],
    return_type: SourceType::Int,
    variadic: false,
    source_offset: None,
}));

let analysis = analyze_source_package(&src);
assert!(analysis.resolved_link_plan.is_some());
```

## Declaration Types

The intake layer supports these declaration kinds:

- `SourceFunction` — function declarations
- `SourceRecord` — struct/union declarations (opaque when `fields` is `None`)
- `SourceEnum` — enum declarations with variants
- `SourceTypeAlias` — typedef/alias declarations
- `SourceVariable` — external variable declarations

## Type Model

`SourceType` is a simplified, language-neutral type representation:

- Primitive types: `Void`, `Bool`, `Char`, `Int`, `UInt`, `Long`, etc.
- Pointers: `Pointer(inner)`, `ConstPointer(inner)`
- Arrays: `Array(element, size)`
- Function pointers: `FunctionPointer { return_type, parameters, variadic }`
- References: `TypedefRef(name)`, `RecordRef(name)`, `EnumRef(name)`
- Qualifiers: `Const(inner)`, `Volatile(inner)`

## Low-Level Intake Support

The `intake::adapters` module still exists as low-level support while `linc`
finishes shedding historical IR assumptions.

New downstream code should not treat those adapters as the normal integration
surface. The intended public story is:

- produce `SourcePackage`
- call `analyze_source_package`
- consume `LinkAnalysisPackage`

## Design Principles

1. LINC core logic should say "analyze this normalized source surface", not "parse this"
2. Adapter code is separate from core analysis logic
3. `parc` can be a dev dependency in tests, but not in core architecture
4. Another frontend should be able to replace `parc` without rewriting LINC

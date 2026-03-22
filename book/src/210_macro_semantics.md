# Macro Semantics

`MacroBinding` is the normalized macro representation in the package.

## Intended Semantics By Category

- `BindableConstant`: safe candidates for generated constants
- `ConfigurationFlag`: environment and availability signals
- `AbiAffecting`: macros that may influence layout or calling behavior
- `Unsupported`: capture and report, but do not assume a safe lowering path

## Function-Like vs Object-Like

`MacroForm` preserves whether the macro was object-like or function-like.
That distinction matters because function-like macros are often not safe to
lower automatically.

## Consumer Guidance

Consumers should treat macro evidence as policy input, not as a promise that
every captured macro should become a generated constant.

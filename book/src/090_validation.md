# Validation

Validation compares a `BindingPackage` against one or more `SymbolInventory`
values.

It answers a practical question: do the declarations we extracted line up with
what the native artifacts actually provide?

## API Entry Points

Use `validate` for one artifact and `validate_many` for several.

## What Validation Looks At

Validation focuses on symbol presence, symbol kind, visibility, binding
strength, decorated names, and conservative ABI-shape evidence where the
artifact can prove something honestly.

## Common Statuses

Current statuses include:

- `Matched`
- `AbiShapeMismatch`
- `Missing`
- `UnresolvedDeclaredLinkInputs`
- `DecorationMismatch`
- `NotAFunction`
- `NotAVariable`
- `Hidden`
- `WeakMatch`
- `DuplicateProviders`

## How To Read A Report

- `Matched` means the declaration resolved to a visible symbol of the expected
  kind
- `Missing` means no matching symbol was found and the package did not declare
  native link inputs that might reasonably have provided it
- `UnresolvedDeclaredLinkInputs` means the package did declare native inputs,
  but validation still found no provider
- `DecorationMismatch` means a decorated or raw spelling normalized to the
  declaration name
- `Hidden` and `WeakMatch` should usually be treated more conservatively than a
  strong export
- `DuplicateProviders` usually blocks promotion until the consumer chooses a
  policy

## Provider Evidence

Provider evidence may include plain artifact paths or archive-member provenance
such as `libfoo.a:bar.o`.

## Consumer Rule

Validation findings are structured evidence, not hard execution errors. Treat
them as policy input for the next stage.

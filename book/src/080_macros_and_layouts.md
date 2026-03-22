# Macros And Layouts

Two of the most important "not just declarations" surfaces in LINC are macro
inventory and compiler-probed type layouts.

Together they close a large part of the gap between syntax-only header
extraction and ABI-aware analysis.

## Macro Inventory

`BindingPackage.macros` captures macro definitions seen during raw-header or
bootstrap scans.

Each `MacroBinding` carries:

- `name`
- `body`
- `function_like`
- `form`
- `kind`
- `category`
- optional parsed `value` for bindable integer/string constants

`BindingPackage.macro_provenance` carries package-level provenance entries for
captured macros, including origin classification and source location where
line-marker evidence is available.

### Macro Kind

Current kinds are:

- `Integer`
- `String`
- `Expression`
- `Other`

This is a structural classification of the macro body.

### Macro Category

Current categories are:

- `BindableConstant`
- `ConfigurationFlag`
- `AbiAffecting`
- `Unsupported`

This is a higher-level classification intended to help downstream consumers
decide which macros are relevant.

## Why Macro Capture Matters

Many real C APIs encode essential information in macros:

- integer constants
- version identifiers
- feature toggles
- calling-convention selectors
- export/import annotations
- ABI-affecting packing or configuration knobs

Without macros, a binding package is often incomplete even if declaration
extraction succeeded.

## Practical Macro Interpretation

Downstream tools should usually treat categories differently:

- `BindableConstant`: good candidates for generated constants
- `ConfigurationFlag`: environment and availability signals
- `AbiAffecting`: do not ignore; these may change layout or calling behavior
- `Unsupported`: evidence worth reporting, not blindly generating

## Layout Probing

`TypeLayout` currently stores:

- `name`
- `size`
- `align`

The layouts are produced by compiler-assisted probing. That means they reflect
the configured compiler environment rather than guessed sizes.

`AbiProbeReport` also preserves target/compiler identity metadata alongside the
layouts. That makes probe evidence auditable and safer to hand across process
or repo boundaries.

## Probe Subjects

The report also carries `subjects`. Each `ProbeSubjectReport` keeps:

- the requested subject name
- its broad subject kind (`Type`, `Record`, or `Enum`)
- probe confidence
- record completeness when the subject is a record
- the measured `TypeLayout`

For record subjects, `fields` may also preserve named field offsets as
compiler-measured evidence.

For bitfields, the current probe surface is intentionally partial:

- `bit_width` may be present
- `offset_bytes` may remain absent

That is deliberate. LINC preserves width evidence where it can, but does not
guess a byte offset for bitfields when the probe path cannot establish one
safely.

## Probe Degradation Semantics

Probe requests do not all fail for the same reason.

- `ProbeUnavailable` means the requested subject did not have a safely
  probeable layout in the current compilation model
- `ProbeFailed` means the probe mechanism itself failed operationally or
  compiled invalid probe input

That split lets a downstream generator apply a policy such as:

- tolerate `ProbeUnavailable` for explicitly opaque inputs
- require layouts for by-value ABI-sensitive records and typedef-backed value
  types
- treat any `ProbeFailed` result as suspicious until the probe path is fixed or
  explicitly waived

Enum subjects also preserve:

- `enum_underlying_size`
- `enum_is_signed`

This gives downstream generators a concrete representation hint even before
field-level enum analysis exists in the declaration IR.

## What Layouts Solve

Compiler-probed layouts are especially useful for:

- checking that opaque vs non-opaque modeling matches reality
- proving `sizeof` and `alignof` for important structs
- gating generation on ABI-sensitive records
- preserving ABI evidence in a transportable JSON package

## What Layouts Do Not Yet Solve

Current layout data is intentionally small. It does not yet provide a full
field-offset or bitfield-layout model.

So treat `TypeLayout` as stronger than guessing, but not yet a complete ABI
proof for all record shapes.

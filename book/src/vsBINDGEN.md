# LINC vs bindgen

This document compares two different approaches to C interop from the point of
view of the current toolchain split.

## The Short Version

- bindgen is a header-to-Rust transpiler
- LINC is an evidence engine

bindgen answers "what does this header say?"
LINC answers "what does the source say, what does the artifact say, and do
they agree?"

## Parsing

bindgen depends on libclang and the Clang frontend. LINC keeps parsing and
source extraction upstream of its own evidence layer and does not depend on
libclang.

## Internal Representation

bindgen's IR is transient and internal to one code generation run. LINC's
`BindingPackage` is a durable, serialized evidence contract.

## ABI Discovery

bindgen reads ABI information from libclang. LINC can attach compiler-probed
layout evidence and keep that evidence alongside the rest of the analysis
package.

## Symbol Inspection And Validation

bindgen does not own native artifact inspection or validation.
LINC does.

## Code Generation

bindgen's job ends in generated Rust. LINC's job ends in evidence. A downstream
tool such as `gerc` can consume LINC's evidence and emit Rust or build
metadata.

## When To Use Which

Use bindgen when you want a direct header-to-Rust generator and are willing to
pay the libclang cost. Use LINC when you want analysis, evidence, link
metadata, validation, and downstream policy separation.

That is the real split:

- bindgen centers immediate Rust emission
- LINC centers analysis and evidence that another tool can consume later

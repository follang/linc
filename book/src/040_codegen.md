# Code Generation

LINC does not own Rust code generation.

This chapter exists only to make the boundary explicit: LINC ends at evidence.
Downstream tooling such as `gerc` owns lowering and emitted build metadata.

## Migration Note

Older bootstrap paths in the repository may still carry codegen-shaped names,
but they are not the architectural center. The evidence contract is the
stable boundary.

## Practical Rule

If you need Rust source, build metadata, or linker directives, consume the
evidence packages from LINC and hand them to the downstream generator that
owns that job.

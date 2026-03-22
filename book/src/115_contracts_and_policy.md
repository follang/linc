# Contracts And Policy

This section groups the durable contract and policy chapters.

The most important policy rule is architectural:

- `linc/src/**` must not depend on `parc` or `gerc`
- cross-package translation belongs in tests/examples/harnesses
- `linc` owns its own internal model and its own evidence artifacts
- there is no shared ABI crate
- there is no backward-compatibility burden for old pipeline shapes
- bootstrap utilities and repo-local shortcuts must never be mistaken for the
  public contract

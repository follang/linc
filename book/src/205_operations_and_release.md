# Operations And Release

This section covers the operational and release posture of LINC.

## Operations

LINC is a library-first analysis tool. It is meant to be embedded, tested,
and serialized, not launched as a separate end-user service.

## Release

A release should be judged on build and test health, JSON contract stability,
documentation alignment, fixture coverage, and platform support posture.

The architectural rule remains the same here too:

- LINC owns evidence and analysis
- downstream build and generation policy still belongs outside LINC
- tests/examples/harnesses are where cross-package composition is proven

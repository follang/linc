# Reproducibility

This chapter describes what must be reproducible for LINC to be trustworthy.

## Reproducibility Requirements

- checked-in JSON contract fixtures must be deterministic
- library-only unit tests should be deterministic without requiring internet
  access
- toolchain-dependent tests should be explicit about their assumptions

## Fixture Rules

Prefer checked-in headers, JSON payloads, and small native test artifacts where
practical.

## Contract Tests

The main contract tests should prove that source intake, validation, and link
planning stay explainable and stable.

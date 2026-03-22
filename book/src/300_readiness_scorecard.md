# Readiness Scorecard

This chapter summarizes current release readiness by subsystem.

## Overall Readiness

LINC should be read as strong for library-first extraction and evidence
production, with ELF as the strongest native-artifact path and Mach-O as a
conservative but useful secondary path.

## Subsystem Scorecard

- Header extraction: high
- JSON contract: high
- ABI layout evidence: medium-high
- Symbol inventories: high for ELF, medium-high for Mach-O
- Validation: medium-high
- Link planning: medium-high
- Consumer integration: high for the documented narrow consumer profile

## How To Read This Scorecard

High means the subsystem is a reliable contract surface for normal downstream
use. Medium-high means consumers should still respect the documented limits.

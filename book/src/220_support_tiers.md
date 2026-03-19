# Support Tiers

This chapter turns the platform matrix into explicit support tiers.

The reason for doing this separately is that a matrix answers "what exists", while a support tier
answers "how hard should downstream code rely on it".

## Tier Definitions

### Tier 1: Preferred

Meaning:

- directly exercised in this repository
- suitable for normal internal production use
- the first path that regressions should be fixed against

### Tier 2: Supported But Maturing

Meaning:

- functionality exists and is useful
- downstream use is reasonable with caution
- edge cases and parity gaps are still expected

### Tier 3: Experimental / Incomplete

Meaning:

- not ready for strong production claims
- contract surface may still change materially
- downstream users should isolate or avoid depending on it

## Current Tier Assignment

| Area | Current tier |
|---|---|
| Linux / ELF header scan + symbol inventory + validation path | Tier 1 |
| Apple / Mach-O scan and symbol path | Tier 2 |
| Windows / COFF/PE native artifact path | Tier 3 |
| JSON contract for `BindingPackage` | Tier 1 |
| `BindingPackage` + diagnostics as library handoff | Tier 1 |
| macro inventory as semantic binding input | Tier 2 |
| ABI probe expansion beyond size/align | Tier 2 |

## Downstream Guidance

Downstream tools should:

- optimize their most critical paths around Tier 1 surfaces
- isolate Tier 2 usage behind smaller adapters or feature gates
- avoid depending on Tier 3 behavior as a core release blocker path

## Maintenance Implication

When a regression appears:

- Tier 1 regressions should block release
- Tier 2 regressions should be tracked explicitly and documented
- Tier 3 regressions should not be surprising, but should still be recorded honestly

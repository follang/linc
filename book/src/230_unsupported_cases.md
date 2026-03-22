# Unsupported Cases

This chapter keeps unsupported or incomplete areas explicit.

## Native Artifact Formats

Windows-native artifact support is still incomplete compared with ELF and
Mach-O.

## ABI Modeling

LINC does not yet model every ABI detail for every record shape. Layout
evidence is conservative and partial where needed.

## Macro Semantics

Not every macro should be lowered automatically. Unsupported macros remain
visible as evidence.

## Validation Depth

Validation is evidence, not a full platform linker oracle.

## Why This Chapter Exists

Unsupported cases should stay visible so downstream consumers can make policy
choices explicitly.

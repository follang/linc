# Header Processing

`HeaderConfig` is a repo-local bootstrap utility for turning raw header sets
into a `BindingPackage`.

It exists because the repository still needs a way to start from real headers
in difficult test and bootstrap scenarios. It is not the architectural center
of LINC, but it is still real public API and still covered by tests.

The intended architecture is:

- an upstream frontend such as `parc` owns preprocessing, parsing, and
  declaration extraction
- LINC consumes normalized source input and produces evidence
- cross-package translation belongs outside `linc/src/**`

## What `HeaderConfig` Is Good For

Use `HeaderConfig` when you need to:

- bootstrap the repository from real system or vendored headers
- drive difficult header fixtures without teaching another frontend every edge
  case first
- gather preprocessing output, extracted declarations, native link metadata,
  and probe evidence in one local pass

It is not the preferred downstream boundary.

## Conceptual Domains

Even though `HeaderConfig` is one builder, it carries several distinct
domains:

1. preprocessing environment
2. entry-header selection
3. declared native-link intent
4. ABI probe requests
5. origin-filtering policy

## Configuration Surface

The most important builder methods are:

| Method | Purpose |
|---|---|
| `header(path)` | Add an entry header |
| `include_dir(path)` | Add an include search path |
| `framework_dir(path)` | Add a framework search path |
| `library_dir(path)` | Add a native library search path |
| `define(name, value)` | Add a preprocessor define |
| `compiler(cmd)` | Override the driver used for preprocessing or probing |
| `flavor(f)` | Select dialect handling |
| `origin_filter(f)` | Keep only declarations from selected origins |
| `no_origin_filter()` | Keep declarations from every origin |
| `probe_type_layout(name)` | Request compiler-probed layout data |

Repeated path, define, link, constraint, and probe calls append in order.
The builder does not deduplicate for you.

## Validation Before Execution

The bootstrap path validates its inputs before it executes. Treat invalid
configuration as an operational error, not as a diagnostic hidden inside a
usable result.

## What The Bootstrap Path Does

The bootstrap helpers are for local repository work and test fixtures:

1. synthesize a temporary translation unit from the configured entry headers
2. preprocess it with the configured compiler and dialect settings
3. capture macros from the same environment
4. extract declarations and attached metadata
5. attach target, input, and declared link provenance
6. optionally probe requested layouts
7. optionally filter by origin

The resulting package is a bootstrap artifact built around `BindingPackage`,
not the preferred downstream boundary.

## Policy

If you are writing new downstream code:

- do not treat `HeaderConfig` as the pipeline contract
- do not move cross-package translation into `linc/src/**`
- do not build new docs or examples around this path unless the point is
  specifically repository bootstrap

Use it when it helps the repository analyze difficult headers.
Do not mistake it for the long-term boundary between packages.

## Compiler And Flavor

LINC uses the compiler as a preprocessor and ABI probe driver.

Flavor affects parsing expectations and extension handling:

- `GnuC11`
- `ClangC11`
- `StdC11`

In general:

- use `ClangC11` when the header stack is written for Clang tooling
- use `GnuC11` when the project assumes GCC-style C extensions
- use `StdC11` only when you want a stricter source profile

## Native Link Inputs During Scan

The bootstrap path can record the native inputs that the extracted API expects.
These declarations are preserved in the resulting package. The bootstrap path
does not link anything by itself; it records intent and normalized link
surface.

## Layout Probing During Scan

You can request ABI layout facts directly in the bootstrap configuration. The
resulting package will include layout evidence when probe requests succeed.

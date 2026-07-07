# Getting Started

This guide explains the structure of the ZANISTARAST Reference Implementation.

## Workspace

```
reference-implementation/

├── core/
├── verification/
├── certification/
├── registry/
├── runtime/
├── kernel/
├── ai/
├── cli/
├── benchmarks/
├── config/
├── examples/
└── docs/
```

## Execution Pipeline

```
Scientific Object
        │
        ▼
 Verification
        │
        ▼
 Certification
        │
        ▼
 Registry
        │
        ▼
 Runtime
        │
        ▼
 Kernel
        │
        ▼
 Native AI Runtime
```

## Configuration

Runtime configuration is stored in:

```
config/runtime.toml
```

## Examples

Example applications are located in:

```
examples/
```

## Tests

Tests are located inside each crate under:

```
tests/
```

## Design Goals

- Deterministic execution
- Reproducible verification
- Certified scientific workflows
- Immutable registry
- Provider-based architecture




# Provider Development

Providers integrate AI systems into the ZANISTARAST platform through a
common interface.

## ScientificProvider

Every provider implements the `ScientificProvider` trait.

```rust
pub trait ScientificProvider {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;

    fn execute(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError>;
}
```

## Registration

Providers are registered using `ProviderRegistry`.

```rust
let mut registry = ProviderRegistry::new();
registry.register(MyProvider::new());
```

## Execution

```rust
registry.execute("my-provider", &object)?;
```

## Design Rules

- Providers must be deterministic when deterministic mode is enabled.
- Providers should not modify certified scientific objects.
- Providers should return structured errors.
- Providers should expose metadata through `metadata()`.
- Providers must be thread-safe (`Send + Sync`).

## Current Providers

- NativeAiProvider



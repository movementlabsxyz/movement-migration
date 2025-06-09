# Checks

1. [`migrator`](./migration/README.md) covers logic for end-to-end validation of the migration.
2. [`node`](./node/README.md) covers logic for validating the executor-only portions of the migration. 

All tests rely on clearly stated criteria APIs which are pulled up under a `Criterion` type. Before evaluating each `Criterion` a `Prelude` is run. Hence tests are formed:

```rust
checked_migration(&mut movement_migrator, &prelude, &migration, vec![criterion_a, criterion_b, criterion_c])
			.await?;
```

The rigidity of this structure is to make it clear what the migration is accomplishing from the testing entrypoint. More standard and flexible assertions should be performed at lower levels in this repository. 

## `Criterion`
Tests are split into separate subdirectories which use their own `Criterion` types:

- `migrator` wherein a `Criterion` is evaluated against a `MovementMigrator` and a `MovementAptosMigrator` respectively. 
- `node` wherein a `Criterion` is evaluated against a `MovementNode` and a `MovementAptosNode` respectively. 


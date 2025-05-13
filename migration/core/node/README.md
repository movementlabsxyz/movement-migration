# `node`
Refers to `mtma` migrations that need to occur with full access to replica software, memory, and disk. Implementations of `node` migrations should be concerned with effects that are not issued over the wire.

These shall be implemented using the [`migration_executor_types::Migrationish`](/migration/util/executor-types/src/migration.rs) trait. Thus, expecting to be provided in types, the executor. 

> [!NOTE]
> It may be a good idea to change this to `migration_node_types`, to keep the semantics clearer. 
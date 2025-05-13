# `node`
Refers to `mtma` migrations that need to occur with full access to replica software, memory, and disk. Implementations of `node` migrations should be concerned with effects that are not issued over the wire.

These shall be implemented using the [`mtma_node_types::Migrationish`](/migration/util/node-types/src/migration.rs) trait. Thus, expecting to be provided in types, the executor. 
# `migrator`
Refers to `mtma` migrations that need to occur with a complete migrator. They contain implementations of the migration under the [`mtma_migrator_types::Migrationish`](/migration/util/migrator-types/src/migration.rs) trait. 

> [!NOTE]
> Currently, this is both the end compositional layer and where over the wire migration effects occur. But, you could later split out at `transaction` migration category. Then $\text{Migration(Migrator)}: \text{Movement Migrator} \times \text{Migration(Transaction)} \times \text{Migration(Node)} \to \text{Movement Aptos Migrator}$. 
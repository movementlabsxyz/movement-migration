<div align="center">
  <pre>
MOVEMENT => MOVEMENT APTOS
  </pre>
</div>

# `movement-to-movement-aptos`

`movement-to-movement-aptos` herein abbreviated as `mtma` is the logic and validation of migration from [`movement`](https://github.com/movementlabsxyz/movement) to [`movement-aptos`](https://github.com/movementlabsxyz/movement-aptos-core).

## Getting started
To run or work with existing migration tools check the [CLI documentation](./docs/cli/README.md).

To get an initial grounding, we recommend spending 10-15 minutes reading the [checks](./checks/README.md) and working down into various elements of the API from there. This should provide you with a sense of both the high-level requirements against which we are attempting to implement and validate as well as the approaches we are taking to perform the migration. 

Once you have completed the above, review the following:

- **[Strategies](#strategies):** describes the different layers of abstraction against which the migration is performed and validated, as well as the reason for each.
- **[Environments](#environments):** describes the different contexts (resp.) in which migration and check logic is run. These are usually wrapped up under the enum for a given **[Strategy](#strategies)**. All **[Migrations](#migrations)** should be available in in all **[Environments](#environments)**.
- **[Migrations](#migrations):** describes the available migrations which have been implemented under the **[Strategies](#strategies)** above. 
- **[Contexts](#contexts):** describes the different contexts (resp.) in which a **[Check](#checks)** can be run. Not all **[Checks](#checks)** make sense in all contexts. For example, some **[Checks](#checks)** only make sense in the `tracking` context. 
- **[Checks](#checks):** describes the available checks for migration correctness. These are often implemented for a specific **[Strategy](#strategies)** but unified under the [`migrator`](#migrator) strategy.

## Contributing

| Task | Description |
|------|-------------|
| [Upcoming Events](https://github.com/movementlabsxyz/movement-migration/issues?q=is%3Aissue%20state%3Aopen%20label%3Apriority%3Ahigh%2Cpriority%3Amedium%20label%3Aevent) | High-priority `event` issues with planned completion dates. |
| [Release Candidates](https://github.com/movementlabsxyz/movement-migration/issues?q=is%3Aissue%20state%3Aopen%20label%3Arelease-candidate%20) | Feature-complete versions linked to events. |
| [Features & Bugs](https://github.com/movementlabsxyz/movement-migration/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Cbug%20label%3Apriority%3Aurgent%2Cpriority%3Ahigh) | High-priority `feature` and `bug` issues. |

Please see [CONTRIBUTING.md](CONTRIBUTING.md) file for additional contribution guidelines.

## Strategies
The migration is organized into passes, which are categorized as follows:

### [`node`](/migration/core/node) 
Migration passes that take place with access to node processes, memory, and disk.

### [`migrator`](migration/core/migrator) 
Migration passes that take place with direct or indirect access to all other levels of abstraction--everything that is needed to migrate. 

> [!NOTE]
> An additional class of strategies `chain` and  `transaction` have been suggested but not broken out for all over-the-wire/transaction-based migration passes. Simply use the `migrator` instead. 

## Environments

Environments are the different contexts in which **[Migrations](#migrations)** are expected to run. 

> [!WARNING]
> Currently, this and its subcategories are suggestive. All **[Strategies](#strategies)**, **[Migrations](#migrations)**, and **[Checks](#checks)** have been written ad hoc and are mostly concerned with a local testing environment. 
>
> Whether or not there is sufficient complexity to realize these Environments in types remains to be seen. 

### `testing`
An environment intended for local `cargo`-based testing of the **[Migrations](#migrations)** and **[Checks](#checks)**.

### `box`
An environment intended for running **[Migrations](#migrations)** and **[Checks](#checks)** on a node which is participating in the migration and on which the appropriate signing keys are available. 

### `provisioner`
An environment intended for running **[Migrations](#migrations)** and **[Checks](#checks)** from a node which shall provision the new network. 

## Migrations

Migrations are subdivided by **[Strategy](#strategies)**.

> [!WARNING]
> This section is a **WIP** in progress. Its contents are intended as aspirational. However, links below to CLIs and documentation should ultimately be valid and currently link to informative material.

> [!NOTE]
> Multiple CLI paths are often shared for usability. Owing to the compositional approach this repository uses to CLI development, the logic should assumed be the same at each CLI path unless otherwise noted below or in the CLI documentation itself. 

### [`node`](./migration/core/node)
At the time of writing, we have planned or developed the following `node` migrations. 

####  [`mtma-node-null`](./migration/core/node/mtma-null/)
- **CLI**
  - [`mtma-node-dev migrate null`](./migration/cli/migrate-node-dev/docs/cli/README.md)
  - [`mtma-node migrate null`](./migration/cli/migrate-node/docs/cli/README.md)
  - [`mtma migration node migrate null`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma migration node migrate select --null`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --node-null`](./migration/cli/mtma/docs/cli/README.md)

`mtma-node-null` is a migration that does not attempt to make any changes to the the existing databases. It is a copying of node state files. 

#### [`mtma-node-replay`](./migration/core/node/mtma-replay)
- **CLI**
  - [`mtma-node-dev migrate replay`](./migration/cli/migrate-node-dev/docs/cli/README.md)
  - [`mtma-node migrate replay`](./migration/cli/migrate-node/docs/cli/README.md)
  - [`mtma migration node migrate replay`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma migration node migrate select --replay`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --node-replay`](./migration/cli/mtma/docs/cli/README.md)

`mtma-node-replay` is a migration which replays transactions from `movement` on a `movement-aptos` executor to derive the intended state. 

> [!WARNING]
> Currently, this migration is not passing on any **[Checks](#checks)** because of an issue with regenesis which prevents appropriately reading the execution config after replaying on the new `movement-aptos` executor. 

### [`migrator`](./migration/core/migrator/)
At the time of writing, we have planned or developed the following `migrator` migrations. 

####  [`mtma-migrator-null`](./migration/core/migrator/mtma-null/README.md)
- **CLI**
  - [`mtma-migrator-dev migrate null`](./migration/cli/migrate-migrator-dev/docs/cli/README.md)
  - [`mtma-migrator migrate null`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma migration migrator migrate null`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma migration migrator migrate select --null`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --migrator-null`](./migration/cli/mtma/docs/cli/README.md)

Rewraps the [`mtma-node-null`](#mtma-node-null) migration for the `migrator`.

####  [`mtma-migrator-pre-l1-merge`](./migration/core/migrator/mtma-pre-l1-merge/README.md)
- **CLI**
  - [`mtma-migrator-dev migrate pre-l1-merge`](./migration/cli/migrate-migrator-dev/docs/cli/README.md)
  - [`mtma-migrator migrate pre-l1-merge`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma migration migrator migrate pre-l1-merge`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma migration migrator migrate select --pre-l1-merge`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --migrator-pre-l1-merge`](./migration/cli/mtma/docs/cli/README.md)

Runs the [`pre-l1-merge`](https://github.com/movementlabsxyz/movement-migration/pull/102) OTA framework migration. 

> [!WARNING]
> This is currently not available on this branch. 

####  [`mtma-migrator-post-l1-merge`](./migration/core/migrator/mtma-post-l1-merge/README.md)
- **CLI**
  - [`mtma-migrator-dev migrate post-l1-merge`](./migration/cli/migrate-migrator-dev/docs/cli/README.md)
  - [`mtma-migrator migrate post-l1-merge`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma migration migrator migrate post-l1-merge`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma migration migrator migrate select --post-l1-merge`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --migrator-post-l1-merge`](./migration/cli/mtma/docs/cli/README.md)

Runs the [`post-l1-merge`](https://github.com/movementlabsxyz/movement-migration/issues/48#issuecomment-2828043486) OTA framework migration. 

> [!WARNING]
> This is currently not available on this branch. 

## Contexts

Contexts are the contexts in which checks are expected to run. 

> [!WARNING]
> This category and its subcategories are currently suggestive. 

> [!CAUTION]
> Not all **[Checks](#checks)** are intended to run in all Contexts. 

### `snapshot`
The context wherein the migration and its checks have access to a safe version of the node which they may modify. 

### `tracking`
The context wherein the migration is presumed to already have run and the checks will continue to track a certain set of criteria against live traffic. 

## Checks

Checks are the criteria for migration correctness. Ultimately, they are intended to be used under `mtma checked-migration` to ensure a performed migration satisfies correctness as defined by the criteria described in the Checks. 

> [!WARNING]
> This section is a **WIP** in progress. Its contents are intended as aspirational. However, links below to CLIs and documentation should ultimately be valid and currently link to informative material. 

> [!NOTE]
> Multiple CLI paths are often shared for usability. Owing to the compositional approach this repository uses to CLI development, the logic should assumed be the same at each CLI path unless otherwise noted below or in the CLI documentation itself. 


> [!NOTE]
> Several checks from [`primata/e2e-criteria`](https://github.com/movementlabsxyz/movement-migration/pull/47) are omitted here as they are still in fairly early development. 

### `snapshot`

> [!WARNING]
> We would like to pull each of these out into a separate unit-test written crate a la [`balances-equal`](./checks/migrator/checks/balances-equal/) and generally reorganize the [`checks`](./checks/) to better match the updated ontology described herein. 
>
> The need for separate crates is driven by some awkward Tokio behavior when dropping various runtimes associated with the `movement` and `movement-aptos` embedded runners. 

#### [`global-storage-includes`](./checks/node/citeria/global-storage-includes/)
- **CLI**
  - [`mtma-check-dev snapshot check global-storage-includes`](./migration/cli//check/docs/cli/README.md)
  - [`mtma-check snapshot check global-storage-includes`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma checks snapshot check global-storage-includes`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checks snapshot check select --global-storage-includes`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --check-global-storage-includes`](./migration/cli/mtma/docs/cli/README.md)
- **Tests**
  - [`global-storage-includes`](./checks/node/checks/sketchpad/src/global_storage_includes.rs)

Checks whether the global storage from `movement` is present in `movement-aptos`.

#### [`global-storage-injective`](./checks/node/citeria/global-storage-injective)
- **CLI**
  - [`mtma-check-dev snapshot check global-storage-injective`](./migration/cli//check/docs/cli/README.md)
  - [`mtma-check snapshot check global-storage-injective`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma checks snapshot check global-storage-injective`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checks snapshot check select --global-storage-injective`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --check-global-storage-injective`](./migration/cli/mtma/docs/cli/README.md)
- **Tests**
  - [`global-storage-injective`](./checks/node/checks/sketchpad/src/global_storage_injective.rs)

Checks whether the global storage from `movement` is injective into the codomain of `movement-aptos` state keys. 

#### [`global-storage-not-empty`](./checks/node/citeria/global-storage-not-empty/)
- **CLI**
  - [`mtma-check-dev snapshot check global-storage-not-empty`](./migration/cli//check/docs/cli/README.md)
  - [`mtma-check snapshot check global-storage-not-empty`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma checks snapshot check global-storage-not-empty`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checks snapshot check select --global-storage-not-empty`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --check-global-storage-not-empty`](./migration/cli/mtma/docs/cli/README.md)
- **Tests**
  - [`global-storage-not-empty`](./checks/node/checks/sketchpad/src/global_storage_not_empty.rs)

Checks that both `movement` and `movement-aptos` global storage are not empty. 

#### [`accounts-equal`](./checks/migrator/citeria/accounts-equal/)
- **CLI**
  - [`mtma-check-dev snapshot check accounts-equal`](./migration/cli//check/docs/cli/README.md)
  - [`mtma-check snapshot check accounts-equal`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma checks snapshot check accounts-equal`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checks snapshot check select --accounts-equal`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --check-accounts-equal`](./migration/cli/mtma/docs/cli/README.md)
- **Tests**
  - [`accounts-equal`](./checks/migrator/checks/sketchpad/src/accounts_equal.rs)

Checks that both `movement` and `movement-aptos` accounts are equivalent in `bcs` representation. Amongst other things, this should ensure that asymmetric cryptography for accounts is preserved. 

#### [`balances-equal`](./checks/migrator/citeria/balances-equal/)
- **CLI**
  - [`mtma-check-dev snapshot check balances-equal`](./migration/cli//check/docs/cli/README.md)
  - [`mtma-check snapshot check balances-equal`](./migration/cli/migrate-migrator/docs/cli/README.md)
  - [`mtma checks snapshot check balances-equal`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checks snapshot check select --balances-equal`](./migration/cli/mtma/docs/cli/README.md)
  - [`mtma checked-migration migrate select --check-balances-equal`](./migration/cli/mtma/docs/cli/README.md)
- **Tests**
  - [`balances-equal`](./checks/migrator/checks/balances-equal/src/balances_equal.rs)

Checks that both `movement` and `movement-aptos` balances for the native token are equal. 

### `tracking`
> [!NOTE]
> This category is completely suggestive. We haven't implemented anything here yet. 


## Organization

There are five subdirectories which progressively build on one another for node logic.

1. [`util`](./util): contains utility logic mainly reused in [`migration`](./migration).
2. [`migration`](./migration): contains the implementation of the migration.
3. [`checks`](./checks): contains checks cover the cases for the migration. We don't call these tests so as not to confuse with the strictly defined testing logic. 

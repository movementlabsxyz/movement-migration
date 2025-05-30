# `migration`
Migration contains the logic for the migration itself. It is broken into three subdirectories:

1. [`cli`](./cli): CLIs for running parts of the migration. These CLIs are also indexed from the projected root at [`docs/cli/README.md`](/docs/cli/README.md).
2. [`core`](./core/): the core APIs used in migrations. CLIs are effectively the frontends, these are the underlying programmatic APIs. 
3. [`util`](./util/): types and similar that are used across migration implementations and in [`checks`](../checks/).

> [!NOTE]
> Currently, migration CLIs containsparts of [`checks`](../checks/) because we run checked migrations. This may be refactored in forthcoming commits. 
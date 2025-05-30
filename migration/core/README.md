# `core`
`core` contains the `-core` APIs for which migration. It contains the following subdirectories. 

1. [`node`](./node/): refers to the parts of the migration which run with privileged access to the node itself--including file-based state and 
2. [`migrator`](./migrator/): refers to parts of the migration which run with all other units. Currently, `migrator` includes access to `node` via `.node()` methods on the migrators as well as access to REST clients via `.wait_for_rest_client()` methods on the migrators. Thus, `migrators` can also be where migrations are run which require transaction API usage.
3. [`mtma`](./mtma/): refers to what is ultimately exported as the correct migration. Currently, this is basically empty. It is the end-target for `mtma` migration development.
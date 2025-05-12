<div align="center">
  <pre>
MOVEMENT => MAPTOS
  </pre>
</div>

# `movement-to-movement-aptos`

Logic and validation of migration from [`movement`](https://github.com/movementlabsxyz/movement) state to [`movement-aptos`](https://github.com/movementlabsxyz/movement-aptos-core).

## Getting started
To run or work with existing migration tools check the [CLI documentation](./docs/cli/README.md).

We otherwise recommend reading the [checks](./checks/README.md) and working down into the migration from there.

## Contributing

| Task | Description |
|------|-------------|
| [Upcoming Events](https://github.com/movementlabsxyz/movement-migration/issues?q=is%3Aissue%20state%3Aopen%20label%3Apriority%3Ahigh%2Cpriority%3Amedium%20label%3Aevent) | High-priority `event` issues with planned completion dates. |
| [Release Candidates](https://github.com/movementlabsxyz/movement-migration/issues?q=is%3Aissue%20state%3Aopen%20label%3Arelease-candidate%20) | Feature-complete versions linked to events. |
| [Features & Bugs](https://github.com/movementlabsxyz/movement-migration/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Cbug%20label%3Apriority%3Aurgent%2Cpriority%3Ahigh) | High-priority `feature` and `bug` issues. |

Please see [CONTRIBUTING.md](CONTRIBUTING.md) file for additional contribution guidelines.

## Organization

There are five subdirectories which progressively build on one another for node logic.

1. [`util`](./util): contains utility logic mainly reused in [`migration`](./migration).
2. [`migration`](./migration): contains the implementation of the migration.
3. [`checks`](./checks): contains checks cover the cases for the migration. We don't call these tests so as not to confuse with the strictly defined testing logic. 

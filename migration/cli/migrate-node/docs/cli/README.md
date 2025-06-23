# Command-Line Help for `mtma-migrate-node`

This document contains the help content for the `mtma-migrate-node` command-line program.

**Command Overview:**

* [`mtma-migrate-node`↴](#mtma-migrate-node)
* [`mtma-migrate-node markdown`↴](#mtma-migrate-node-markdown)
* [`mtma-migrate-node markdown generate`↴](#mtma-migrate-node-markdown-generate)
* [`mtma-migrate-node markdown file`↴](#mtma-migrate-node-markdown-file)
* [`mtma-migrate-node markdown print`↴](#mtma-migrate-node-markdown-print)
* [`mtma-migrate-node markdown workspace`↴](#mtma-migrate-node-markdown-workspace)
* [`mtma-migrate-node migrate`↴](#mtma-migrate-node-migrate)
* [`mtma-migrate-node migrate core`↴](#mtma-migrate-node-migrate-core)
* [`mtma-migrate-node migrate select`↴](#mtma-migrate-node-migrate-select)

## `mtma-migrate-node`

The `mtma-migrate-node` CLI

**Usage:** `mtma-migrate-node [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `mtma-migrate-node markdown`

Generates markdown for the CLI

**Usage:** `mtma-migrate-node markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-migrate-node markdown generate`

Generate and update the documentation

**Usage:** `mtma-migrate-node markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-migrate-node markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-migrate-node markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-migrate-node markdown print`

Print the documentation in the shell

**Usage:** `mtma-migrate-node markdown print`



## `mtma-migrate-node markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-migrate-node markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-migrate-node migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-migrate-node migrate <COMMAND>`

###### **Subcommands:**

* `core` — Core migration over the node
* `select` — Select migration over the node



## `mtma-migrate-node migrate core`

Core migration over the node

**Usage:** `mtma-migrate-node migrate core --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



## `mtma-migrate-node migrate select`

Select migration over the node

**Usage:** `mtma-migrate-node migrate select [OPTIONS] [-- <EXTRA_ARGS>...]`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to selections

###### **Options:**

* `--environment-testing` — Enable the environment-testing selection
* `--environment-box` — Enable the environment-box selection
* `--environment-provisioner` — Enable the environment-provisioner selection
* `--null` — Enable the null selection

**Selection (1/4):** `environment-testing`
The config for the [TestingEnvironment]

Usage: --environment-testing.*

Options:
  -h, --help  Print help (see more with '--help')

**Selection (2/4):** `environment-box`
The config for the [BoxEnvironment]

Usage: --environment-box.* [OPTIONS] --rest-api-url <REST_API_URL> --db-dir <DB_DIR>

Options:
      --rest-api-url <REST_API_URL>  The rest api url of the box environment
      --db-dir <DB_DIR>              The db dir of the box environment
      --snapshot-dir <SNAPSHOT_DIR>  Whether to isolate the box environment by snapshotting the movement runner and where
                                     to store the snapshot
  -h, --help                         Print help (see more with '--help')

**Selection (3/4):** `environment-provisioner`
The config for the [ProvisionerEnvironment]

Usage: --environment-provisioner.*

Options:
  -h, --help  Print help (see more with '--help')

**Selection (4/4):** `null`
The config for the null migration

Usage: --null.*

Options:
  -h, --help  Print help (see more with '--help')





<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

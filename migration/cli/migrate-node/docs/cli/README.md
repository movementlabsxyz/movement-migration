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
* [`mtma-migrate-node migrate where`↴](#mtma-migrate-node-migrate-where)
* [`mtma-migrate-node migrate using`↴](#mtma-migrate-node-migrate-using)

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

* `where` — Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-migrate-node migrate where`

Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node migrate where --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



## `mtma-migrate-node migrate using`

Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node migrate using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for migrate



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

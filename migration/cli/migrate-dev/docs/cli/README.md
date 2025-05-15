# Command-Line Help for `mtma-migrate-dev`

This document contains the help content for the `mtma-migrate-dev` command-line program.

**Command Overview:**

* [`mtma-migrate-dev`↴](#mtma-migrate-dev)
* [`mtma-migrate-dev markdown`↴](#mtma-migrate-dev-markdown)
* [`mtma-migrate-dev markdown generate`↴](#mtma-migrate-dev-markdown-generate)
* [`mtma-migrate-dev markdown file`↴](#mtma-migrate-dev-markdown-file)
* [`mtma-migrate-dev markdown print`↴](#mtma-migrate-dev-markdown-print)
* [`mtma-migrate-dev markdown workspace`↴](#mtma-migrate-dev-markdown-workspace)
* [`mtma-migrate-dev migrate`↴](#mtma-migrate-dev-migrate)
* [`mtma-migrate-dev migrate where`↴](#mtma-migrate-dev-migrate-where)
* [`mtma-migrate-dev migrate using`↴](#mtma-migrate-dev-migrate-using)

## `mtma-migrate-dev`

The `mtma-migrate-dev` CLI

**Usage:** `mtma-migrate-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `mtma-migrate-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-migrate-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-migrate-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-migrate-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-migrate-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-migrate-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-migrate-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-migrate-dev markdown print`



## `mtma-migrate-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-migrate-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-migrate-dev migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-migrate-dev migrate <COMMAND>`

###### **Subcommands:**

* `where` — Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-migrate-dev migrate where`

Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-dev migrate where --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



## `mtma-migrate-dev migrate using`

Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-dev migrate using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for migrate



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

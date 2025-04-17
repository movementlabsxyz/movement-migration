# Command-Line Help for `mtma`

This document contains the help content for the `mtma` command-line program.

**Command Overview:**

* [`mtma`↴](#mtma)
* [`mtma markdown`↴](#mtma-markdown)
* [`mtma markdown generate`↴](#mtma-markdown-generate)
* [`mtma markdown file`↴](#mtma-markdown-file)
* [`mtma markdown print`↴](#mtma-markdown-print)
* [`mtma markdown workspace`↴](#mtma-markdown-workspace)
* [`mtma migrate`↴](#mtma-migrate)

## `mtma`

The `movement-to-aptos` CLI

**Usage:** `mtma [COMMAND]`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: migration/cli/mtma/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `mtma markdown`

Generates markdown for the CLI

**Usage:** `mtma markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma markdown generate`

Generate and update the documentation

**Usage:** `mtma markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma markdown print`

Print the documentation in the shell

**Usage:** `mtma markdown print`



## `mtma markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma migrate --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

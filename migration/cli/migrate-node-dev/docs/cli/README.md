# Command-Line Help for `mtma-migrate-node-dev`

This document contains the help content for the `mtma-migrate-node-dev` command-line program.

**Command Overview:**

* [`mtma-migrate-node-dev`↴](#mtma-migrate-node-dev)
* [`mtma-migrate-node-dev markdown`↴](#mtma-migrate-node-dev-markdown)
* [`mtma-migrate-node-dev markdown generate`↴](#mtma-migrate-node-dev-markdown-generate)
* [`mtma-migrate-node-dev markdown file`↴](#mtma-migrate-node-dev-markdown-file)
* [`mtma-migrate-node-dev markdown print`↴](#mtma-migrate-node-dev-markdown-print)
* [`mtma-migrate-node-dev markdown workspace`↴](#mtma-migrate-node-dev-markdown-workspace)
* [`mtma-migrate-node-dev migrate`↴](#mtma-migrate-node-dev-migrate)
* [`mtma-migrate-node-dev migrate null`↴](#mtma-migrate-node-dev-migrate-null)
* [`mtma-migrate-node-dev migrate null where`↴](#mtma-migrate-node-dev-migrate-null-where)
* [`mtma-migrate-node-dev migrate null using`↴](#mtma-migrate-node-dev-migrate-null-using)
* [`mtma-migrate-node-dev migrate replay`↴](#mtma-migrate-node-dev-migrate-replay)
* [`mtma-migrate-node-dev migrate replay where`↴](#mtma-migrate-node-dev-migrate-replay-where)
* [`mtma-migrate-node-dev migrate replay using`↴](#mtma-migrate-node-dev-migrate-replay-using)
* [`mtma-migrate-node-dev migrate-checked`↴](#mtma-migrate-node-dev-migrate-checked)
* [`mtma-migrate-node-dev migrate-checked where`↴](#mtma-migrate-node-dev-migrate-checked-where)
* [`mtma-migrate-node-dev migrate-checked using`↴](#mtma-migrate-node-dev-migrate-checked-using)

## `mtma-migrate-node-dev`

The `mtma-migrate-node-dev` CLI

**Usage:** `mtma-migrate-node-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos
* `migrate-checked` — Migrate from Movement to MovementAptos with checks



## `mtma-migrate-node-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-migrate-node-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-migrate-node-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-migrate-node-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-migrate-node-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-migrate-node-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-migrate-node-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-migrate-node-dev markdown print`



## `mtma-migrate-node-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-migrate-node-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-migrate-node-dev migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-migrate-node-dev migrate <COMMAND>`

###### **Subcommands:**

* `null` — Runs the null migration
* `replay` — Runs the replay migration



## `mtma-migrate-node-dev migrate null`

Runs the null migration

**Usage:** `mtma-migrate-node-dev migrate null <COMMAND>`

###### **Subcommands:**

* `where` — Run null with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run null with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-migrate-node-dev migrate null where`

Run null with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node-dev migrate null where`



## `mtma-migrate-node-dev migrate null using`

Run null with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node-dev migrate null using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-migrate-node-dev migrate replay`

Runs the replay migration

**Usage:** `mtma-migrate-node-dev migrate replay <COMMAND>`

###### **Subcommands:**

* `where` — Run replay with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run replay with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-migrate-node-dev migrate replay where`

Run replay with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node-dev migrate replay where`



## `mtma-migrate-node-dev migrate replay using`

Run replay with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node-dev migrate replay using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-migrate-node-dev migrate-checked`

Migrate from Movement to MovementAptos with checks

**Usage:** `mtma-migrate-node-dev migrate-checked <COMMAND>`

###### **Subcommands:**

* `where` — Run migratechecked with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run migratechecked with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-migrate-node-dev migrate-checked where`

Run migratechecked with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node-dev migrate-checked where [OPTIONS] --state <STATE>`

###### **Options:**

* `--state <STATE>` — The state to download
* `--state-source <STATE_SOURCE>` — The string identifying the download source if necessary



## `mtma-migrate-node-dev migrate-checked using`

Run migratechecked with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-node-dev migrate-checked using [OPTIONS] --state <STATE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for migratechecked
* `--state <STATE>` — The state to download
* `--state-source <STATE_SOURCE>` — The string identifying the download source if necessary



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `mtma-migrate-chain-dev`

This document contains the help content for the `mtma-migrate-chain-dev` command-line program.

**Command Overview:**

* [`mtma-migrate-chain-dev`↴](#mtma-migrate-chain-dev)
* [`mtma-migrate-chain-dev markdown`↴](#mtma-migrate-chain-dev-markdown)
* [`mtma-migrate-chain-dev markdown generate`↴](#mtma-migrate-chain-dev-markdown-generate)
* [`mtma-migrate-chain-dev markdown file`↴](#mtma-migrate-chain-dev-markdown-file)
* [`mtma-migrate-chain-dev markdown print`↴](#mtma-migrate-chain-dev-markdown-print)
* [`mtma-migrate-chain-dev markdown workspace`↴](#mtma-migrate-chain-dev-markdown-workspace)
* [`mtma-migrate-chain-dev migrate`↴](#mtma-migrate-chain-dev-migrate)
* [`mtma-migrate-chain-dev migrate where`↴](#mtma-migrate-chain-dev-migrate-where)
* [`mtma-migrate-chain-dev migrate using`↴](#mtma-migrate-chain-dev-migrate-using)

## `mtma-migrate-chain-dev`

The `mtma-migrate-chain-dev` CLI

**Usage:** `mtma-migrate-chain-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `mtma-migrate-chain-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-migrate-chain-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-migrate-chain-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-migrate-chain-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-migrate-chain-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-migrate-chain-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-migrate-chain-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-migrate-chain-dev markdown print`



## `mtma-migrate-chain-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-migrate-chain-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-migrate-chain-dev migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-migrate-chain-dev migrate <COMMAND>`

###### **Subcommands:**

* `where` — Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-migrate-chain-dev migrate where`

Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-chain-dev migrate where --from <FROM> --to <TO> --maptos-signer <MAPTOS_SIGNER> --da-signer <DA_SIGNER> --mcr-signer <MCR_SIGNER> --movement-args <MOVEMENT_ARGS>`

###### **Options:**

* `--from <FROM>` — The known release you are migrating from, eg elsa or biarritz-rc1
* `--to <TO>` — The known release you are migrating to, eg biarritz-rc1 or pre-l1-merge
* `--maptos-signer <MAPTOS_SIGNER>` — The canonical string for the Maptos signer used in the migration
* `--da-signer <DA_SIGNER>` — The canonical string for the DA signer used in the migration
* `--mcr-signer <MCR_SIGNER>` — The canonical string for the MCR signer used in the migration
* `--movement-args <MOVEMENT_ARGS>` — Movement configuration arguments



## `mtma-migrate-chain-dev migrate using`

Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-migrate-chain-dev migrate using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for migrate



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

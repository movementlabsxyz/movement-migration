# Command-Line Help for `movement-aptos`

This document contains the help content for the `movement-aptos` command-line program.

**Command Overview:**

* [`movement-aptos`↴](#movement-aptos)
* [`movement-aptos markdown`↴](#movement-aptos-markdown)
* [`movement-aptos markdown generate`↴](#movement-aptos-markdown-generate)
* [`movement-aptos markdown file`↴](#movement-aptos-markdown-file)
* [`movement-aptos markdown print`↴](#movement-aptos-markdown-print)
* [`movement-aptos markdown workspace`↴](#movement-aptos-markdown-workspace)
* [`movement-aptos run`↴](#movement-aptos-run)
* [`movement-aptos run where`↴](#movement-aptos-run-where)
* [`movement-aptos run using`↴](#movement-aptos-run-using)

## `movement-aptos`

The `movement-to-aptos` CLI

**Usage:** `movement-aptos [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `run` — Run the movement



## `movement-aptos markdown`

Generates markdown for the CLI

**Usage:** `movement-aptos markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `movement-aptos markdown generate`

Generate and update the documentation

**Usage:** `movement-aptos markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `movement-aptos markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `movement-aptos markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `movement-aptos markdown print`

Print the documentation in the shell

**Usage:** `movement-aptos markdown print`



## `movement-aptos markdown workspace`

Generate the documentation for the workspace

**Usage:** `movement-aptos markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `movement-aptos run`

Run the movement

**Usage:** `movement-aptos run <COMMAND>`

###### **Subcommands:**

* `where` — Run run with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run run with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `movement-aptos run where`

Run run with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `movement-aptos run where [OPTIONS] --node-config <NODE_CONFIG>`

###### **Options:**

* `--node-config <NODE_CONFIG>` — The node config to use
* `--faucet-port <FAUCET_PORT>` — The faucet port to use
* `--log-file <LOG_FILE>` — The log file to use



## `movement-aptos run using`

Run run with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `movement-aptos run using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for run



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

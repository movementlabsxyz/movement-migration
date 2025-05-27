# Command-Line Help for `movement`

This document contains the help content for the `movement` command-line program.

**Command Overview:**

* [`movement`↴](#movement)
* [`movement markdown`↴](#movement-markdown)
* [`movement markdown generate`↴](#movement-markdown-generate)
* [`movement markdown file`↴](#movement-markdown-file)
* [`movement markdown print`↴](#movement-markdown-print)
* [`movement markdown workspace`↴](#movement-markdown-workspace)
* [`movement run`↴](#movement-run)
* [`movement run where`↴](#movement-run-where)
* [`movement run using`↴](#movement-run-using)

## `movement`

The `movement-to-aptos` CLI

**Usage:** `movement [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `run` — Run the movement



## `movement markdown`

Generates markdown for the CLI

**Usage:** `movement markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `movement markdown generate`

Generate and update the documentation

**Usage:** `movement markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `movement markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `movement markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `movement markdown print`

Print the documentation in the shell

**Usage:** `movement markdown print`



## `movement markdown workspace`

Generate the documentation for the workspace

**Usage:** `movement markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `movement run`

Run the movement

**Usage:** `movement run <COMMAND>`

###### **Subcommands:**

* `where` — Run run with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run run with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `movement run where`

Run run with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `movement run where [OPTIONS] --celestia <CELESTIA> --eth <ETH>`

###### **Options:**

* `--setup` — Whether to use the setup overlay
* `--celestia <CELESTIA>` — Which celestia network to use
* `--eth <ETH>` — Which ethereum network to use
* `--biarritz-rc1-to-pre-l1-merge` — Whether to use the BiarritizRc1ToPreL1Merge overlay



## `movement run using`

Run run with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `movement run using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for run



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

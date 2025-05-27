# Command-Line Help for `mtma-track-dev`

This document contains the help content for the `mtma-track-dev` command-line program.

**Command Overview:**

* [`mtma-track-dev`↴](#mtma-track-dev)
* [`mtma-track-dev markdown`↴](#mtma-track-dev-markdown)
* [`mtma-track-dev markdown generate`↴](#mtma-track-dev-markdown-generate)
* [`mtma-track-dev markdown file`↴](#mtma-track-dev-markdown-file)
* [`mtma-track-dev markdown print`↴](#mtma-track-dev-markdown-print)
* [`mtma-track-dev markdown workspace`↴](#mtma-track-dev-markdown-workspace)
* [`mtma-track-dev track`↴](#mtma-track-dev-track)
* [`mtma-track-dev track where`↴](#mtma-track-dev-track-where)
* [`mtma-track-dev track using`↴](#mtma-track-dev-track-using)

## `mtma-track-dev`

The `mtma-track-dev` CLI

**Usage:** `mtma-track-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Track the state of the node



## `mtma-track-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-track-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-track-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-track-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-track-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-track-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-track-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-track-dev markdown print`



## `mtma-track-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-track-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-track-dev track`

Track the state of the node

**Usage:** `mtma-track-dev track <COMMAND>`

###### **Subcommands:**

* `where` — Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-track-dev track where`

Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-track-dev track where`



## `mtma-track-dev track using`

Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-track-dev track using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

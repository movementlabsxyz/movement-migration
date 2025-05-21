# Command-Line Help for `mtma-track`

This document contains the help content for the `mtma-track` command-line program.

**Command Overview:**

* [`mtma-track`↴](#mtma-track)
* [`mtma-track markdown`↴](#mtma-track-markdown)
* [`mtma-track markdown generate`↴](#mtma-track-markdown-generate)
* [`mtma-track markdown file`↴](#mtma-track-markdown-file)
* [`mtma-track markdown print`↴](#mtma-track-markdown-print)
* [`mtma-track markdown workspace`↴](#mtma-track-markdown-workspace)
* [`mtma-track track`↴](#mtma-track-track)
* [`mtma-track track where`↴](#mtma-track-track-where)
* [`mtma-track track using`↴](#mtma-track-track-using)

## `mtma-track`

The `mtma-track` CLI

**Usage:** `mtma-track [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Track the state of the node



## `mtma-track markdown`

Generates markdown for the CLI

**Usage:** `mtma-track markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-track markdown generate`

Generate and update the documentation

**Usage:** `mtma-track markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-track markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-track markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-track markdown print`

Print the documentation in the shell

**Usage:** `mtma-track markdown print`



## `mtma-track markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-track markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-track track`

Track the state of the node

**Usage:** `mtma-track track <COMMAND>`

###### **Subcommands:**

* `where` — Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-track track where`

Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-track track where`



## `mtma-track track using`

Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-track track using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

# Command-Line Help for `mtma-check-dev`

This document contains the help content for the `mtma-check-dev` command-line program.

**Command Overview:**

* [`mtma-check-dev`↴](#mtma-check-dev)
* [`mtma-check-dev markdown`↴](#mtma-check-dev-markdown)
* [`mtma-check-dev markdown generate`↴](#mtma-check-dev-markdown-generate)
* [`mtma-check-dev markdown file`↴](#mtma-check-dev-markdown-file)
* [`mtma-check-dev markdown print`↴](#mtma-check-dev-markdown-print)
* [`mtma-check-dev markdown workspace`↴](#mtma-check-dev-markdown-workspace)
* [`mtma-check-dev all`↴](#mtma-check-dev-all)
* [`mtma-check-dev all where`↴](#mtma-check-dev-all-where)
* [`mtma-check-dev all using`↴](#mtma-check-dev-all-using)
* [`mtma-check-dev all-of`↴](#mtma-check-dev-all-of)
* [`mtma-check-dev all-of where`↴](#mtma-check-dev-all-of-where)
* [`mtma-check-dev all-of using`↴](#mtma-check-dev-all-of-using)
* [`mtma-check-dev global-storage-includes`↴](#mtma-check-dev-global-storage-includes)
* [`mtma-check-dev global-storage-includes where`↴](#mtma-check-dev-global-storage-includes-where)
* [`mtma-check-dev global-storage-includes using`↴](#mtma-check-dev-global-storage-includes-using)
* [`mtma-check-dev global-storage-injective`↴](#mtma-check-dev-global-storage-injective)
* [`mtma-check-dev global-storage-injective where`↴](#mtma-check-dev-global-storage-injective-where)
* [`mtma-check-dev global-storage-injective using`↴](#mtma-check-dev-global-storage-injective-using)
* [`mtma-check-dev global-storage-not-empty`↴](#mtma-check-dev-global-storage-not-empty)
* [`mtma-check-dev global-storage-not-empty where`↴](#mtma-check-dev-global-storage-not-empty-where)
* [`mtma-check-dev global-storage-not-empty using`↴](#mtma-check-dev-global-storage-not-empty-using)

## `mtma-check-dev`

The `mtma-check-dev` CLI

**Usage:** `mtma-check-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `all` — Check all criteria
* `all-of` — Check all of the specified criteria
* `global-storage-includes` — Check the Global Storage Includes criterion
* `global-storage-injective` — Check the Global Storage Injective criterion
* `global-storage-not-empty` — Check the Global Storage Not Empty criterion



## `mtma-check-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-check-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-check-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-check-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-check-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-check-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-check-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-check-dev markdown print`



## `mtma-check-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-check-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-check-dev all`

Check all criteria

**Usage:** `mtma-check-dev all <COMMAND>`

###### **Subcommands:**

* `where` — Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check-dev all where`

Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev all where`



## `mtma-check-dev all using`

Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev all using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check-dev all-of`

Check all of the specified criteria

**Usage:** `mtma-check-dev all-of <COMMAND>`

###### **Subcommands:**

* `where` — Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check-dev all-of where`

Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev all-of where`



## `mtma-check-dev all-of using`

Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev all-of using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check-dev global-storage-includes`

Check the Global Storage Includes criterion

**Usage:** `mtma-check-dev global-storage-includes <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check-dev global-storage-includes where`

Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev global-storage-includes where`



## `mtma-check-dev global-storage-includes using`

Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev global-storage-includes using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check-dev global-storage-injective`

Check the Global Storage Injective criterion

**Usage:** `mtma-check-dev global-storage-injective <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check-dev global-storage-injective where`

Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev global-storage-injective where`



## `mtma-check-dev global-storage-injective using`

Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev global-storage-injective using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check-dev global-storage-not-empty`

Check the Global Storage Not Empty criterion

**Usage:** `mtma-check-dev global-storage-not-empty <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check-dev global-storage-not-empty where`

Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev global-storage-not-empty where`



## `mtma-check-dev global-storage-not-empty using`

Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check-dev global-storage-not-empty using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

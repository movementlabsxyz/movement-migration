# Command-Line Help for `mtma-check`

This document contains the help content for the `mtma-check` command-line program.

**Command Overview:**

* [`mtma-check`↴](#mtma-check)
* [`mtma-check markdown`↴](#mtma-check-markdown)
* [`mtma-check markdown generate`↴](#mtma-check-markdown-generate)
* [`mtma-check markdown file`↴](#mtma-check-markdown-file)
* [`mtma-check markdown print`↴](#mtma-check-markdown-print)
* [`mtma-check markdown workspace`↴](#mtma-check-markdown-workspace)
* [`mtma-check all`↴](#mtma-check-all)
* [`mtma-check all where`↴](#mtma-check-all-where)
* [`mtma-check all using`↴](#mtma-check-all-using)
* [`mtma-check all-of`↴](#mtma-check-all-of)
* [`mtma-check all-of where`↴](#mtma-check-all-of-where)
* [`mtma-check all-of using`↴](#mtma-check-all-of-using)
* [`mtma-check global-storage-includes`↴](#mtma-check-global-storage-includes)
* [`mtma-check global-storage-includes where`↴](#mtma-check-global-storage-includes-where)
* [`mtma-check global-storage-includes using`↴](#mtma-check-global-storage-includes-using)
* [`mtma-check global-storage-injective`↴](#mtma-check-global-storage-injective)
* [`mtma-check global-storage-injective where`↴](#mtma-check-global-storage-injective-where)
* [`mtma-check global-storage-injective using`↴](#mtma-check-global-storage-injective-using)
* [`mtma-check global-storage-not-empty`↴](#mtma-check-global-storage-not-empty)
* [`mtma-check global-storage-not-empty where`↴](#mtma-check-global-storage-not-empty-where)
* [`mtma-check global-storage-not-empty using`↴](#mtma-check-global-storage-not-empty-using)

## `mtma-check`

The `mtma-check` CLI

**Usage:** `mtma-check [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `all` — Check all criteria
* `all-of` — Check all of the specified criteria
* `global-storage-includes` — Check the Global Storage Includes criterion
* `global-storage-injective` — Check the Global Storage Injective criterion
* `global-storage-not-empty` — Check the Global Storage Not Empty criterion



## `mtma-check markdown`

Generates markdown for the CLI

**Usage:** `mtma-check markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-check markdown generate`

Generate and update the documentation

**Usage:** `mtma-check markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-check markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-check markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-check markdown print`

Print the documentation in the shell

**Usage:** `mtma-check markdown print`



## `mtma-check markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-check markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-check all`

Check all criteria

**Usage:** `mtma-check all <COMMAND>`

###### **Subcommands:**

* `where` — Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check all where`

Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check all where`



## `mtma-check all using`

Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check all using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check all-of`

Check all of the specified criteria

**Usage:** `mtma-check all-of <COMMAND>`

###### **Subcommands:**

* `where` — Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check all-of where`

Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check all-of where`



## `mtma-check all-of using`

Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check all-of using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check global-storage-includes`

Check the Global Storage Includes criterion

**Usage:** `mtma-check global-storage-includes <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check global-storage-includes where`

Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check global-storage-includes where`



## `mtma-check global-storage-includes using`

Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check global-storage-includes using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check global-storage-injective`

Check the Global Storage Injective criterion

**Usage:** `mtma-check global-storage-injective <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check global-storage-injective where`

Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check global-storage-injective where`



## `mtma-check global-storage-injective using`

Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check global-storage-injective using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-check global-storage-not-empty`

Check the Global Storage Not Empty criterion

**Usage:** `mtma-check global-storage-not-empty <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-check global-storage-not-empty where`

Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check global-storage-not-empty where`



## `mtma-check global-storage-not-empty using`

Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-check global-storage-not-empty using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

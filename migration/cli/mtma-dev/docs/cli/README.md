# Command-Line Help for `mtma-dev`

This document contains the help content for the `mtma-dev` command-line program.

**Command Overview:**

* [`mtma-dev`↴](#mtma-dev)
* [`mtma-dev markdown`↴](#mtma-dev-markdown)
* [`mtma-dev markdown generate`↴](#mtma-dev-markdown-generate)
* [`mtma-dev markdown file`↴](#mtma-dev-markdown-file)
* [`mtma-dev markdown print`↴](#mtma-dev-markdown-print)
* [`mtma-dev markdown workspace`↴](#mtma-dev-markdown-workspace)
* [`mtma-dev check`↴](#mtma-dev-check)
* [`mtma-dev check dev`↴](#mtma-dev-check-dev)
* [`mtma-dev check dev markdown`↴](#mtma-dev-check-dev-markdown)
* [`mtma-dev check dev markdown generate`↴](#mtma-dev-check-dev-markdown-generate)
* [`mtma-dev check dev markdown file`↴](#mtma-dev-check-dev-markdown-file)
* [`mtma-dev check dev markdown print`↴](#mtma-dev-check-dev-markdown-print)
* [`mtma-dev check dev markdown workspace`↴](#mtma-dev-check-dev-markdown-workspace)
* [`mtma-dev check dev all`↴](#mtma-dev-check-dev-all)
* [`mtma-dev check dev all where`↴](#mtma-dev-check-dev-all-where)
* [`mtma-dev check dev all using`↴](#mtma-dev-check-dev-all-using)
* [`mtma-dev check dev all-of`↴](#mtma-dev-check-dev-all-of)
* [`mtma-dev check dev all-of where`↴](#mtma-dev-check-dev-all-of-where)
* [`mtma-dev check dev all-of using`↴](#mtma-dev-check-dev-all-of-using)
* [`mtma-dev check dev global-storage-includes`↴](#mtma-dev-check-dev-global-storage-includes)
* [`mtma-dev check dev global-storage-includes where`↴](#mtma-dev-check-dev-global-storage-includes-where)
* [`mtma-dev check dev global-storage-includes using`↴](#mtma-dev-check-dev-global-storage-includes-using)
* [`mtma-dev check dev global-storage-injective`↴](#mtma-dev-check-dev-global-storage-injective)
* [`mtma-dev check dev global-storage-injective where`↴](#mtma-dev-check-dev-global-storage-injective-where)
* [`mtma-dev check dev global-storage-injective using`↴](#mtma-dev-check-dev-global-storage-injective-using)
* [`mtma-dev check dev global-storage-not-empty`↴](#mtma-dev-check-dev-global-storage-not-empty)
* [`mtma-dev check dev global-storage-not-empty where`↴](#mtma-dev-check-dev-global-storage-not-empty-where)
* [`mtma-dev check dev global-storage-not-empty using`↴](#mtma-dev-check-dev-global-storage-not-empty-using)
* [`mtma-dev check prod`↴](#mtma-dev-check-prod)
* [`mtma-dev check prod markdown`↴](#mtma-dev-check-prod-markdown)
* [`mtma-dev check prod markdown generate`↴](#mtma-dev-check-prod-markdown-generate)
* [`mtma-dev check prod markdown file`↴](#mtma-dev-check-prod-markdown-file)
* [`mtma-dev check prod markdown print`↴](#mtma-dev-check-prod-markdown-print)
* [`mtma-dev check prod markdown workspace`↴](#mtma-dev-check-prod-markdown-workspace)
* [`mtma-dev check prod all`↴](#mtma-dev-check-prod-all)
* [`mtma-dev check prod all where`↴](#mtma-dev-check-prod-all-where)
* [`mtma-dev check prod all using`↴](#mtma-dev-check-prod-all-using)
* [`mtma-dev check prod all-of`↴](#mtma-dev-check-prod-all-of)
* [`mtma-dev check prod all-of where`↴](#mtma-dev-check-prod-all-of-where)
* [`mtma-dev check prod all-of using`↴](#mtma-dev-check-prod-all-of-using)
* [`mtma-dev check prod global-storage-includes`↴](#mtma-dev-check-prod-global-storage-includes)
* [`mtma-dev check prod global-storage-includes where`↴](#mtma-dev-check-prod-global-storage-includes-where)
* [`mtma-dev check prod global-storage-includes using`↴](#mtma-dev-check-prod-global-storage-includes-using)
* [`mtma-dev check prod global-storage-injective`↴](#mtma-dev-check-prod-global-storage-injective)
* [`mtma-dev check prod global-storage-injective where`↴](#mtma-dev-check-prod-global-storage-injective-where)
* [`mtma-dev check prod global-storage-injective using`↴](#mtma-dev-check-prod-global-storage-injective-using)
* [`mtma-dev check prod global-storage-not-empty`↴](#mtma-dev-check-prod-global-storage-not-empty)
* [`mtma-dev check prod global-storage-not-empty where`↴](#mtma-dev-check-prod-global-storage-not-empty-where)
* [`mtma-dev check prod global-storage-not-empty using`↴](#mtma-dev-check-prod-global-storage-not-empty-using)
* [`mtma-dev migrate`↴](#mtma-dev-migrate)
* [`mtma-dev migrate dev`↴](#mtma-dev-migrate-dev)
* [`mtma-dev migrate dev markdown`↴](#mtma-dev-migrate-dev-markdown)
* [`mtma-dev migrate dev markdown generate`↴](#mtma-dev-migrate-dev-markdown-generate)
* [`mtma-dev migrate dev markdown file`↴](#mtma-dev-migrate-dev-markdown-file)
* [`mtma-dev migrate dev markdown print`↴](#mtma-dev-migrate-dev-markdown-print)
* [`mtma-dev migrate dev markdown workspace`↴](#mtma-dev-migrate-dev-markdown-workspace)
* [`mtma-dev migrate dev migrate`↴](#mtma-dev-migrate-dev-migrate)
* [`mtma-dev migrate dev migrate where`↴](#mtma-dev-migrate-dev-migrate-where)
* [`mtma-dev migrate dev migrate using`↴](#mtma-dev-migrate-dev-migrate-using)
* [`mtma-dev migrate prod`↴](#mtma-dev-migrate-prod)
* [`mtma-dev migrate prod markdown`↴](#mtma-dev-migrate-prod-markdown)
* [`mtma-dev migrate prod markdown generate`↴](#mtma-dev-migrate-prod-markdown-generate)
* [`mtma-dev migrate prod markdown file`↴](#mtma-dev-migrate-prod-markdown-file)
* [`mtma-dev migrate prod markdown print`↴](#mtma-dev-migrate-prod-markdown-print)
* [`mtma-dev migrate prod markdown workspace`↴](#mtma-dev-migrate-prod-markdown-workspace)
* [`mtma-dev migrate prod migrate`↴](#mtma-dev-migrate-prod-migrate)
* [`mtma-dev migrate prod migrate where`↴](#mtma-dev-migrate-prod-migrate-where)
* [`mtma-dev migrate prod migrate using`↴](#mtma-dev-migrate-prod-migrate-using)
* [`mtma-dev track`↴](#mtma-dev-track)
* [`mtma-dev track dev`↴](#mtma-dev-track-dev)
* [`mtma-dev track dev markdown`↴](#mtma-dev-track-dev-markdown)
* [`mtma-dev track dev markdown generate`↴](#mtma-dev-track-dev-markdown-generate)
* [`mtma-dev track dev markdown file`↴](#mtma-dev-track-dev-markdown-file)
* [`mtma-dev track dev markdown print`↴](#mtma-dev-track-dev-markdown-print)
* [`mtma-dev track dev markdown workspace`↴](#mtma-dev-track-dev-markdown-workspace)
* [`mtma-dev track dev track`↴](#mtma-dev-track-dev-track)
* [`mtma-dev track dev track where`↴](#mtma-dev-track-dev-track-where)
* [`mtma-dev track dev track using`↴](#mtma-dev-track-dev-track-using)
* [`mtma-dev track prod`↴](#mtma-dev-track-prod)
* [`mtma-dev track prod markdown`↴](#mtma-dev-track-prod-markdown)
* [`mtma-dev track prod markdown generate`↴](#mtma-dev-track-prod-markdown-generate)
* [`mtma-dev track prod markdown file`↴](#mtma-dev-track-prod-markdown-file)
* [`mtma-dev track prod markdown print`↴](#mtma-dev-track-prod-markdown-print)
* [`mtma-dev track prod markdown workspace`↴](#mtma-dev-track-prod-markdown-workspace)
* [`mtma-dev track prod track`↴](#mtma-dev-track-prod-track)
* [`mtma-dev track prod track where`↴](#mtma-dev-track-prod-track-where)
* [`mtma-dev track prod track using`↴](#mtma-dev-track-prod-track-using)

## `mtma-dev`

The `movement-to-aptos` CLI

**Usage:** `mtma-dev [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `check` — Check the state for given migration criteria
* `migrate` — Migrate from Movement to MovementAptos
* `track` — Track the state of the node for real-time comparison



## `mtma-dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev markdown print`



## `mtma-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev check`

Check the state for given migration criteria

**Usage:** `mtma-dev check <COMMAND>`

###### **Subcommands:**

* `dev` — Runs the null migration
* `prod` — Runs the replay migration



## `mtma-dev check dev`

Runs the null migration

**Usage:** `mtma-dev check dev <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `all` — Check all criteria
* `all-of` — Check all of the specified criteria
* `global-storage-includes` — Check the Global Storage Includes criterion
* `global-storage-injective` — Check the Global Storage Injective criterion
* `global-storage-not-empty` — Check the Global Storage Not Empty criterion



## `mtma-dev check dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev check dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev check dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev check dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev check dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev check dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev check dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev check dev markdown print`



## `mtma-dev check dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev check dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev check dev all`

Check all criteria

**Usage:** `mtma-dev check dev all <COMMAND>`

###### **Subcommands:**

* `where` — Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check dev all where`

Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev all where`



## `mtma-dev check dev all using`

Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev all using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check dev all-of`

Check all of the specified criteria

**Usage:** `mtma-dev check dev all-of <COMMAND>`

###### **Subcommands:**

* `where` — Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check dev all-of where`

Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev all-of where`



## `mtma-dev check dev all-of using`

Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev all-of using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check dev global-storage-includes`

Check the Global Storage Includes criterion

**Usage:** `mtma-dev check dev global-storage-includes <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check dev global-storage-includes where`

Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev global-storage-includes where`



## `mtma-dev check dev global-storage-includes using`

Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev global-storage-includes using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check dev global-storage-injective`

Check the Global Storage Injective criterion

**Usage:** `mtma-dev check dev global-storage-injective <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check dev global-storage-injective where`

Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev global-storage-injective where`



## `mtma-dev check dev global-storage-injective using`

Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev global-storage-injective using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check dev global-storage-not-empty`

Check the Global Storage Not Empty criterion

**Usage:** `mtma-dev check dev global-storage-not-empty <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check dev global-storage-not-empty where`

Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev global-storage-not-empty where`



## `mtma-dev check dev global-storage-not-empty using`

Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check dev global-storage-not-empty using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check prod`

Runs the replay migration

**Usage:** `mtma-dev check prod <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `all` — Check all criteria
* `all-of` — Check all of the specified criteria
* `global-storage-includes` — Check the Global Storage Includes criterion
* `global-storage-injective` — Check the Global Storage Injective criterion
* `global-storage-not-empty` — Check the Global Storage Not Empty criterion



## `mtma-dev check prod markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev check prod markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev check prod markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev check prod markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev check prod markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev check prod markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev check prod markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev check prod markdown print`



## `mtma-dev check prod markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev check prod markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev check prod all`

Check all criteria

**Usage:** `mtma-dev check prod all <COMMAND>`

###### **Subcommands:**

* `where` — Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check prod all where`

Run allcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod all where`



## `mtma-dev check prod all using`

Run allcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod all using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check prod all-of`

Check all of the specified criteria

**Usage:** `mtma-dev check prod all-of <COMMAND>`

###### **Subcommands:**

* `where` — Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check prod all-of where`

Run allofcheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod all-of where`



## `mtma-dev check prod all-of using`

Run allofcheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod all-of using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check prod global-storage-includes`

Check the Global Storage Includes criterion

**Usage:** `mtma-dev check prod global-storage-includes <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check prod global-storage-includes where`

Run globalstorageincludescheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod global-storage-includes where`



## `mtma-dev check prod global-storage-includes using`

Run globalstorageincludescheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod global-storage-includes using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check prod global-storage-injective`

Check the Global Storage Injective criterion

**Usage:** `mtma-dev check prod global-storage-injective <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check prod global-storage-injective where`

Run globalstorageinjectivecheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod global-storage-injective where`



## `mtma-dev check prod global-storage-injective using`

Run globalstorageinjectivecheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod global-storage-injective using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev check prod global-storage-not-empty`

Check the Global Storage Not Empty criterion

**Usage:** `mtma-dev check prod global-storage-not-empty <COMMAND>`

###### **Subcommands:**

* `where` — Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev check prod global-storage-not-empty where`

Run globalstoragenotemptycheck with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod global-storage-not-empty where`



## `mtma-dev check prod global-storage-not-empty using`

Run globalstoragenotemptycheck with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev check prod global-storage-not-empty using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-dev migrate <COMMAND>`

###### **Subcommands:**

* `dev` — Runs the null migration
* `prod` — Runs the replay migration



## `mtma-dev migrate dev`

Runs the null migration

**Usage:** `mtma-dev migrate dev <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `mtma-dev migrate dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev migrate dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev migrate dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev migrate dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev migrate dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev migrate dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev migrate dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev migrate dev markdown print`



## `mtma-dev migrate dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev migrate dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev migrate dev migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-dev migrate dev migrate <COMMAND>`

###### **Subcommands:**

* `where` — Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev migrate dev migrate where`

Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev migrate dev migrate where --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



## `mtma-dev migrate dev migrate using`

Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev migrate dev migrate using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for migrate



## `mtma-dev migrate prod`

Runs the replay migration

**Usage:** `mtma-dev migrate prod <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `migrate` — Migrate from Movement to MovementAptos



## `mtma-dev migrate prod markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev migrate prod markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev migrate prod markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev migrate prod markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev migrate prod markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev migrate prod markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev migrate prod markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev migrate prod markdown print`



## `mtma-dev migrate prod markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev migrate prod markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev migrate prod migrate`

Migrate from Movement to MovementAptos

**Usage:** `mtma-dev migrate prod migrate <COMMAND>`

###### **Subcommands:**

* `where` — Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev migrate prod migrate where`

Run migrate with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev migrate prod migrate where --movement-state-db-path <MOVEMENT_STATE_DB_PATH> --movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>`

###### **Options:**

* `--movement-state-db-path <MOVEMENT_STATE_DB_PATH>` — The path to the input Movement state database
* `--movement-aptos-state-db-path <MOVEMENT_APTOS_STATE_DB_PATH>` — The path to the output MovementAptos state database



## `mtma-dev migrate prod migrate using`

Run migrate with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev migrate prod migrate using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>` — Path to the config file for migrate



## `mtma-dev track`

Track the state of the node for real-time comparison

**Usage:** `mtma-dev track <COMMAND>`

###### **Subcommands:**

* `dev` — Runs the null migration
* `prod` — Runs the replay migration



## `mtma-dev track dev`

Runs the null migration

**Usage:** `mtma-dev track dev <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Track the state of the node



## `mtma-dev track dev markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev track dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev track dev markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev track dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev track dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev track dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev track dev markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev track dev markdown print`



## `mtma-dev track dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev track dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev track dev track`

Track the state of the node

**Usage:** `mtma-dev track dev track <COMMAND>`

###### **Subcommands:**

* `where` — Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev track dev track where`

Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track dev track where`



## `mtma-dev track dev track using`

Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track dev track using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



## `mtma-dev track prod`

Runs the replay migration

**Usage:** `mtma-dev track prod <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `track` — Track the state of the node



## `mtma-dev track prod markdown`

Generates markdown for the CLI

**Usage:** `mtma-dev track prod markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mtma-dev track prod markdown generate`

Generate and update the documentation

**Usage:** `mtma-dev track prod markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mtma-dev track prod markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mtma-dev track prod markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mtma-dev track prod markdown print`

Print the documentation in the shell

**Usage:** `mtma-dev track prod markdown print`



## `mtma-dev track prod markdown workspace`

Generate the documentation for the workspace

**Usage:** `mtma-dev track prod markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mtma-dev track prod track`

Track the state of the node

**Usage:** `mtma-dev track prod track <COMMAND>`

###### **Subcommands:**

* `where` — Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mtma-dev track prod track where`

Run track with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track prod track where`



## `mtma-dev track prod track using`

Run track with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mtma-dev track prod track using [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

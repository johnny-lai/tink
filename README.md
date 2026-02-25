# tink

A CLI tool for managing [Zed](https://zed.dev) debug launch profiles. It reads and writes `.zed/debug.json` in the current working directory.

## Installation

```sh
cargo install --path .
```

## Usage

```
tink <COMMAND>
```

### Commands

| Command   | Description                                           |
|-----------|-------------------------------------------------------|
| `add`     | Add a new profile (errors if label already exists)    |
| `replace` | Upsert a profile (replaces if exists, adds if not)    |

### Flags

| Flag                    | Description                                                                    |
|-------------------------|--------------------------------------------------------------------------------|
| `-l, --language <LANG>` | Debug adapter language (`go`, `rust`, `c`, `cpp`, `python`, `js`, `ts`, `php`) |
| `-n, --label <NAME>`    | Display name for the profile (default: `Debug <program>`)                      |
| `-- <PROGRAM> [ARGS]`   | Program path and its arguments                                                 |

## Examples

Add a profile for a Go binary:

```sh
tink add -l go -- ./bin/myapp --port 8080
```

Add a profile with a custom label:

```sh
tink add -l rust -n "Debug server" -- ./target/debug/myapp
```

Replace (upsert) an existing profile:

```sh
tink replace -l rust -n "Debug server" -- ./target/debug/myapp --verbose
```

## Supported Languages

| Language flag         | Zed adapter  |
|-----------------------|--------------|
| `go`                  | Go           |
| `rust`                | CodeLLDB     |
| `c`, `cpp`, `c++`     | CodeLLDB     |
| `python`              | Debugpy      |
| `javascript`, `js`    | JavaScript   |
| `typescript`, `ts`    | JavaScript   |
| `php`                 | PHP          |

## Output

Profiles are written to `.zed/debug.json` as a JSON array. Example output:

```json
[
  {
    "label": "Debug server",
    "adapter": "CodeLLDB",
    "request": "launch",
    "program": "./target/debug/myapp",
    "args": ["--verbose"]
  }
]
```

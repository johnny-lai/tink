# tink

A CLI tool for managing debug launch profiles for [Zed](https://zed.dev) and [VSCode](https://code.visualstudio.com). It reads and writes config files in the current working directory.

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
| `-t, --target <TARGET>` | Target editor: `zed` (default) or `vscode`                                     |
| `-- <PROGRAM> [ARGS]`   | Program path and its arguments                                                 |

## Examples

Add a Zed profile for a Go binary:

```sh
tink add -l go -- ./bin/myapp --port 8080
```

Add a VSCode profile with a custom label:

```sh
tink add -l rust -t vscode -n "Debug server" -- ./target/debug/myapp
```

Replace (upsert) an existing profile:

```sh
tink replace -l rust -n "Debug server" -- ./target/debug/myapp --verbose
```

## Supported Languages

| Language flag          | Zed adapter  | VSCode type |
|------------------------|--------------|-------------|
| `go`                   | Go           | go          |
| `rust`, `c`, `cpp`     | CodeLLDB     | lldb        |
| `python`               | Debugpy      | debugpy     |
| `javascript`, `js`     | JavaScript   | node        |
| `typescript`, `ts`     | JavaScript   | node        |
| `php`                  | PHP          | php         |

## Output

### Zed

Profiles are written to `.zed/debug.json` as a JSON array:

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

### VSCode

Configurations are written to `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug server",
      "type": "lldb",
      "request": "launch",
      "program": "./target/debug/myapp",
      "args": ["--verbose"]
    }
  ]
}
```

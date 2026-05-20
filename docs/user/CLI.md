# Mercurio CLI Guide

## Overview

The public CLI is one cohesive `mercurio` binary with `project`, `parse`, `compile`, `query`, `evaluate`, `lint`, `package`, and `completions` subcommands.

`parse`, `compile`, `query`, and `evaluate` can read source input from:

- `--file PATH`
- `--text TEXT`
- `--url URL`

Inline text defaults to SysML. File and URL input infer language from `.sysml` or `.kerml`. Use `--language kerml` for inline KerML.

## Quick Commands

Show CLI help:

```powershell
mercurio --help
```

Parse inline SysML:

```powershell
mercurio parse --text "package Demo { part def Vehicle; }"
```

Compile inline KerML:

```powershell
mercurio compile --text "package Demo { classifier Vehicle; }" --language kerml
```

Lint a file:

```powershell
mercurio lint --file "examples/src/examples/Simple Tests/PartTest.sysml"
```

## Parse SysML Or KerML

Parse one file and print a syntax summary:

```powershell
mercurio parse --file "examples/src/examples/Simple Tests/PartTest.sysml"
```

Parse inline SysML:

```powershell
mercurio parse --text "package Demo { part def Vehicle; }"
```

Emit the syntax AST as JSON:

```powershell
mercurio parse --file "examples/src/examples/Simple Tests/PartTest.sysml" --format json
```

## Compile To KIR

Compile a file to KIR using the default standard library:

```powershell
mercurio compile --file "examples/src/examples/Simple Tests/PartTest.sysml"
```

Emit the KIR document as JSON:

```powershell
mercurio compile --text "package Demo { part def Vehicle; }" --format json
```

Override the standard library:

```powershell
mercurio compile --file model.sysml --stdlib resources/stdlib.full.kir.json
```

Compile source from a network URL:

```powershell
mercurio compile --url https://example.com/models/vehicle.sysml
```

## Lint SysML Or KerML

Lint one file:

```powershell
mercurio lint --file "examples/src/examples/Simple Tests/PartTest.sysml"
```

Lint every `.sysml` and `.kerml` file under a directory:

```powershell
mercurio lint --file "examples/src/examples/Simple Tests"
```

Emit JSON diagnostics:

```powershell
mercurio lint --file "examples/src/examples/Simple Tests" --format json
```

Fail when warnings are present, useful for CI:

```powershell
mercurio lint --file "examples/src/examples/Simple Tests" --warnings-as-errors
```

## Shell Completions

Generate completion scripts:

```powershell
mercurio completions powershell
```

Supported shells are `bash`, `elvish`, `fish`, `powershell`, and `zsh`.

For PowerShell:

```powershell
mercurio completions powershell > mercurio-completions.ps1
. .\mercurio-completions.ps1
```

For Bash or Zsh:

```bash
mercurio completions bash > ~/.local/share/bash-completion/completions/mercurio
mercurio completions zsh > ~/.zfunc/_mercurio
```

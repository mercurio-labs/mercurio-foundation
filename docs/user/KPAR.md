# KPAR Packages

## What A KPAR Is

A `.kpar` package is a source-backed model package. It contains SysML/KerML sources plus package metadata and can be used later as a model input or library dependency.

KPARs are useful when you want to:

- distribute a reusable model library
- compile or evaluate a packaged model
- mount read-only dependencies through a project descriptor
- preserve package metadata alongside source

## Build A Package

Build a source-backed `.kpar` package from a model file:

```powershell
mercurio package build --file model.sysml --out model.kpar
```

Build a package from every `.sysml` and `.kerml` file under a directory:

```powershell
mercurio package build --file examples/src/examples --out examples.kpar
```

Override package metadata:

```powershell
mercurio package build --file model.sysml --out model.kpar --name Demo --version 0.1.0
```

## Compile A KPAR

Compile a KPAR package directly as a model input:

```powershell
mercurio compile --kpar model.kpar --format json
```

Compile a package from a URL when the URL ends in `.kpar`:

```powershell
mercurio compile --url https://example.com/packages/domain.kpar --format json
```

## Evaluate From A KPAR

Evaluate a derived feature from a KPAR package:

```powershell
mercurio evaluate --kpar model.kpar --feature totalMass --owner Demo.Vehicle
```

## Use A KPAR As A Library

Add a KPAR dependency in `mercurio-project.json`:

```json
{
  "version": 1,
  "name": "My Model",
  "libraries": [
    {
      "id": "domain-lib",
      "provider": {
        "kind": "kpar_file",
        "path": "libs/domain.kpar"
      }
    }
  ]
}
```

Relative paths are resolved from the descriptor location.

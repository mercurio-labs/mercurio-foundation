# Project Descriptors

## Create A Project

Create a new project directory with a project descriptor and sample SysML file:

```powershell
mercurio project new my-model --name "My Model"
```

This writes:

- `my-model/.mercurio-project.json`
- `my-model/src/main.sysml`

Use `--force` to write the scaffold files into an existing non-empty directory, and `--quiet` to suppress the creation summary.

## Descriptor Shape

The project descriptor is the root-level `.mercurio-project.json` file.

The generated descriptor is intentionally small:

```json
{
  "version": 1,
  "name": "My Model",
  "libraries": []
}
```

Fields:

- `version`: descriptor schema version. The current version is `1`; omitted values default to `1`.
- `name`: optional display name for the project.
- `libraries`: baseline and dependency libraries. If no baseline library is declared, Mercurio uses the bundled standard library locator.

## Library Providers

Each entry in `libraries` has this shape:

```json
{
  "id": "domain-lib",
  "role": "dependency",
  "provider": {
    "kind": "kpar_file",
    "path": "libs/domain.kpar"
  }
}
```

Supported provider `kind` values:

- `bundled_stdlib`: use Mercurio's bundled standard library; no extra fields.
- `precompiled_kir_artifact`: load a KIR JSON file with `path`.
- `sysml_directory`: load all SysML/KerML sources under `path`.
- `kpar_file`: load one `.kpar` package file from `path`.
- `package_set_directory`: load a package from a local package-set directory using `path` and `entry`.

Relative provider paths are resolved from the directory containing `.mercurio-project.json`.

## Package Locator Provider

Mercurio supports a package-locator provider for portable KPAR dependencies:

```json
{
  "id": "domain-lib",
  "role": "dependency",
  "locator": "kpar:domain-lib:0.1.0"
}
```

The locator is a stable package coordinate. The resolver decides where to find the package:

1. Local user package repository, such as `~/.mercurio/packages`.
2. Bundled package repository shipped with Mercurio.
3. Configured remote package sources, such as OCI registries, in a later implementation.

This allows one project to stage a package locally:

```powershell
mercurio package build --file src --name domain-lib --version 0.1.0
```

And another project to consume it without hardcoding an absolute file path:

```json
{
  "version": 1,
  "name": "Vehicle Model",
  "libraries": [
    {
      "id": "domain-lib",
      "role": "dependency",
      "locator": "kpar:domain-lib:0.1.0"
    }
  ]
}
```

Supported locator forms in the first implementation:

```text
kpar:domain-lib:0.1.0
kpar:com.acme/domain-lib:0.1.0
file:libs/domain-lib-0.1.0.kpar
```

Planned later locator forms:

```text
kpar:domain-lib@sha256:abc123...
oci:ghcr.io/acme/mercurio/domain-lib:0.1.0
```

If a locator includes a digest, the resolver should verify the package before loading it.

## Standard Library Locator

The standard library should follow the same package convention as project and domain libraries. The default baseline can be modeled as a locator:

```json
{
  "id": "stdlib",
  "role": "baseline",
  "locator": "kpar:org.omg/sysml-stdlib:2.0.0"
}
```

If no `role: "baseline"` library is declared, Mercurio behaves as if the descriptor declared that standard library locator. Resolution prefers cached and bundled KPAR packages before falling back to the current bundled KIR during migration.

During migration, `bundled_stdlib` remains the compatibility provider and fallback for environments that do not yet use locator-backed standard library packages.

## Descriptor Discovery

Semantic CLI commands discover this descriptor automatically:

- `compile --file PATH` looks for `.mercurio-project.json` from `PATH` upward.
- `lint --file PATH` uses the first input path as the project anchor.
- `package build --file PATH` validates the package against the descriptor discovered from the first input path.
- Inline `--text` commands use the current working directory as the project anchor.

Passing `--stdlib PATH` skips descriptor discovery for that command and uses the provided KIR document as the semantic library context.

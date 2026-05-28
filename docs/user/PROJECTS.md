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
  "baseline_libraries": [],
  "libraries": []
}
```

Fields:

- `version`: descriptor schema version. The current version is `1`; omitted values default to `1`.
- `name`: optional display name for the project.
- `baseline_libraries`: foundational libraries used as the baseline semantic context. If this array is empty or omitted, Mercurio uses the bundled standard library.
- `libraries`: ordinary read-only dependency libraries added after the baseline context.

## Library Providers

Each entry in `baseline_libraries` or `libraries` has this shape:

```json
{
  "id": "domain-lib",
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

## Descriptor Discovery

Semantic CLI commands discover this descriptor automatically:

- `compile --file PATH` looks for `.mercurio-project.json` from `PATH` upward.
- `lint --file PATH` uses the first input path as the project anchor.
- `package build --file PATH` validates the package against the descriptor discovered from the first input path.
- Inline `--text` commands use the current working directory as the project anchor.

Passing `--stdlib PATH` skips descriptor discovery for that command and uses the provided KIR document as the semantic library context.

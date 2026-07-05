# Performance Harness

Foundation includes an explicit KIR performance harness for large-model timing and memory measurements. It is a binary, not a normal unit test, because the 100k and 1M element cases can be expensive.

Run the default size ladder:

```powershell
cargo run --release -p mercurio-core --bin kir_performance -- --output-dir target/kir-performance --keep-files
```

The default ladder is:

- 100 elements
- 1,000 elements
- 10,000 elements
- 100,000 elements
- 1,000,000 elements

For a smaller smoke run:

```powershell
cargo run -p mercurio-core --bin kir_performance -- --sizes 100,1000 --edits 10 --output-dir target/kir-performance-smoke
```

Measure warm-load cache layers:

```powershell
cargo run --release -p mercurio-core --bin cache_performance -- --sizes 1000,10000,100000 --output-dir target/cache-performance --keep-files
```

This reports cold document-to-runtime timing plus warm-load paths for text KIR, pretty runtime JSON, and `runtime.mruntime`.

The `runtime.mruntime` timing includes a phase breakdown for file read, binary payload decode, and `Runtime::from_artifact` reconstruction. This is intended to keep warm-load optimization focused on the dominant cost.

By default, runtime construction and semantic diff are skipped above 100,000 elements. Those guards keep the full ladder from exhausting memory while still measuring creation, validation, JSON persistence, JSON load, graph construction, and mutation for 1M elements. Override with `--max-runtime-size` and `--max-diff-size`.

The JSON report includes:

- synthetic KIR creation time
- persisted KIR validation time
- JSON write time
- JSON load time
- JSON file size
- graph construction time
- runtime construction time
- KIR mutation time
- semantic diff time
- current and peak resident memory samples, when supported by the OS
- changed element/relationship counts after mutation

## EMF Comparison

Foundation does not vendor EMF. To compare against EMF, pass an external benchmark command:

```powershell
cargo run --release -p mercurio-core --bin kir_performance -- `
  --output-dir target/kir-performance `
  --emf-command "java -jar path\to\emf-benchmark.jar"
```

The harness sets `MERCURIO_PERF_OUTPUT_DIR` for the external command and captures stdout, stderr, exit code, and elapsed time. The EMF benchmark should use the same size ladder and mutation count if the results are meant to be compared directly.

## Notes

Use release builds for real measurements. Debug builds are useful only for smoke checks.

The synthetic KIR model uses one package and `N` type elements linked by specialization. This intentionally stresses common KIR operations and graph/runtime relationship handling without depending on a source language parser.

JSON KIR remains the canonical human-readable interchange/debug format.

Workspace compile cache targets runtime state directly instead of storing additional intermediate model formats.

Workspace compile artifacts now store `document.kir.json` and `runtime.mruntime`. On a cache hit, the workspace cache loads canonical text KIR first, then loads `runtime.mruntime` when its manifest matches the compile inputs. If the runtime cache is stale, missing, or malformed, the compile artifact is rejected and rebuilt from source.

Clients can opt out of creating runtime cache files when opening or compiling a workspace. `ReadOnly` will use an existing cache if present, but will not write a new `runtime.mruntime`; `Disabled` skips runtime cache reads and writes entirely.

```rust
use mercurio_core::PersistentWorkspaceCache;

let cache = PersistentWorkspaceCache::for_workspace_root(workspace_root)
    .without_runtime_cache_writes();
```

## Workspace Cache Layers

The cache keeps text KIR as the canonical document and stores runtime-focused warm-load artifacts so a workspace can avoid repeated runtime construction when compile inputs are unchanged.

Implemented artifact set:

```text
document.kir.json
runtime.mruntime
runtime.mruntime.manifest.json
manifest.json
```

The runtime cache manifest includes:

- cache format version
- KIR schema version
- compile artifact key digest
- source input digest
- artifact digest
- generator version
- element, edge, and derived-index counts where applicable

Load order:

```text
if valid text KIR:
    load KirDocument
else:
    rebuild from source

if valid runtime.mruntime:
    load RuntimeArtifact
else:
    rebuild from source
```

`runtime.mruntime` uses a versioned binary envelope with compact numeric records. The payload starts with a shared string table, then stores element ids, kinds, property keys, edge relations, and derived-index values as numeric references. Property values still use structured JSON bytes inside the record stream because KIR property values remain open-ended.

The compact payload is split into a hot runtime section and a diagnostic section. Normal warm loads decode only the hot section: element records, property records, edge records, and derived-index records. Datalog explanation records are stored separately as diagnostic data so ordinary workspace open does not pay their decode cost. Public APIs still expose strings at the boundary.

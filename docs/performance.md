# Performance Harness

Foundation includes an explicit KIR performance harness for large-model timing and memory measurements. It is a binary, not a normal unit test, because the 100k and 1M element cases can be expensive.

Run the default size ladder:

```powershell
cargo run --release -p mercurio-foundation --bin kir_performance -- --output-dir target/kir-performance --keep-files
```

The default ladder is:

- 100 elements
- 1,000 elements
- 10,000 elements
- 100,000 elements
- 1,000,000 elements

For a smaller smoke run:

```powershell
cargo run -p mercurio-foundation --bin kir_performance -- --sizes 100,1000 --edits 10 --output-dir target/kir-performance-smoke
```

Measure warm-load cache layers:

```powershell
cargo run --release -p mercurio-foundation --bin cache_performance -- --sizes 1000,10000,100000 --output-dir target/cache-performance --keep-files
```

This reports cold document-to-runtime timing plus warm-load paths for text KIR, graph cache, pretty runtime JSON, and `runtime.mruntime`.

By default, runtime construction and semantic diff are skipped above 100,000 elements. Those guards keep the full ladder from exhausting memory while still measuring creation, validation, JSON persistence, binary KIR persistence, JSON load, binary KIR load, graph construction, and mutation for 1M elements. Override with `--max-runtime-size` and `--max-diff-size`.

The JSON report includes:

- synthetic KIR creation time
- persisted KIR validation time
- JSON write time
- binary KIR write time
- JSON load time
- binary KIR load time
- JSON and binary KIR file sizes
- graph construction time
- runtime construction time
- KIR mutation time
- semantic diff time
- current and peak resident memory samples, when supported by the OS
- changed element/relationship counts after mutation

## EMF Comparison

Foundation does not vendor EMF. To compare against EMF, pass an external benchmark command:

```powershell
cargo run --release -p mercurio-foundation --bin kir_performance -- `
  --output-dir target/kir-performance `
  --emf-command "java -jar path\to\emf-benchmark.jar"
```

The harness sets `MERCURIO_PERF_OUTPUT_DIR` for the external command and captures stdout, stderr, exit code, and elapsed time. The EMF benchmark should use the same size ladder and mutation count if the results are meant to be compared directly.

## Notes

Use release builds for real measurements. Debug builds are useful only for smoke checks.

The synthetic KIR model uses one package and `N` type elements linked by specialization. This intentionally stresses common KIR operations and graph/runtime relationship handling without depending on a source language parser.

Binary KIR is available as a compact KIR persistence format. JSON remains the human-readable interchange/debug format. The first binary format stores a versioned header, a string table, element records, metadata, and structured property values.

Workspace compile cache no longer writes `.mkir` by default. Because text KIR remains the canonical portable artifact, a workspace `.mkir` was additive storage and still required runtime construction. The warm-load cache now targets runtime state directly.

Workspace compile artifacts now store `document.kir.json`, `runtime.mruntime`, and `graph.mgraph`. On a cache hit, the workspace cache loads canonical text KIR first, then loads `runtime.mruntime` when its manifest matches the compile inputs. If the runtime cache is stale, missing, or malformed, the loader can rebuild it from `graph.mgraph` when the graph manifest matches the compile inputs. If all usable runtime cache layers fail, the compile artifact is rejected and rebuilt from source.

## Workspace Cache Layers

The cache keeps text KIR as the canonical document and stores runtime-focused warm-load artifacts so a workspace can avoid repeated runtime construction when compile inputs are unchanged.

Implemented artifact set:

```text
document.kir.json
runtime.mruntime
runtime.mruntime.manifest.json
graph.mgraph
graph.mgraph.manifest.json
manifest.json
```

The runtime and graph cache manifests include:

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
else if valid graph.mgraph:
    load GraphArtifact -> materialize runtime indexes
else:
    rebuild from source
```

The first `runtime.mruntime` and `graph.mgraph` implementations are intentionally conservative: they use versioned binary envelopes around serialized artifacts. This establishes cache validation and load-order behavior. The next optimization is replacing those payloads with compact numeric records.

Compact graph/runtime binaries should use numeric ids internally: element id table, kind table, property key table, edge relation table, element records, property records, derived-index records, and edge records. Public APIs can still expose strings at the boundary.

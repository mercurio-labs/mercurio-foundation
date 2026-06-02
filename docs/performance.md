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

Binary KIR is intended as a warm-load cache format. JSON remains the human-readable interchange/debug format. The first binary format stores a versioned header, a string table, element records, metadata, and structured property values. Future cache layers can build on this by storing graph or runtime artifacts directly.

Binary cache entries should be paired with a manifest. The manifest records the binary format version, KIR schema version, generator version, source digest, and binary digest. A loader should use the binary cache only when the manifest matches the current source bytes and binary bytes; otherwise it should rebuild from the canonical source/text KIR and write a fresh binary cache.

Workspace compile artifacts now store both `document.kir.json` and `document.mkir`. On a cache hit, the workspace cache loads `document.mkir` first when its manifest matches the compile inputs. If the binary cache is stale, missing, or malformed, the loader falls back to `document.kir.json`. If both are unusable, the compile artifact is rejected and rebuilt from source.

## Phase 2: Graph Cache

Binary KIR still reconstructs a full `KirDocument` and then builds a `Graph`. The next warm-load cache layer should store a graph artifact so a workspace can skip document reconstruction and graph construction when compile inputs are unchanged.

Proposed artifact set:

```text
runtime-artifact.json
document.kir.json
document.mkir
document.mkir.manifest.json
graph.mgraph
graph.mgraph.manifest.json
manifest.json
```

The graph cache manifest should include:

- graph binary format version
- KIR schema version
- compile artifact key digest
- binary KIR digest or source input digest
- graph generator version
- element count and edge count

Load order:

```text
if valid runtime artifact:
    load RuntimeArtifact
else if valid binary graph:
    load GraphArtifact -> materialize runtime indexes
else if valid binary KIR:
    load KirDocument -> build Graph -> materialize runtime indexes
else if valid text KIR:
    load KirDocument -> build Graph -> materialize runtime indexes
else:
    rebuild from source
```

The graph binary should use numeric ids internally: element id table, kind table, property key table, edge relation table, element records, property records, and edge records. Public APIs can still expose strings at the boundary.

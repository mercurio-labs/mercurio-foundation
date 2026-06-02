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

By default, runtime construction and semantic diff are skipped above 100,000 elements. Those guards keep the full ladder from exhausting memory while still measuring creation, validation, JSON persistence, JSON load, graph construction, and mutation for 1M elements. Override with `--max-runtime-size` and `--max-diff-size`.

The JSON report includes:

- synthetic KIR creation time
- persisted KIR validation time
- JSON write time
- JSON load time
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

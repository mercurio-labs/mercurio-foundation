# Mercurio

Mercurio Foundation is the reusable Rust library workspace for Mercurio's KIR JSON model representation, semantic graph/runtime APIs, and host-facing contracts.

The goal of this repository is to make the modeling substrate useful on its own: load and validate KIR, build semantic graphs, run deterministic runtime/query operations, and expose reusable library APIs that source-language repos, private products, and external tools can build on.

## Objectives

- Provide reusable Rust libraries for KIR, graph/runtime/query behavior, and host contracts.
- Keep the foundation model semantics independent from any particular source language, server, desktop app, or hosted product.
- Support host applications that register source-language services such as SysML and KerML.
- Use KIR as the stable semantic interchange format for graph queries, derived values, package loading, and downstream applications.
- Optimize for high-performance model loading, compilation, and runtime use.
- Keep maintainer-only diagnostics, benchmarks, and Pilot comparison workflows separate from the public CLI.

## What Lives Here

- `mercurio-foundation` loads libraries, builds runtime graphs, and computes deterministic derived values.
- `mercurio-kir` owns the KIR document model, validation, and artifact IO.
- `mercurio-language-contracts` owns host-facing language service contracts.
- `mercurio-tools` contains maintainer tools for diagnostics, benchmarks, demos, and transitional comparison workflows.
- `resources/` contains bundled runtime and standard library artifacts.
- `examples/` and `fixtures/` provide SysML, KerML, and KIR models for tests and demonstrations.

SysML and KerML language libraries live in the sibling `mercurio-sysml` repository. The command-line host lives in the sibling `mercurio-cli` repository. Reasoning APIs, plugin contracts, deterministic reference capabilities, and AI orchestration live in the sibling `mercurio-reasoning` repository. The hosted product, UI, and privileged console API live in the private `mercurio-product` repository.

## Core Concepts

- Source languages: host applications register language services. SysML and KerML are provided by the sibling `mercurio-sysml` repository.
- KIR: Mercurio's validated semantic JSON format, used by graph queries, derived values, projections, package loading, and product hosts.
- Standard library: semantic compilation and linting use the bundled default standard library unless a command is given `--stdlib PATH`.
- Project descriptors: `.mercurio-project.json` files describe baseline and dependency libraries in a single `libraries` array.
- KPAR packages: source-backed zip packages containing SysML/KerML sources plus package metadata.

## Requirements

- Rust toolchain with Cargo
- Java, only for Pilot comparison/export tools under `../mercurio-sysml/tools/pilot-exporter`

Most commands assume you are running them from the repository root.

## Quick Start

Build the workspace:

```powershell
cargo build
```

Run the test suite:

```powershell
cargo test
```

Show the public CLI:

```powershell
mercurio --help
```

Parse an inline SysML model:

```powershell
mercurio parse --text "package Demo { part def Vehicle; }"
```

Create a project scaffold:

```powershell
mercurio project new my-model --name "My Model"
```

## User Documentation

- [CLI Guide](docs/user/CLI.md): public `mercurio` command examples for project, parse, compile, query, evaluate, lint, package, completions, and common input forms.
- [Project Descriptors](docs/user/PROJECTS.md): `.mercurio-project.json`, provider kinds, and descriptor discovery.
- [KIR User Guide](docs/user/KIR.md): compiled semantic JSON, ids, provenance, validation, and low-level workflows.
- [Querying And Evaluation](docs/user/QUERY_EVALUATE.md): model queries, derived values, runtime context, and explanations.
- [KPAR Packages](docs/user/KPAR.md): building and consuming `.kpar` model packages.
- [Troubleshooting](docs/user/TROUBLESHOOTING.md): common command, descriptor, stdlib, KPAR, and Pilot-tool issues.

## Developer Documentation

- [Development Docs](docs/development/README.md): architecture notes, implementation plans, roadmap, runtime design, server plans, and semantic-service references.
- [Maintainer Tools](docs/development/MAINTAINER_TOOLS.md): diagnostics, benchmarks, demos, and Pilot comparison/export workflows.

## Repository Layout

- `Cargo.toml` - workspace manifest
- `crates/mercurio-foundation/` - foundation library crate
- `crates/mercurio-tools/` - maintainer diagnostics, benchmarks, demos, and Pilot comparison tools
- `crates/mercurio-kir/` - KIR document and validation crate
- `crates/mercurio-language-contracts/` - host-facing language service contracts
- `examples/` - KIR JSON models and SysML/KerML example corpora
- `fixtures/` - test fixtures
- `resources/` - bundled runtime and library resources
- `docs/` - user docs plus development architecture and implementation notes
- `../mercurio-sysml/tools/pilot-exporter/` - Java helper used by Pilot comparison workflows

## Performance

Mercurio keeps the default standard library precompiled as KIR, uses bounded semantic caches for warm project workflows, and tracks load speed and memory use in benchmark runs.

The current benchmark snapshot is in [Compile Performance Benchmark](docs/development/COMPILE_PERFORMANCE_BENCHMARK.md).

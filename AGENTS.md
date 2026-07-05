# mercurio-foundation — Agent Orientation

Source-language-neutral KIR substrate, graph, runtime, and simulation core. This is the **innermost workspace** — all other Mercurio workspaces depend on it; it depends on nothing Mercurio-specific.

---

## Crates

| Crate | Responsibility |
|-------|---------------|
| `mercurio-kir` | KIR data contract: `KirDocument`, `KirElement`, validation, merge, IO. Small and stable — keep it small. |
| `mercurio-language-contracts` | Language-service contracts, diagnostics, `LanguageService` / `LanguageRegistry` traits, expression IR |
| `mercurio-model` | Source-language-neutral model structures, graph projection, metamodel views |
| `mercurio-runtime` | **Deterministic** runtime: index construction, semantic queries, expression evaluation — no I/O |
| `mercurio-core` | Public compatibility facade: reexports the focused foundation crates under the `mercurio_core` Rust library target |
| `mercurio-simulation-core` | Simulation execution primitives shared by all execution back-ends |
| `mercurio-views` | UI-oriented view DTOs and rendering helpers |

Key file locations:

```
crates/mercurio-kir/src/           — KIR schema and validation
crates/mercurio-model/src/graph.rs — graph projection and traversal API
crates/mercurio-runtime/src/       — runtime construction and semantic queries
crates/mercurio-language-contracts/src/ — diagnostics, LanguageService trait
```

---

## Forbidden Dependencies

These crates must **never** import from:

```
mercurio-adapter    mercurio-ai           mercurio-console-api
mercurio-plugin-api mercurio-product      mercurio-python
mercurio-reasoner-api mercurio-reasoner-host
mercurio-reasoning-services mercurio-views mercurio-wasm
```

After any `Cargo.toml` change, run:

```powershell
cargo run --manifest-path ..\mercurio-sysml\Cargo.toml -p mercurio-tools --bin check_repo_boundaries -- --manifest repo-boundaries.json
cargo run --manifest-path ..\mercurio-sysml\Cargo.toml -p mercurio-tools --bin check_repo_boundaries -- --manifest repo-boundaries.json --strict
```

The machine-readable constraints live in [`repo-boundaries.json`](repo-boundaries.json).

---

## WASM Portability

Foundation crates must compile to `wasm32-unknown-unknown` without changes. Avoid `std::fs`, `std::thread`, system time, or OS-specific APIs. Abstract I/O behind trait boundaries.

---

## Build & Test

```powershell
cargo build
cargo test
cargo test --no-run      # compile-only smoke check
```

---

## Key Constraints

- `mercurio-runtime` must remain deterministic — no randomness, wall-clock reads, or I/O in core evaluation paths.
- `mercurio-kir` is a **stable data contract** — adding `Option` fields is safe; removing or renaming fields breaks all consumers.
- KIR `kind` values must correspond to KerML/SysML v2 metaclass names — never invent proprietary kinds.
- Do not add concrete language parsers or version-specific metamodel bundles here; those belong in `mercurio-sysml`.

---

## Further Reading

- [docs/crates.md](docs/crates.md) — detailed crate responsibilities
- [docs/kir.md](docs/kir.md) — KIR format and schema
- [docs/philosophy.md](docs/philosophy.md) — design philosophy and boundary rationale
- [docs/language-services.md](docs/language-services.md) — `LanguageService` contract
- [repo-boundaries.json](repo-boundaries.json) — machine-readable dependency constraints

# Releasing Mercurio Foundation

Mercurio Foundation is released as one product with `mercurio-foundation` as
its public crates.io entry point. Its implementation crates are published
because Cargo must be able to resolve every transitive dependency from the
registry, but they are not separate product release trains.

All publishable workspace packages use the version in `[workspace.package]`.
Internal dependencies must retain both a local `path` and the matching
registry `version`.

## Qualification

From this repository:

```powershell
cargo test --workspace --locked
cargo doc --workspace --no-deps --locked
cargo package --workspace --no-verify --locked
```

From the sibling `mercurio-sysml` repository:

```powershell
cargo run -p mercurio-tools --bin check_repo_boundaries -- --manifest ..\mercurio-foundation\repo-boundaries.json --strict
```

`cargo package --no-verify` validates package assembly before the internal
versions exist in crates.io. The release workflow performs the registry-backed
verification while publishing each package.

## First Release

Crate names are allocated by the first successful publish. Before pushing the
first tag:

1. Create a crates.io API token with permission to publish new crates.
2. Add it to the `mercurio-foundation` GitHub repository as the
   `CARGO_REGISTRY_TOKEN` Actions secret.
3. Protect the optional `crates-io` GitHub environment if release approval is
   desired.
4. Merge the qualified release commit to `main`.
5. Create and push `foundation-v<version>`, for example
   `foundation-v0.85.0`.

The workflow is resumable. It skips any package version already present on
crates.io and retries dependents while the registry index catches up.

After the first release, configure crates.io Trusted Publishing for this GitHub
repository and replace the long-lived token step with the crates.io OIDC action.

## Publish Order

The workflow publishes:

1. `mercurio-kir`
2. `mercurio-simulation-core`
3. `mercurio-language-contracts`
4. `mercurio-model`
5. `mercurio-authoring`
6. `mercurio-runtime`
7. `mercurio-semantic-services`
8. `mercurio-views`
9. `mercurio-workspace`
10. `mercurio-analysis`
11. `mercurio-codegen`
12. `mercurio-session`
13. `mercurio-query-dsl`
14. `mercurio-core`
15. `mercurio-foundation`

Do not publish Foundation and SysML concurrently. A SysML release starts only
after its required Foundation version is visible on crates.io.

# Mercurio Core Docs

This directory separates user-facing documentation from development architecture notes and implementation plans.

UI and desktop/web product docs live in the sibling repo at `../../mercurio-ui/docs`.

## Doc Sets

- [User Docs](user/README.md): user-facing guides and operational documentation.
- [Development Docs](development/README.md): architecture notes, implementation plans, runtime design, and engineering references.
- [Core Repo Layout](CORE_REPO_LAYOUT.md): target open-core repository structure and migration boundaries.
- [Repo Boundary Audit](REPO_BOUNDARY_AUDIT.md): current crate/module classification before the physical split.
- [Peer Repository Layout](PEER_REPOSITORY_LAYOUT.md): placement rules for Pilot, examples, and sibling Mercurio repos.

## User Guides

- [CLI Guide](user/CLI.md): public `mercurio` command examples and common workflows.
- [Project Descriptors](user/PROJECTS.md)
- [KIR User Guide](user/KIR.md)
- [Querying And Evaluation](user/QUERY_EVALUATE.md)
- [KPAR Packages](user/KPAR.md)
- [Troubleshooting](user/TROUBLESHOOTING.md)

## Placement Rule

Put documentation for people using Mercurio in `user/`.
Put architecture, plans, implementation notes, benchmarks, and internal design docs in `development/`.

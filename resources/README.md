# Resources

`resources/` contains versioned inputs and compatibility artifacts used by the
current bundled stdlib path.

## `stdlib-sources/<source-id>/`

Locked stdlib source and derived compatibility artifacts for one stdlib lineage.
For `sysml-2.0-pilot-0.57.0` this includes:

- `pilot-stdlib-export.json`: raw Pilot export, the repeatable source boundary
- `source.lock.json`: source identity and export digest
- `stdlib.full.kir.json`: legacy full precompiled stdlib KIR retained for compatibility
- `stdlib.kir.json`: lightweight stdlib KIR embedded by the WASM crate
- `stdlib.rulepack.json`: generated stdlib metamodel adapter rulepack
- `sysml.library.kpar/`: bundled OMG package-set directory fallback

## `language-profiles/<profile-id>/`

Language/profile binding for a compiler profile, including:

- `profile.json`
- `provenance.json`
- `mappings/`: construct and KIR-emission mapping files for that profile

Longer term, native defaults should resolve stdlib content through bundled
KPAR/MPack packages. The unpackaged KIR/rulepack files remain here as explicit
versioned compatibility artifacts during that migration.

Language-aware loading uses split artifacts:

- `resources/kernel/kerml-kernel.kir.json`: KerML/Kernel baseline extracted from the pilot stdlib
- `resources/sysml/sysml-library.kir.json`: SysML delta artifact that excludes KerML/Kernel elements

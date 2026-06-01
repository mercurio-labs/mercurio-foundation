# SysML 2.0 Pilot 0.57.0 Mappings

These files are part of the `sysml-2.0-pilot-0.57.0` language profile.

`pilot_constructs.seed.json` maps textual parser constructs and keywords to
Pilot-derived SysML/KerML metaclasses.

`kir_emission.seed.json` maps those metaclasses to Mercurio KIR emission rules:
KIR kind, id template, emitted properties, relationships, and metadata policy.

`lowering_rules.seed.json` is the first declarative lowering rule seed. It
connects AST patterns, collection fields, elaboration notes, KIR emission
properties, and Pilot evidence citations. It is audited today and can become an
executable lowering table incrementally.

`semantic_defaults.seed.json` contains declarative semantic defaults that are
applied after syntax collection and before final KIR emission. It is the current
EMF-lite/MMP-style profile data for behavior that used to live directly in Rust:

- `reference_usage_semantics` defines modifier-triggered reference semantics,
  typed reference subset defaults, and synthetic declared-name handling.
- `definition_context` defines construct-level definition policies such as
  constructs that are always abstract.
- `usage_context` defines generic usage policies: variable ownership contexts,
  type/ownership context suppression, member-list participation, and direction
  modifier precedence.
- `usage_type_defaults`, `usage_subset_defaults`, and `usage_family_defaults`
  define default type, subset, specialization, and family behavior.
- `usage_property_defaults` defines guarded property-ref additions. Rules can
  match owner constructs and absent modifiers, then append refs to named KIR
  properties such as `type` or `definition`.

The lowering path is intentionally split into three profile-backed stages:

1. `pilot_constructs.seed.json` identifies language constructs and metaclasses.
2. `lowering_rules.seed.json` describes AST collection and emission intent.
3. `semantic_defaults.seed.json` fills semantic defaults and small elaboration
   policies that are not directly represented by syntax.

This is the current declarative lowering syntax. Future MMP input can generate
these seed files, and compiled profile packages can include them with KIR
libraries and generated wrappers.

They are compiler/profile inputs, not runtime workspace files. Stdlib release
builds include their digests in provenance and package them with the profile.

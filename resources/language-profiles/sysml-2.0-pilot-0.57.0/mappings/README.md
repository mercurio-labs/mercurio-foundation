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
applied after syntax collection and before final KIR emission. It is profile
data for behavior that used to live directly in Rust:

- `reference_usage_semantics` defines modifier-triggered reference semantics,
  typed reference subset defaults, and synthetic declared-name handling.
- `definition_context` defines construct-level definition policies such as
  constructs that are always abstract.
- `usage_context` defines generic usage policies: variable ownership contexts,
  type/ownership context suppression, member-list participation, and direction
  modifier precedence.
- `usage_type_defaults`, `usage_subset_defaults`, and `usage_family_defaults`
  define default type, subset, specialization, and family behavior.
- `usage_property_defaults` defines guarded property additions and small
  elaborations. Rules can match owner constructs, required modifiers, and absent
  modifiers. They can append refs with `property_refs`, assign string values
  with `property_values`, and override the emitted `kir_kind` for relationship
  compatibility cases such as satisfy/verify.
- `usage_actions` defines cross-element or traversal-sensitive actions that do
  not fit simple property defaults. Current actions are
  `attach_metadata_application` and `source_from_previous_sibling_state`.
- `usage_specialization_policies` names small Rust-executed specialization
  algorithms that are profile-selected and audited rather than hard-coded to a
  construct in the emission path.
- `usage_resolution_policies` names resolver ordering policies for cases where
  the same textual reference may prefer annotation targets, types, or feature
  references depending on the construct.

`usage_property_defaults.property_values` supports a deliberately small
placeholder vocabulary. Missing optional placeholders skip the property:
`$owner_id`, `$qualified_name`, `$declared_name`, `$owner_qualified_name`,
`$allocation_source`, `$allocation_target`, `$reference_target`,
`$metadata_body`, `$metadata_locale`, `$modifier_value_trigger`,
`$modifier_value_trigger_kind`, and
`$sibling_state_id_transition_target`.

`usage_actions.target` supports `$reference_target_or_owner`,
`$reference_target`, and `$owner_id`.

`usage_specialization_policies.materialized_refs_policy` currently supports
`prepend_feature_for_specialized_actions_without_multiplicity`.

`usage_resolution_policies.reference_target_policy` currently supports
`annotation_target_then_type_then_reference` and `type_then_reference`.

The lowering path is intentionally split into three profile-backed stages:

1. `pilot_constructs.seed.json` identifies language constructs and metaclasses.
2. `lowering_rules.seed.json` describes AST collection and emission intent.
3. `semantic_defaults.seed.json` fills semantic defaults and small elaboration
   policies that are not directly represented by syntax.

The lowering audit checks this bridge in both directions: construct mappings,
lowering rules, emission rules, and semantic-default construct references must
line up before profile changes are considered repeatable.

This is the current declarative lowering support. KIR remains the durable model
input for runtime and library distribution; these seed files explain how source
syntax is collected, elaborated, and emitted into that KIR.

They are compiler/profile inputs, not runtime workspace files. Stdlib release
builds include their digests in provenance and package them with the profile.

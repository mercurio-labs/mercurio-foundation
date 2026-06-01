# Pilot Lowering Evidence

Pilot lowering evidence is a checked, reviewable input for improving Mercurio's
language lowering layer. It captures facts extracted from the Pilot grammar,
Ecore metamodel, and transformed Pilot models without making live Pilot
execution part of normal builds.

The evidence is not the runtime authority. It feeds audits, mapping seeds, MMP
or registry generation, and tests. Mercurio lowering remains explicit code plus
reviewed profile data.

The Pilot-side exporter scaffold lives at:

```text
tools/pilot-exporter/src/main/java/dev/mercurio/pilot/PilotLoweringEvidenceExporter.java
```

It currently targets Ecore/EMF class and structural-feature evidence. Grammar
rule extraction can be added later from the Pilot Xtext sources or a generated
grammar report.

## Declarative Lowering Rules

The first declarative lowering rule seed lives at:

```text
resources/language-profiles/sysml-2.0-pilot-0.57.0/mappings/lowering_rules.seed.json
```

This file is not an executable lowering engine yet. It is a reviewed profile
artifact that records how a construct is expected to move through AST matching,
collection, optional elaboration, and KIR emission. Each rule can cite Pilot
grammar rules, an Ecore class, and transform observations so audits can compare
Mercurio's lowering assumptions against extracted Pilot facts.

Rules carry a `status` field. Checked-in hand-reviewed rules use `reviewed`.
Rules produced by the draft generator use `generated-draft` and should be
treated as scaffolding until reviewed and corrected.

## Artifact Shape

```json
{
  "source": {
    "pilot_source_id": "pilot-0.57.0-g5694b8a813c3",
    "exporter_version": "0.1.0",
    "captured_at_utc": "2026-06-01T00:00:00Z"
  },
  "grammar_rules": [
    {
      "grammar": "SysML",
      "rule": "PartDefinition",
      "returns": "SysML::PartDefinition",
      "keywords": ["part", "def"],
      "source_file": "SysML.xtext",
      "source_line": 937
    }
  ],
  "ecore_classes": [
    {
      "package": "SysML",
      "name": "PartDefinition",
      "supertypes": ["Definition"],
      "abstract_class": false,
      "structural_features": [
        {
          "name": "ownedFeature",
          "kind": "reference",
          "target": "Feature",
          "lower_bound": 0,
          "upper_bound": -1,
          "containment": true,
          "derived": false,
          "transient": false,
          "volatile": false
        }
      ]
    }
  ],
  "transform_observations": [
    {
      "construct": "PartUsage",
      "source_metaclass": "SysML::PartUsage",
      "produced_metaclass": "SysML::PartUsage",
      "produced_relationships": ["subsettedFeature"],
      "note": "Observed after ElementUtil.transformAll."
    }
  ]
}
```

## Use In The Lowering Plan

- Grammar evidence audits `keyword -> construct -> metaclass` mappings.
- Ecore evidence audits future MMP/registry concepts, fields, inheritance, and
  containment/reference declarations.
- Transform observations identify elaboration rules that should be explicit in
  Mercurio rather than hidden in KIR emission.
- The `audit_lowering` tool compares existing construct and emission mappings.
  It also validates the declarative lowering rule seed by default; pass
  `--no-rules` to audit only the older mapping files. With `--evidence`, it also
  compares Pilot grammar returns, Ecore classes, exact-name non-derived Ecore
  structural features, and lowering-rule Pilot source citations. Feature gaps
  are parity hints, not hard failures, because KIR may intentionally use
  normalized property names instead of Pilot Ecore feature names.
- `audit_lowering --verbose-rules` lists construct mappings and Pilot grammar
  returns that do not yet have declarative lowering rules. This is the burndown
  view for expanding the rule seed from a prototype into broader coverage.
- `audit_lowering --write-rule-draft target/lowering_rules.draft.json` writes a
  generated draft that preserves reviewed rules and fills the remaining mapped
  metaclasses from the construct and emission seeds. The generated file is a
  starting point for review, not an automatically trusted replacement for the
  checked-in rule seed.

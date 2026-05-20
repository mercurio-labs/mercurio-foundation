# Querying And Evaluation

## Query Models

Query a model with a small field/filter language:

```powershell
mercurio query --file model.sysml --query 'from elements where kind = "SysML::Systems::PartDefinition" select id, qualified_name'
mercurio query --kpar model.kpar --query 'from elements where qualified_name = "Demo.Vehicle" select id, kind' --format json
mercurio query --kpar model.kpar --query-file queries/requirements.mq
```

Use `match` patterns to bind relationships:

```powershell
mercurio query --file model.sysml --query 'match ?type kind "SysML::Systems::PartDefinition" match ?type features ?feature select ?type, ?feature'
mercurio query --file model.sysml --query 'match ?type features ?feature select ?type.qualified_name, ?feature.qualified_name'
mercurio query --file model.sysml --query 'match ?req features ?feature where ?feature.metatype = "SysML::RequireUsage" select ?req.qualified_name, ?feature.qualified_name'
```

Use `contains`, `in`, `!=`, repeated `where` clauses, and `order by` for broader classification queries:

```powershell
mercurio query --file model.sysml --query 'from elements where metatype contains "Requirement" select id, qualified_name, metatype'
mercurio query --file model.sysml --query 'from elements where metatype in ["SysML::Systems::RequirementDefinition", "SysML::RequirementUsage"] select id, qualified_name, metatype'
mercurio query --file model.sysml --query 'from elements where metatype contains "Requirement" where qualified_name != "Demo.SkipNeed" select id, qualified_name, metatype order by qualified_name'
```

## Evaluate Runtime Expressions

Evaluate a derived feature from source by compiling the model first:

```powershell
mercurio evaluate --file model.sysml --feature totalMass --owner Demo.Vehicle
```

Evaluate directly from a precompiled KIR document:

```powershell
mercurio evaluate --kir model.kir.json --feature Demo.Vehicle.totalMass --owner Demo.Vehicle
```

Evaluate directly from a KPAR package:

```powershell
mercurio evaluate --kpar model.kpar --feature totalMass --owner Demo.Vehicle
```

Evaluate an inline expression model:

```powershell
mercurio evaluate --text "package Demo { part def Vehicle { attribute mass = 40+(2); } }" --feature mass --owner Demo.Vehicle
```

## Runtime Context

Provide overlay values for runtime context:

```powershell
mercurio evaluate --kir model.kir.json --feature totalMass --owner Demo.Vehicle --value assembly.Vehicle.mass=42
```

For larger overlays, use nested JSON where the first key is owner name and the second key is feature name:

```powershell
mercurio evaluate --kir model.kir.json --feature totalMass --owner Demo.Vehicle --context-json '{ "assembly.Vehicle": { "mass": 42 } }'
```

User-facing evaluation arguments use model qualified names. Existing KIR ids such as `type.Demo.Vehicle` and `feature.Demo.Vehicle.totalMass` are still accepted for diagnostics and low-level workflows.

Add `--explain` to include runtime explanation steps in text output, or `--format json` for structured output.

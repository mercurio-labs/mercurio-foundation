# SysML Expression Support

Status: partially implemented architecture and remaining work.

## Purpose

This document describes the current expression path and the remaining work needed before `expression_ir` can be treated as a stable KIR sub-schema.

Expressions flow through the normal semantic pipeline:

```text
SysML source
  -> expression AST
  -> resolved expression
  -> KIR expression_ir
  -> runtime evaluation, constraints, simulation, verification
```

The old first-slice goal of preserving expressions from source into KIR is implemented for a useful subset. New work should focus on coverage, schema stability, diagnostics, and downstream consistency.

## Current Implementation Shape

Implementation anchors:

- `crates/mercurio-core/src/frontend/ast.rs`: expression AST nodes.
- `crates/mercurio-core/src/frontend/sysml.rs`: expression parsing.
- `crates/mercurio-core/src/frontend/resolver.rs`: expression name/path resolution.
- `crates/mercurio-core/src/frontend/transpile.rs`: `expression_ir` emission.
- `crates/mercurio-core/src/runtime.rs`: runtime `expression_ir` evaluation with legacy string fallback.
- `crates/mercurio-core/src/constraints.rs`: constraint rendering and lightweight solving from expressions.

Supported expression forms include:

- literals: integer, real, boolean, and string
- `self`
- names and feature paths
- tuple-like grouped values where parsed
- unary expressions
- binary arithmetic and boolean expressions
- function calls such as `count(...)` and `sum(...)`
- resolved feature path segments with optional semantic feature ids

The runtime evaluates `expression_ir` before legacy raw string `expression` values.

## KIR Shape

`expression_ir` is stored as structured JSON under element properties.

Representative path expression:

```json
{
  "kind": "path",
  "root": "self",
  "segments": [
    {
      "name": "mass",
      "feature": "feature.Demo.Wheel.mass"
    }
  ]
}
```

Representative binary expression:

```json
{
  "kind": "binary",
  "op": "add",
  "left": {
    "kind": "path",
    "root": "self",
    "segments": [{"name": "dryMass"}]
  },
  "right": {
    "kind": "path",
    "root": "self",
    "segments": [{"name": "fuelMass"}]
  }
}
```

This shape is useful but not yet a formal schema. The KIR spec should eventually define the complete `expression_ir` contract.

## Remaining Work

### Schema

- Define a versioned `expression_ir` JSON Schema.
- Document required fields for each expression kind.
- Decide how units, quantities, comparisons, and constraint forms are represented.
- Define compatibility rules for legacy raw string `expression`.

### Coverage

- Expand operator coverage deliberately, with tests for precedence and associativity.
- Improve function-call policy beyond the current narrow runtime subset.
- Clarify tuple semantics.
- Clarify qualified-name and initializer-reference semantics.
- Track unsupported SysML expression families explicitly rather than silently accepting partial semantics.

### Resolution

- Keep source text, source spans, and resolved semantic feature ids where possible.
- Preserve unresolved or partially resolved paths with diagnostics when the expression can still be useful for inspection.
- Make ambiguity diagnostics deterministic across local definitions, imports, aliases, and libraries.

### Runtime

- Keep expression evaluation pure: it may read graph/runtime context, but it must not mutate model or simulation state.
- Align runtime evaluator behavior with constraint solving and future simulation guard/assertion evaluation.
- Add explainability output for evaluated expressions where useful for diagnostics and verification evidence.

### Tests

- Parser tests for every supported expression family.
- Resolver tests for local, imported, alias, library, and unresolved paths.
- Transpiler snapshot tests for stable `expression_ir`.
- Runtime tests for evaluation and legacy fallback.
- Constraint tests that prove the same expression semantics are interpreted consistently.

## Non-Goals

- Full SysML action-language execution.
- Arbitrary user code execution.
- Source rewriting as part of expression evaluation.
- Treating expression parsing as separate from the normal frontend-to-KIR pipeline.

## Near-Term Recommendation

Write the `expression_ir` schema section in [KIR Specification](KIR_SPEC.md), then backfill tests around the currently supported expression kinds before expanding coverage.

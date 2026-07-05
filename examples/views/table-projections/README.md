# Table Projection Examples

These examples show table view documents where each row is a model element and each
column is a projection over that row.

- `vehicle_model.kir.json` is a small KIR model with a package, part, nested feature,
  and requirements.
- `requirements-direct.view.json` uses direct column paths such as `requirement_id`
  and `parent.name`.
- `requirements-dsl.view.json` uses the DSL-style row binding, such as
  `row.parent.name`.
- `requirements-preset.view.json` shows the requirements table as a table preset,
  not a separate model view payload.


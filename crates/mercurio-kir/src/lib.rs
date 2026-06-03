use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const KIR_SCHEMA_VERSION: &str = "0.2";
pub const KIR_SCHEMA_VERSION_METADATA_KEY: &str = "kir_schema_version";
pub const REPRESENTATIVE_KIR_JSON: &str =
    include_str!("../../../resources/foundation/representative.kir.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KirDocument {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, Value>,
    pub elements: Vec<KirElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KirElement {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub layer: u8,
    #[serde(default)]
    pub properties: BTreeMap<String, Value>,
}

#[derive(Debug)]
pub enum KirError {
    Io(std::io::Error),
    Json(serde_json::Error),
    DuplicateId(String),
    Validation(Vec<KirValidationDiagnostic>),
    Frontend(String),
    Model(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KirValidationDiagnostic {
    pub code: &'static str,
    pub message: String,
    pub element_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KirFieldKind {
    Scalar,
    Reference,
    ReferenceList,
    Expression,
    Metadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KirFieldSpec {
    pub kind: KirFieldKind,
}

#[derive(Debug, Clone, Copy)]
pub struct KirFieldRegistry;

impl KirFieldRegistry {
    pub fn standard() -> Self {
        Self
    }

    pub fn field(&self, name: &str) -> Option<KirFieldSpec> {
        let kind = match name {
            "declared_name"
            | "declared_short_name"
            | "name"
            | "qualified_name"
            | "kir_property"
            | "feature_kind"
            | "type_label"
            | "metamodel_language"
            | "metamodel_layer"
            | "pilot_library_group"
            | "lower"
            | "upper"
            | "direction"
            | "multiplicity"
            | "multiplicity_lower"
            | "multiplicity_upper"
            | "declared_multiplicity"
            | "operator"
            | "operator_expression"
            | "trigger"
            | "trigger_kind"
            | "effect"
            | "text"
            | "requirement_id"
            | "body"
            | "locale"
            | "language"
            | "source_file"
            | "source_language" => KirFieldKind::Scalar,

            "is_abstract" | "is_conjugated" | "is_derived" | "is_end" | "is_variable"
            | "is_readonly" | "is_ordered" | "is_unique" => KirFieldKind::Scalar,

            "owner"
            | "owning_type"
            | "owning_definition"
            | "owning_namespace"
            | "type"
            | "definition"
            | "metatype"
            | "featuring_type"
            | "chaining_feature"
            | "source_feature"
            | "source"
            | "target"
            | "allocated"
            | "allocated_to"
            | "parent_state"
            | "payload"
            | "result"
            | "original_definition"
            | "conjugated"
            | "opposite"
            | "documentedElement"
            | "annotatedElement"
            | "target_ref" => KirFieldKind::Reference,

            "members"
            | "features"
            | "ownedElement"
            | "documentation"
            | "owned_features"
            | "feature_typings"
            | "specializes"
            | "subsets"
            | "subsetted_features"
            | "redefines"
            | "redefined_features"
            | "specialized_features"
            | "imports"
            | "relationships"
            | "sources"
            | "targets"
            | "parts"
            | "items"
            | "owned_feature"
            | "verify"
            | "satisfy"
            | "related"
            | "parameters"
            | "arguments"
            | "successions"
            | "dependencies" => KirFieldKind::ReferenceList,

            "expression" => KirFieldKind::Scalar,
            "expression_ir" => KirFieldKind::Expression,
            "metadata" | "source_span" | "doc" => KirFieldKind::Metadata,
            "element_id" => KirFieldKind::Scalar,
            _ => return None,
        };

        Some(KirFieldSpec { kind })
    }

    pub fn reference_ids<'a>(&self, field: &str, value: &'a Value) -> Vec<&'a str> {
        match self.field(field).map(|spec| spec.kind) {
            Some(KirFieldKind::Reference) => reference_list_items(value),
            Some(KirFieldKind::ReferenceList) => reference_list_items(value),
            _ => Vec::new(),
        }
    }

    fn validate_value(
        &self,
        field: &str,
        value: &Value,
        element_id: &str,
        strict_shapes: bool,
    ) -> Option<KirValidationDiagnostic> {
        let spec = self.field(field)?;
        let valid = match spec.kind {
            KirFieldKind::Scalar => {
                value.is_string() || value.is_number() || value.is_boolean() || value.is_null()
            }
            KirFieldKind::Reference if strict_shapes => {
                value.is_string()
                    || value
                        .as_array()
                        .is_some_and(|items| items.iter().all(Value::is_string))
                    || value.is_null()
            }
            KirFieldKind::Reference => {
                value.is_string()
                    || value
                        .as_array()
                        .is_some_and(|items| items.iter().all(Value::is_string))
                    || value.is_null()
            }
            KirFieldKind::ReferenceList if strict_shapes => {
                value
                    .as_array()
                    .is_some_and(|items| items.iter().all(Value::is_string))
                    || value.is_null()
            }
            KirFieldKind::ReferenceList => {
                value.is_string()
                    || value
                        .as_array()
                        .is_some_and(|items| items.iter().all(Value::is_string))
                    || value.is_null()
            }
            KirFieldKind::Expression | KirFieldKind::Metadata => {
                value.is_object() || value.is_array() || value.is_null()
            }
        };

        (!valid).then(|| KirValidationDiagnostic {
            code: "kir.element.property.shape",
            message: format!(
                "KIR element {element_id} property `{field}` has invalid shape for {:?}",
                spec.kind
            ),
            element_id: Some(element_id.to_string()),
        })
    }

    fn unknown_field_diagnostic(
        &self,
        field: &str,
        element_id: &str,
    ) -> Option<KirValidationDiagnostic> {
        (self.field(field).is_none() && !field.starts_with("x_")).then(|| {
            KirValidationDiagnostic {
                code: "kir.element.property.unknown",
                message: format!(
                    "KIR element {element_id} property `{field}` is not registered in the field contract"
                ),
                element_id: Some(element_id.to_string()),
            }
        })
    }
}

fn reference_list_items(value: &Value) -> Vec<&str> {
    match value {
        Value::String(value) => vec![value.as_str()],
        Value::Array(items) => items.iter().filter_map(Value::as_str).collect(),
        _ => Vec::new(),
    }
}

impl fmt::Display for KirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "failed to read KIR document: {err}"),
            Self::Json(err) => write!(f, "failed to parse KIR document: {err}"),
            Self::DuplicateId(id) => write!(f, "duplicate KIR element id: {id}"),
            Self::Validation(diagnostics) => {
                write!(f, "invalid KIR document")?;
                if let Some(first) = diagnostics.first() {
                    write!(f, ": {}", first.message)?;
                }
                Ok(())
            }
            Self::Frontend(err) => write!(f, "{err}"),
            Self::Model(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for KirError {}

impl From<std::io::Error> for KirError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for KirError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl KirDocument {
    pub fn representative_example() -> Result<Self, KirError> {
        Self::from_str(REPRESENTATIVE_KIR_JSON)
    }

    pub fn from_str(input: &str) -> Result<Self, KirError> {
        let document: Self = serde_json::from_str(input)?;
        document.validate_persisted()?;
        Ok(document)
    }

    pub fn from_str_lenient(input: &str) -> Result<Self, KirError> {
        let document: Self = serde_json::from_str(input)?;
        document.validate()?;
        Ok(document.normalized_for_persistence())
    }

    pub fn from_path(path: &Path) -> Result<Self, KirError> {
        let input = std::fs::read_to_string(path)?;
        Self::from_str(&input)
    }

    pub fn from_path_lenient(path: &Path) -> Result<Self, KirError> {
        let input = std::fs::read_to_string(path)?;
        Self::from_str_lenient(&input)
    }

    pub fn validate(&self) -> Result<(), KirError> {
        self.validate_with_options(ValidationOptions {
            require_schema_version: false,
            reject_unknown_fields: false,
            strict_field_shapes: false,
        })
    }

    pub fn validate_strict_field_contract(&self) -> Result<(), KirError> {
        self.validate_with_options(ValidationOptions {
            require_schema_version: false,
            reject_unknown_fields: true,
            strict_field_shapes: true,
        })
    }

    pub fn validate_persisted(&self) -> Result<(), KirError> {
        self.validate_with_options(ValidationOptions {
            require_schema_version: true,
            reject_unknown_fields: true,
            strict_field_shapes: true,
        })
    }

    pub fn schema_version(&self) -> Option<&str> {
        self.metadata
            .get(KIR_SCHEMA_VERSION_METADATA_KEY)
            .and_then(Value::as_str)
    }

    pub fn with_schema_version(mut self) -> Self {
        self.set_schema_version();
        self
    }

    pub fn normalized_for_persistence(mut self) -> Self {
        self.set_schema_version();
        let field_registry = KirFieldRegistry::standard();
        for element in &mut self.elements {
            normalize_reference_shapes(&field_registry, element);
            if !element.properties.contains_key("qualified_name")
                && let Some(qualified_name) = qualified_name_from_element_id(&element.id)
            {
                element
                    .properties
                    .insert("qualified_name".to_string(), Value::String(qualified_name));
            }
        }
        self
    }

    pub fn set_schema_version(&mut self) {
        self.metadata.insert(
            KIR_SCHEMA_VERSION_METADATA_KEY.to_string(),
            Value::String(KIR_SCHEMA_VERSION.to_string()),
        );
    }

    fn validate_with_options(&self, options: ValidationOptions) -> Result<(), KirError> {
        let mut diagnostics = Vec::new();
        let mut seen = BTreeSet::new();
        let field_registry = KirFieldRegistry::standard();

        if options.require_schema_version {
            match self.metadata.get(KIR_SCHEMA_VERSION_METADATA_KEY) {
                Some(Value::String(version)) if version == KIR_SCHEMA_VERSION => {}
                Some(Value::String(version)) => diagnostics.push(KirValidationDiagnostic {
                    code: "kir.document.schema_version.unsupported",
                    message: format!(
                        "KIR document schema version `{version}` is not supported; expected `{KIR_SCHEMA_VERSION}`"
                    ),
                    element_id: None,
                }),
                Some(_) => diagnostics.push(KirValidationDiagnostic {
                    code: "kir.document.schema_version.invalid",
                    message: format!(
                        "KIR document metadata `{KIR_SCHEMA_VERSION_METADATA_KEY}` must be a string"
                    ),
                    element_id: None,
                }),
                None => diagnostics.push(KirValidationDiagnostic {
                    code: "kir.document.schema_version.missing",
                    message: format!(
                        "KIR document metadata must include `{KIR_SCHEMA_VERSION_METADATA_KEY}`"
                    ),
                    element_id: None,
                }),
            }
        }

        for element in &self.elements {
            let trimmed_id = element.id.trim();
            if trimmed_id.is_empty() {
                diagnostics.push(KirValidationDiagnostic {
                    code: "kir.element.id.empty",
                    message: "KIR element id must not be empty".to_string(),
                    element_id: None,
                });
            } else if trimmed_id != element.id {
                diagnostics.push(KirValidationDiagnostic {
                    code: "kir.element.id.invalid",
                    message: format!(
                        "KIR element id must not contain leading or trailing whitespace: {}",
                        element.id
                    ),
                    element_id: Some(element.id.clone()),
                });
            }

            if element.kind.trim().is_empty() {
                diagnostics.push(KirValidationDiagnostic {
                    code: "kir.element.kind.empty",
                    message: format!("KIR element {} must declare a semantic kind", element.id),
                    element_id: Some(element.id.clone()),
                });
            }

            if element.layer > 2 {
                diagnostics.push(KirValidationDiagnostic {
                    code: "kir.element.layer.unsupported",
                    message: format!(
                        "KIR element {} uses unsupported layer {}",
                        element.id, element.layer
                    ),
                    element_id: Some(element.id.clone()),
                });
            }

            if !element.id.is_empty() && !seen.insert(element.id.clone()) {
                diagnostics.push(KirValidationDiagnostic {
                    code: "kir.element.id.duplicate",
                    message: format!("duplicate KIR element id: {}", element.id),
                    element_id: Some(element.id.clone()),
                });
            }

            if options.strict_field_shapes
                && qualified_name_from_element_id(&element.id).is_some()
                && !element
                    .properties
                    .get("qualified_name")
                    .is_some_and(Value::is_string)
            {
                diagnostics.push(KirValidationDiagnostic {
                    code: "kir.element.qualified_name.missing",
                    message: format!(
                        "KIR element {} must include string property `qualified_name`",
                        element.id
                    ),
                    element_id: Some(element.id.clone()),
                });
            }

            for (property, value) in &element.properties {
                if options.reject_unknown_fields {
                    if let Some(diagnostic) =
                        field_registry.unknown_field_diagnostic(property, &element.id)
                    {
                        diagnostics.push(diagnostic);
                        continue;
                    }
                }
                if let Some(diagnostic) = field_registry.validate_value(
                    property,
                    value,
                    &element.id,
                    options.strict_field_shapes,
                ) {
                    diagnostics.push(diagnostic);
                }
                validate_structured_property(property, value, &element.id, &mut diagnostics);
            }
        }

        if diagnostics.is_empty() {
            Ok(())
        } else {
            Err(KirError::Validation(diagnostics))
        }
    }

    pub fn merge<I>(documents: I) -> Result<Self, KirError>
    where
        I: IntoIterator<Item = KirDocument>,
    {
        let mut seen = BTreeSet::new();
        let mut elements = Vec::new();
        let mut source_metadata = Vec::new();

        for document in documents {
            if !document.metadata.is_empty() {
                source_metadata.push(Value::Object(document.metadata.into_iter().collect()));
            }
            for element in document.elements {
                if !seen.insert(element.id.clone()) {
                    return Err(KirError::DuplicateId(element.id));
                }
                elements.push(element);
            }
        }

        elements.sort_by(|left, right| left.id.cmp(&right.id));
        let mut metadata = BTreeMap::from([(
            KIR_SCHEMA_VERSION_METADATA_KEY.to_string(),
            Value::String(KIR_SCHEMA_VERSION.to_string()),
        )]);
        if !source_metadata.is_empty() {
            metadata.insert("merged_sources".to_string(), Value::Array(source_metadata));
        }

        let merged = Self { metadata, elements }.normalized_for_persistence();
        merged.validate_persisted()?;
        Ok(merged)
    }

    pub fn write_pretty_to_path(&self, path: &Path) -> Result<(), KirError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(
            path,
            serde_json::to_string_pretty(&self.clone().normalized_for_persistence())?,
        )?;
        Ok(())
    }
}

fn normalize_reference_shapes(field_registry: &KirFieldRegistry, element: &mut KirElement) {
    for (property, value) in &mut element.properties {
        if !matches!(
            field_registry.field(property).map(|spec| spec.kind),
            Some(KirFieldKind::Reference)
        ) {
            continue;
        }

        let Some(items) = value.as_array() else {
            continue;
        };

        *value = match items.as_slice() {
            [] => Value::Null,
            [single] if single.is_string() => single.clone(),
            _ => continue,
        };
    }
}

#[derive(Debug, Clone, Copy)]
struct ValidationOptions {
    require_schema_version: bool,
    reject_unknown_fields: bool,
    strict_field_shapes: bool,
}

fn validate_structured_property(
    property: &str,
    value: &Value,
    element_id: &str,
    diagnostics: &mut Vec<KirValidationDiagnostic>,
) {
    match property {
        "metadata" => validate_metadata(value, element_id, diagnostics),
        "source_span" => validate_source_span(value, element_id, "source_span", diagnostics),
        "expression_ir" => validate_expression_ir(value, element_id, "expression_ir", diagnostics),
        _ => {}
    }
}

fn validate_metadata(
    value: &Value,
    element_id: &str,
    diagnostics: &mut Vec<KirValidationDiagnostic>,
) {
    let Some(metadata) = value.as_object() else {
        return;
    };

    if let Some(source_file) = metadata.get("source_file")
        && !(source_file.is_string() || source_file.is_null())
    {
        diagnostics.push(KirValidationDiagnostic {
            code: "kir.element.metadata.source_file.shape",
            message: format!("KIR element {element_id} metadata `source_file` must be a string"),
            element_id: Some(element_id.to_string()),
        });
    }

    if let Some(source_language) = metadata.get("source_language")
        && !(source_language.is_string() || source_language.is_null())
    {
        diagnostics.push(KirValidationDiagnostic {
            code: "kir.element.metadata.source_language.shape",
            message: format!(
                "KIR element {element_id} metadata `source_language` must be a string"
            ),
            element_id: Some(element_id.to_string()),
        });
    }

    if let Some(generated) = metadata.get("generated")
        && !(generated.is_boolean() || generated.is_null())
    {
        diagnostics.push(KirValidationDiagnostic {
            code: "kir.element.metadata.generated.shape",
            message: format!("KIR element {element_id} metadata `generated` must be a boolean"),
            element_id: Some(element_id.to_string()),
        });
    }

    if let Some(source_span) = metadata.get("source_span") {
        validate_source_span(source_span, element_id, "metadata.source_span", diagnostics);
    }
}

fn validate_source_span(
    value: &Value,
    element_id: &str,
    field: &'static str,
    diagnostics: &mut Vec<KirValidationDiagnostic>,
) {
    if value.is_null() {
        return;
    }
    let Some(span) = value.as_object() else {
        diagnostics.push(KirValidationDiagnostic {
            code: "kir.element.source_span.shape",
            message: format!("KIR element {element_id} `{field}` must be an object"),
            element_id: Some(element_id.to_string()),
        });
        return;
    };

    for key in ["start_line", "end_line"] {
        if !span.get(key).is_some_and(Value::is_number) {
            diagnostics.push(KirValidationDiagnostic {
                code: "kir.element.source_span.shape",
                message: format!("KIR element {element_id} `{field}.{key}` must be a number"),
                element_id: Some(element_id.to_string()),
            });
        }
    }
    for key in ["start_col", "end_col"] {
        if let Some(value) = span.get(key)
            && !value.is_number()
        {
            diagnostics.push(KirValidationDiagnostic {
                code: "kir.element.source_span.shape",
                message: format!(
                    "KIR element {element_id} `{field}.{key}` must be a number when present"
                ),
                element_id: Some(element_id.to_string()),
            });
        }
    }
}

fn validate_expression_ir(
    value: &Value,
    element_id: &str,
    field: &'static str,
    diagnostics: &mut Vec<KirValidationDiagnostic>,
) {
    if value.is_null() {
        return;
    }
    let Some(object) = value.as_object() else {
        diagnostics.push(KirValidationDiagnostic {
            code: "kir.element.expression_ir.shape",
            message: format!("KIR element {element_id} `{field}` must be an object"),
            element_id: Some(element_id.to_string()),
        });
        return;
    };
    let Some(kind) = object.get("kind").and_then(Value::as_str) else {
        diagnostics.push(KirValidationDiagnostic {
            code: "kir.element.expression_ir.kind.missing",
            message: format!("KIR element {element_id} `{field}.kind` must be a string"),
            element_id: Some(element_id.to_string()),
        });
        return;
    };

    match kind {
        "literal" => {
            if !object.contains_key("value") {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "literal requires `value`",
                ));
            }
        }
        "self" => {}
        "path" => {
            if !object
                .get("root")
                .and_then(Value::as_str)
                .is_some_and(|root| root == "self")
            {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "path requires root `self`",
                ));
            }
            if !object.get("segments").is_some_and(Value::is_array) {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "path requires array `segments`",
                ));
            }
        }
        "tuple" => {
            if !object.get("items").is_some_and(Value::is_array) {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "tuple requires array `items`",
                ));
            }
        }
        "unary" => {
            if !object.get("op").is_some_and(Value::is_string)
                || !object.get("expr").is_some_and(Value::is_object)
            {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "unary requires `op` and object `expr`",
                ));
            }
        }
        "binary" => {
            if !object.get("op").is_some_and(Value::is_string)
                || !object.get("left").is_some_and(Value::is_object)
                || !object.get("right").is_some_and(Value::is_object)
            {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "binary requires `op`, `left`, and `right`",
                ));
            }
        }
        "call" => {
            if !object.get("function").is_some_and(Value::is_string)
                || !object.get("args").is_some_and(Value::is_array)
            {
                diagnostics.push(expression_ir_diagnostic(
                    element_id,
                    field,
                    "call requires `function` and array `args`",
                ));
            }
        }
        _ => diagnostics.push(expression_ir_diagnostic(
            element_id,
            field,
            "unknown expression kind",
        )),
    }
}

fn expression_ir_diagnostic(
    element_id: &str,
    field: &'static str,
    message: &'static str,
) -> KirValidationDiagnostic {
    KirValidationDiagnostic {
        code: "kir.element.expression_ir.shape",
        message: format!("KIR element {element_id} `{field}` is invalid: {message}"),
        element_id: Some(element_id.to_string()),
    }
}

fn qualified_name_from_element_id(id: &str) -> Option<String> {
    const PREFIXES: [&str; 14] = [
        "pkg.",
        "type.",
        "feature.",
        "part.",
        "item.",
        "requirement.",
        "case.",
        "action.",
        "state.",
        "transition.",
        "connection.",
        "metadata.",
        "relationship.",
        "doc.",
    ];

    PREFIXES
        .iter()
        .find_map(|prefix| id.strip_prefix(prefix))
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::{KirDocument, KirElement, KirError};

    #[test]
    fn merge_rejects_duplicate_ids() {
        let left = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "dup".to_string(),
                kind: "core.Type".to_string(),
                layer: 0,
                properties: Default::default(),
            }],
        };
        let right = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "dup".to_string(),
                kind: "model.PartDefinition".to_string(),
                layer: 1,
                properties: Default::default(),
            }],
        };

        let error = KirDocument::merge([left, right]).unwrap_err();
        assert!(matches!(error, KirError::DuplicateId(id) if id == "dup"));
    }

    #[test]
    fn validate_rejects_empty_element_id() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: String::new(),
                kind: "Core::Core::Type".to_string(),
                layer: 0,
                properties: Default::default(),
            }],
        };

        let error = document.validate().unwrap_err();
        assert!(
            matches!(error, KirError::Validation(diagnostics) if diagnostics[0].code == "kir.element.id.empty")
        );
    }

    #[test]
    fn validate_rejects_unsupported_layer() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "type.Bad.Layer".to_string(),
                kind: "Core::Core::Type".to_string(),
                layer: 3,
                properties: Default::default(),
            }],
        };

        let error = document.validate().unwrap_err();
        assert!(
            matches!(error, KirError::Validation(diagnostics) if diagnostics[0].code == "kir.element.layer.unsupported")
        );
    }

    #[test]
    fn validate_rejects_registered_reference_field_with_wrong_shape() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "feature.Demo.Vehicle.engine".to_string(),
                kind: "Model::PartUsage".to_string(),
                layer: 2,
                properties: [("owner".to_string(), Value::Bool(true))]
                    .into_iter()
                    .collect(),
            }],
        };

        let error = document.validate().unwrap_err();
        assert!(
            matches!(error, KirError::Validation(diagnostics) if diagnostics[0].code == "kir.element.property.shape")
        );
    }

    #[test]
    fn validate_accepts_registered_reference_list_as_scalar_or_array() {
        let scalar = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "type.Demo.Vehicle".to_string(),
                kind: "Model::Systems::PartDefinition".to_string(),
                layer: 2,
                properties: [(
                    "specializes".to_string(),
                    Value::String("Parts::Part".to_string()),
                )]
                .into_iter()
                .collect(),
            }],
        };
        let array = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "type.Demo.Vehicle".to_string(),
                kind: "Model::Systems::PartDefinition".to_string(),
                layer: 2,
                properties: [(
                    "specializes".to_string(),
                    Value::Array(vec![Value::String("Parts::Part".to_string())]),
                )]
                .into_iter()
                .collect(),
            }],
        };

        scalar.validate().unwrap();
        array.validate().unwrap();
    }

    #[test]
    fn strict_field_contract_rejects_unknown_non_extension_property() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "type.Demo.Vehicle".to_string(),
                kind: "Model::Systems::PartDefinition".to_string(),
                layer: 2,
                properties: [
                    (
                        "qualified_name".to_string(),
                        Value::String("Demo.Vehicle".to_string()),
                    ),
                    ("custom".to_string(), Value::String("value".to_string())),
                    (
                        "x_vendor".to_string(),
                        Value::String("preserved".to_string()),
                    ),
                ]
                .into_iter()
                .collect(),
            }],
        };

        document.validate().unwrap();
        let error = document.validate_strict_field_contract().unwrap_err();
        assert!(
            matches!(error, KirError::Validation(diagnostics) if diagnostics[0].code == "kir.element.property.unknown")
        );
    }

    #[test]
    fn persisted_validation_requires_schema_version() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "type.Demo.Vehicle".to_string(),
                kind: "Model::Systems::PartDefinition".to_string(),
                layer: 2,
                properties: Default::default(),
            }],
        };

        document.validate().unwrap();
        let error = document.validate_persisted().unwrap_err();
        assert!(
            matches!(error, KirError::Validation(diagnostics) if diagnostics[0].code == "kir.document.schema_version.missing")
        );
    }

    #[test]
    fn with_schema_version_normalizes_persisted_envelope() {
        let document = KirDocument {
            metadata: Default::default(),
            elements: vec![KirElement {
                id: "type.Demo.Vehicle".to_string(),
                kind: "Model::Systems::PartDefinition".to_string(),
                layer: 2,
                properties: Default::default(),
            }],
        }
        .normalized_for_persistence();

        assert_eq!(document.schema_version(), Some(super::KIR_SCHEMA_VERSION));
        document.validate_persisted().unwrap();
    }

    #[test]
    fn representative_example_is_valid_persisted_kir() {
        let document = KirDocument::representative_example().unwrap();

        assert_eq!(document.schema_version(), Some(super::KIR_SCHEMA_VERSION));
        assert!(
            document
                .elements
                .iter()
                .any(|element| element.id == "pkg.Example")
        );
        assert!(
            document
                .elements
                .iter()
                .any(|element| element.id == "activity.Example.Startup")
        );
        document.validate_persisted().unwrap();
    }

    #[test]
    fn from_str_rejects_unknown_fields_in_persisted_kir() {
        let input = format!(
            r#"{{
  "metadata": {{ "kir_schema_version": "{}" }},
  "elements": [
    {{
      "id": "type.Demo.Vehicle",
      "kind": "Model::Systems::PartDefinition",
      "layer": 2,
      "properties": {{ "qualified_name": "Demo.Vehicle", "custom": "value" }}
    }}
  ]
}}"#,
            super::KIR_SCHEMA_VERSION
        );

        let error = KirDocument::from_str(&input).unwrap_err();
        assert!(
            matches!(error, KirError::Validation(diagnostics) if diagnostics[0].code == "kir.element.property.unknown")
        );
    }

    #[test]
    fn merge_preserves_document_metadata_as_sources() {
        let left = KirDocument {
            metadata: [("source".to_string(), Value::String("stdlib".to_string()))]
                .into_iter()
                .collect(),
            elements: vec![],
        };
        let right = KirDocument {
            metadata: [("source".to_string(), Value::String("user".to_string()))]
                .into_iter()
                .collect(),
            elements: vec![],
        };

        let merged = KirDocument::merge([left, right]).unwrap();
        assert_eq!(
            merged.metadata[super::KIR_SCHEMA_VERSION_METADATA_KEY],
            Value::String(super::KIR_SCHEMA_VERSION.to_string())
        );
        assert_eq!(
            merged.metadata["merged_sources"].as_array().unwrap().len(),
            2
        );
    }
}

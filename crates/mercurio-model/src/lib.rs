//! Model-layer data structures and semantic helpers.
//!
//! Prefer the root-level re-exports as the supported API. The module tree is
//! public for compatibility with existing callers, but hidden from rustdoc so
//! generated documentation focuses on the intentional contract.

#[doc(hidden)]
mod derived;
#[doc(hidden)]
mod expression;
#[doc(hidden)]
mod graph;
#[doc(hidden)]
mod ir;
#[doc(hidden)]
mod metadata;
#[doc(hidden)]
mod metamodel;

pub use derived::{
    DerivedFeatureCache, DerivedFeatureManifest, DerivedFeatureManifestError,
    DerivedFeatureRegistry, DerivedFeatureRule, DerivedFeatureSpec, DerivedPropertySource,
    DerivedPropertyValue, builtin_core_derived_feature_manifest, derived_properties,
    derived_property, manifest_from_metadata,
};
pub use expression::{
    BinaryExpressionOp, ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
    ExpressionIrError, ExpressionPathRoot, ExpressionPathSegment, ExpressionValidationError,
    UnaryExpressionOp,
};
pub use graph::{Edge, Element, ElementProperties, Graph, GraphArtifact, GraphError, NodeId};
pub use ir::{
    Diagnostic, DiagnosticKind, KIR_SCHEMA_VERSION, KirDocument, KirElement, KirError,
    KirFieldKind, KirFieldRegistry, KirFieldSpec, REPRESENTATIVE_KIR_JSON,
};
pub use metadata::{
    ElementMetadataView, KirMetadataAnnotation, MetadataView, metadata_annotations,
    metadata_annotations_named, metadata_string_property,
};
pub use metamodel::{
    AttributeRow, AttributeValueSource, DerivedMetamodelCapabilities, ElementAttributeQuery,
    ElementSummary, MetamodelAttributeDeclaration, MetamodelAttributeRegistry, MetamodelClassView,
    MetamodelFeatureRegistry, MetamodelFeatureView, MetamodelValidationDiagnostic,
    MetatypeQueryOverride, collect_specialization_ancestors, derive_metamodel_capabilities,
    effective_element_properties_with_derived, effective_properties,
    effective_properties_with_derived, element_metatype, query_element_attributes,
    validate_derived_metamodel_semantics,
};

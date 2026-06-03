pub mod derived;
pub mod expression;
pub mod graph;
pub mod ir;
pub mod metadata;
pub mod metamodel;

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
    KIR_SCHEMA_VERSION, KirDocument, KirElement, KirError, KirFieldKind, KirFieldRegistry,
    KirFieldSpec, KirValidationDiagnostic, REPRESENTATIVE_KIR_JSON,
};
pub use metadata::{
    ElementMetadataView, KirMetadataAnnotation, MetadataView, metadata_annotations,
    metadata_annotations_named, metadata_string_property,
};
pub use metamodel::{
    AttributeRow, AttributeValueSource, ElementAttributeQuery, ElementSummary,
    MetamodelAttributeRegistry, MetamodelClassView, MetamodelFeatureRegistry, MetamodelFeatureView,
    MetatypeQueryOverride, collect_specialization_ancestors, effective_properties,
    effective_properties_with_derived, element_metatype, query_element_attributes,
};

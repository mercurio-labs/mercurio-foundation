pub mod assessment;
pub mod authoring;
pub mod capability;
pub mod datalog {
    pub use mercurio_runtime::datalog::*;
}
pub mod derived {
    pub use mercurio_model::derived::*;
}
pub mod element_view;
pub mod expression {
    pub use mercurio_model::expression::*;
}
pub mod feasibility;
pub mod frontend;
#[allow(deprecated)]
pub mod goal;
pub mod graph {
    pub use mercurio_model::graph::*;
}
pub mod identity;
pub mod ir;
pub mod language;
pub mod library;
pub mod logging;
pub mod metadata {
    pub use mercurio_model::metadata::*;
}
pub mod metamodel {
    pub use mercurio_model::metamodel::*;
}
pub mod mpack;
pub mod mutation;
pub mod outline;
pub mod paths;
pub mod performance;
pub mod plugin_registry;
pub mod proposal;
pub mod python_codegen;
#[allow(deprecated)]
pub mod query;
pub mod runtime {
    pub use mercurio_runtime::runtime::*;
}
pub mod semantic_compare;
pub mod semantic_profile;
pub mod semantic_target;
pub mod session;
pub mod source_set;
pub mod syntax_compare;
#[cfg(test)]
pub(crate) mod test_support;
#[allow(deprecated)]
pub mod views;
pub mod workspace;
pub mod workspace_cache;

pub use assessment::{
    AssessmentAssertion, AssessmentAssertionReport, AssessmentError, AssessmentExpectation,
    AssessmentQuery, AssessmentReport, AssessmentSpec, AssessmentStatus, RuntimeAssessmentRequest,
    RuntimeAssessmentResult, query_evaluation, run_evaluation_assessment, run_graph_assessment,
    run_runtime_assessment,
};
pub use authoring::{
    Alias, AttributeWritePolicy, AuthoringError, AuthoringModule, AuthoringProject,
    AuthoringRenderProfile, ContainerSelector, Declaration, Definition, Import, Mutation,
    MutationResult, Package, QualifiedName, RenderedSpan, SemanticAttribute, SemanticEdit, Usage,
    ValidationReport, WriteBackMode, WriteBackResult, create_empty_model,
    load_authoring_project_from_kir, textual_model_authoring_render_profile,
};
pub use capability::{
    CapabilityCostClass, CapabilityDescriptor, CapabilityError, CapabilityKind, CapabilityMaturity,
    CapabilityReadinessReport, CapabilityReadinessStatus, CapabilityRegistry, CapabilityRunReport,
    CapabilityRunRequest, CapabilityRunStatus, CapabilityTarget, DecisionAssessment,
    DecisionContext, EvidenceEdge, EvidenceGraph, EvidenceNode, EvidenceNodeKind, EvidenceRelation,
    GenericImpactCapability, InsightConfidence, InsightKind, InsightPolarity, InsightScope,
    InsightSeverity, SemanticArtifact, SemanticCapability, SemanticDiagnostic,
    SemanticDiagnosticSeverity, SemanticElementRef, SemanticInsight, SemanticWorkspaceSnapshot,
    assess_decision_context,
};
pub use datalog::{
    Atom, CORE_RULEPACK_ID, CORE_RULEPACK_VERSION, DatalogError, DerivedIndexes, Evaluation,
    Explanation, Fact, Rule, RulePack, Term, evaluate, extract_graph_facts, load_default_rulepacks,
    materialize_core_indexes,
};
pub use derived::{
    DerivedFeatureCache, DerivedFeatureManifest, DerivedFeatureManifestError,
    DerivedFeatureRegistry, DerivedFeatureRule, DerivedFeatureSpec, DerivedPropertySource,
    DerivedPropertyValue, builtin_core_derived_feature_manifest, derived_properties,
    derived_property, manifest_from_metadata,
};
pub use element_view::ElementView;
pub use expression::{
    BinaryExpressionOp, ExpressionEvaluationContext, ExpressionEvaluationError, ExpressionIr,
    ExpressionIrError, ExpressionPathRoot, ExpressionPathSegment, ExpressionValidationError,
    UnaryExpressionOp,
};
pub use feasibility::{
    CoreMutationFeasibilityService, FeasibilityIssue, FeasibilityIssueKind, FeasibilityStatus,
    MutationContext, MutationFeasibilityReport, MutationFeasibilityService, RequiredChoice,
    workspace_revision_for_project,
};
pub use frontend::pilot::{
    PilotDocumentationBlock, PilotExportDocument, PilotExportElement, PilotExportRelationship,
    PilotImportError, PilotSource, load_pilot_export, normalize_pilot_export,
    normalize_pilot_export_for_compare,
};
pub use goal::{
    GoalCheckEvaluation, GoalEvaluation, GoalPolicy, SemanticGoalCheck, SemanticGoalExplanation,
    SemanticGoalProfile, SemanticGoalProfileKind, SemanticGoalSpec, default_model_quality_profile,
    evaluate_semantic_goal, explain_semantic_goal,
};
pub use graph::{Edge, Graph, GraphError, NodeId};
pub use identity::{
    ConceptId, ElementId, PackageId, ProfileId, RelationshipId, SourceSpanRef, StdlibVersion,
    stable_digest, workspace_revision_for_kir_document,
};
pub use ir::{
    KIR_SCHEMA_VERSION, KirDocument, KirElement, KirError, KirFieldKind, KirFieldRegistry,
    KirFieldSpec, REPRESENTATIVE_KIR_JSON, load_model_stack, load_model_stack_with_registry,
};
pub use language::{
    BaselineLibrary, CURRENT_DEFAULT_PROFILE_ID, LanguageProfile, LanguageProfileError,
    LibraryContext, MetamodelConceptRegistry, SemanticConcept, SourceLanguage,
    default_language_profile, default_metamodel_registry, load_language_profile,
};
pub use library::{
    BaselineLibraryConfig, KparLocator, KparPackageBuild, KparPackageSource, LibraryCacheMetadata,
    LibraryProviderConfig, LocalPackageManifest, LocalPackageRecord, LocalPackageRepository,
    LocalPackageSource, MercurioLockFile, MercurioLockedPackage, MercurioPackageBuildProvenance,
    MercurioPackageDependency, MercurioPackageManifest, MercurioPackageSourceProvenance,
    PackageKirCache, PackageKirCacheManifest, PackageReference, ResolvedLibraryArtifact,
    load_baseline_library_document, package_bytes_digest, parse_package_reference,
    write_kpar_package,
};
pub use mercurio_language_contracts::{CompileContext, LanguageRegistry, LanguageService};
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
pub use mpack::{
    MpackLanguageProfile, MpackLibrary, MpackManifest, MpackPythonPackage,
    MpackPythonWrapperBinding, MpackRequirements, MpackRulepack, MpackService,
    MpackValidationError, validate_mpack_manifest,
};
pub use mutation::{
    ChangedAttribute, ChangedSpecialization, ElementRef, MovedElement, MutationApplicationResult,
    MutationEvidence, MutationPlan, MutationProposal, RelationshipChange, RenamedElement,
    RetypedUsage, SemanticAffordanceContext, SemanticDiff, SemanticElementContext,
    SemanticExpression, SemanticFactContext, SemanticMutation, SemanticMutationCapabilityContext,
    SemanticReasoningContext, SemanticRelationshipContext, WorkspaceRevision,
    default_semantic_mutation_capability_context, diff_kir_documents,
    enrich_semantic_reasoning_context_with_child_affordances,
    enrich_semantic_reasoning_context_with_child_affordances_for_capability,
    enrich_semantic_reasoning_context_with_graph,
    semantic_reasoning_context_from_authoring_project,
    semantic_reasoning_context_from_authoring_project_with_oracle,
};
pub use outline::{
    EditorOutlineKey, EditorOutlineNodeDto, build_editor_outline,
    build_editor_outline_index_for_graph, build_semantic_editor_outline_from_document,
};
pub use paths::{
    bundled_extension_repo_path, bundled_package_repo_path, bundled_stdlib_package_set_path,
    default_kernel_library_path, default_package_kir_cache_path, default_package_repo_path,
    default_stdlib_path, default_stdlib_rulepack_path, default_user_config_path,
    default_workspace_root, repo_path, repo_root,
};
pub use performance::{
    CachePerformanceConfig, CachePerformanceReport, CachePerformanceScenarioReport,
    CachePerformanceTimings, CoreScalabilityCreationStrategy, CoreScalabilityMetricConfig,
    CoreScalabilityReport, CoreScalabilityScenarioReport, CoreScalabilityTimings,
    EmfComparisonReport, KirPerformanceConfig, KirPerformanceMemory, KirPerformanceReport,
    KirPerformanceScenarioReport, KirPerformanceTimings, MemoryMetric, SemanticDiffSummary,
    TimingMetric, run_cache_performance, run_core_scalability_metric, run_kir_performance,
};
pub use plugin_registry::{
    InstalledMpack, MpackActivationIndex, MpackAssetRef,
    PluginInstallSource as RegistryPluginInstallSource, PluginRegistryError,
    default_plugin_registry_root, install_plugin_manifest, installed_mpack_manifests,
    installed_plugin_manifest_paths, mpack_activation_index, plugin_manifest_dir,
    plugin_package_digest, plugin_registry_root, publish_plugin_package,
    read_plugin_install_source, read_plugin_manifest as read_registry_plugin_manifest,
};
pub use proposal::{
    Proposal, ProposalStatus, PullRequestBinding, PullRequestState, SemanticImpact,
    SemanticImpactStatus, SemanticImpactSummary,
};
pub use python_codegen::{PythonWrapperGeneration, generate_python_wrappers};
#[allow(deprecated)]
pub use query::{
    FilterExpr, OrderBy, Projection, Query, QueryEngine, QueryError, QueryResultSet, QuerySource,
    RequirementTrace, SortDirection, TermPattern, TriplePattern, elements_with_metadata,
    parse_query, requirement_traces,
};
pub use runtime::{
    ExecutionContext, QueryResult, Runtime, RuntimeArtifact, RuntimeError, RuntimeProfile,
    RuntimeProfileTimings,
};
pub use semantic_compare::{
    SemanticCompareError, SemanticCompareOptions, SemanticComparisonReport,
    SemanticElementMismatch, SemanticSnapshot, SemanticSnapshotAttribute, SemanticSnapshotElement,
    SemanticSourceSpan, SemanticValueMismatch, SnapshotMode, build_semantic_snapshot,
    build_semantic_snapshot_with_registry, compare_snapshots, compare_snapshots_with_options,
};
pub use semantic_profile::{
    AttributePolicyAnswer, CapabilityAnswer, ConservativeSemanticCapabilityOracle,
    SemanticCapabilityOracle,
};
pub use semantic_target::{
    IncludeSubtypes, ResolvedSemanticTarget, SemanticTarget, SemanticTargetError,
    SemanticTargetResolver, TargetLayers,
};
pub use session::{
    CommitMode, CommitResult, CommitStrategy, ForkElement, ForkElementSpec, KirOverlay, ModelFork,
    ModelSession, ModelWorkspace, SessionError, WorkspaceSnapshot,
};
pub use source_set::{
    SourceDocument, compile_source_document_with_registry, compile_source_documents,
    compile_source_documents_with_registry, parse_source_module,
};
pub use syntax_compare::{
    SyntaxComparisonReport, SyntaxNodeMismatch, SyntaxSnapshot, SyntaxSnapshotNode,
    SyntaxSourceSpan, build_rust_syntax_snapshot, compare_syntax_snapshots,
};
#[allow(deprecated)]
pub use views::{
    ElementDetailsDto, ElementPropertyRowDto, ElementPropertyTableDto, ElementSummaryDto,
    ExplorerAttributeDto, GraphDto, GraphEdgeDto, GraphNodeDto, GraphScope, InheritedPropertiesDto,
    InheritedPropertyValueDto, L2ExplorerEdgeDto, L2ExplorerGraphDto, L2ExplorerNodeDto,
    L2ExplorerRequestDto, LibraryTreeNodeDto, MetatypeExplorerEdgeDto, MetatypeExplorerGraphDto,
    MetatypeExplorerNodeDto, MetatypeExplorerRequestDto, ModelMetadataDto, RequirementSourceDto,
    RequirementTableColumnDto, RequirementTableRowDto, RequirementTableViewDto, SearchResultDto,
    document_model_metadata_view, element_details, graph_view, l2_explorer_view, library_tree_view,
    library_tree_view_from_document, metatype_explorer_view, model_metadata_view,
    requirements_table_view, search_view,
};
pub use workspace::{
    ResolvedWorkspaceContext, ResolvedWorkspaceLibrary, WorkspaceConfig, WorkspaceConfigError,
    WorkspaceContextOptions, WorkspaceLibraryConfig, WorkspaceLibraryRole, WorkspacePluginConfig,
    discover_workspace_config_path, resolve_workspace_context,
    resolve_workspace_context_from_config_path, resolve_workspace_context_with_options,
};
pub use workspace_cache::{
    PersistentCacheStatus, PersistentCompileResult, PersistentWorkspaceCache,
    PersistentWorkspaceCacheOptions, RuntimeCachePolicy, WorkspaceCompileArtifactKey,
    WorkspaceCompileCacheManifest, WorkspaceCompileCacheOutputs, WorkspaceSourceFileFingerprint,
    source_file_fingerprints, workspace_compile_artifact_key,
};

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use mercurio_language_contracts::LanguageRegistry;

use crate::ir::{KIR_SCHEMA_VERSION, KirDocument, KirError};
use crate::runtime::{Runtime, RuntimeArtifact};
use crate::source_set::{
    SourceDocument, compile_source_documents, compile_source_documents_with_registry,
};

const CACHE_SCHEMA_VERSION: u32 = 3;
const ARTIFACT_FAMILY_COMPILE: &str = "compile";
const DOCUMENT_FILE_NAME: &str = "document.kir.json";
const BINARY_DOCUMENT_FILE_NAME: &str = "document.mkir";
const BINARY_DOCUMENT_MANIFEST_FILE_NAME: &str = "document.mkir.manifest.json";
const MANIFEST_FILE_NAME: &str = "manifest.json";
const RUNTIME_ARTIFACT_FILE_NAME: &str = "runtime-artifact.json";

#[derive(Debug, Clone)]
pub struct PersistentWorkspaceCache {
    root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PersistentCacheStatus {
    FreshCompile,
    PersistentHit,
    PersistentMiss,
    PersistentRejected { reason: String },
}

#[derive(Debug, Clone)]
pub struct PersistentCompileResult {
    pub document: KirDocument,
    pub runtime_artifact: RuntimeArtifact,
    pub cache_status: PersistentCacheStatus,
    pub artifact_key: String,
    pub cache_write_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSourceFileFingerprint {
    pub path: String,
    pub size_bytes: usize,
    pub content_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCompileArtifactKey {
    pub source_authority: String,
    pub source_tree_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_config_digest: Option<String>,
    pub compiler_digest: String,
    pub kir_schema_version: String,
    pub library_context_digest: String,
    pub mapping_rules_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCompileCacheManifest {
    pub cache_schema_version: u32,
    pub artifact_family: String,
    pub artifact_key: String,
    pub key: WorkspaceCompileArtifactKey,
    pub files: Vec<WorkspaceSourceFileFingerprint>,
    pub outputs: WorkspaceCompileCacheOutputs,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCompileCacheOutputs {
    pub kir: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_kir: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_kir_manifest: Option<String>,
    pub runtime_artifact: String,
}

enum CacheLookup {
    Hit {
        document: KirDocument,
        runtime_artifact: RuntimeArtifact,
    },
    Miss,
    Rejected(String),
}

impl PersistentWorkspaceCache {
    pub fn for_workspace_root(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            root: workspace_root
                .into()
                .join(".workspace-cache")
                .join("compile"),
        }
    }

    pub fn from_cache_root(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn compile_source_documents(
        &self,
        source_documents: Vec<SourceDocument>,
        library_context: &KirDocument,
        workspace_config_path: Option<&Path>,
    ) -> Result<PersistentCompileResult, KirError> {
        self.compile_source_documents_with(
            source_documents,
            library_context,
            workspace_config_path,
            |source_documents, library_context| {
                compile_source_documents(source_documents, library_context)
            },
        )
    }

    pub fn compile_source_documents_with_registry(
        &self,
        source_documents: Vec<SourceDocument>,
        library_context: &KirDocument,
        workspace_config_path: Option<&Path>,
        registry: &LanguageRegistry,
    ) -> Result<PersistentCompileResult, KirError> {
        self.compile_source_documents_with(
            source_documents,
            library_context,
            workspace_config_path,
            |source_documents, library_context| {
                compile_source_documents_with_registry(source_documents, library_context, registry)
            },
        )
    }

    fn compile_source_documents_with(
        &self,
        source_documents: Vec<SourceDocument>,
        library_context: &KirDocument,
        workspace_config_path: Option<&Path>,
        compile: impl Fn(Vec<SourceDocument>, &KirDocument) -> Result<KirDocument, KirError>,
    ) -> Result<PersistentCompileResult, KirError> {
        let (key, files) = workspace_compile_artifact_key(
            &source_documents,
            library_context,
            workspace_config_path,
        )?;
        let artifact_key = artifact_key_digest(&key)?;

        match self.load_compile_artifact(&artifact_key, &key, &files)? {
            CacheLookup::Hit {
                document,
                runtime_artifact,
            } => {
                return Ok(PersistentCompileResult {
                    document,
                    runtime_artifact,
                    cache_status: PersistentCacheStatus::PersistentHit,
                    artifact_key,
                    cache_write_error: None,
                });
            }
            CacheLookup::Miss => {
                let document = compile(source_documents, library_context)?;
                let runtime_artifact = runtime_artifact_for_document(&document, library_context)?;
                let cache_write_error = self
                    .write_compile_artifact(
                        &artifact_key,
                        &key,
                        &files,
                        &document,
                        &runtime_artifact,
                    )
                    .err()
                    .map(|err| err.to_string());
                return Ok(PersistentCompileResult {
                    document,
                    runtime_artifact,
                    cache_status: PersistentCacheStatus::PersistentMiss,
                    artifact_key,
                    cache_write_error,
                });
            }
            CacheLookup::Rejected(reason) => {
                let document = compile(source_documents, library_context)?;
                let runtime_artifact = runtime_artifact_for_document(&document, library_context)?;
                let cache_write_error = self
                    .write_compile_artifact(
                        &artifact_key,
                        &key,
                        &files,
                        &document,
                        &runtime_artifact,
                    )
                    .err()
                    .map(|err| err.to_string());
                return Ok(PersistentCompileResult {
                    document,
                    runtime_artifact,
                    cache_status: PersistentCacheStatus::PersistentRejected { reason },
                    artifact_key,
                    cache_write_error,
                });
            }
        }
    }

    fn artifact_dir(&self, artifact_key: &str) -> PathBuf {
        self.root
            .join("artifacts")
            .join(safe_cache_segment(artifact_key))
    }

    fn load_compile_artifact(
        &self,
        artifact_key: &str,
        key: &WorkspaceCompileArtifactKey,
        files: &[WorkspaceSourceFileFingerprint],
    ) -> Result<CacheLookup, KirError> {
        let artifact_dir = self.artifact_dir(artifact_key);
        let manifest_path = artifact_dir.join(MANIFEST_FILE_NAME);
        let document_path = artifact_dir.join(DOCUMENT_FILE_NAME);
        let binary_document_path = artifact_dir.join(BINARY_DOCUMENT_FILE_NAME);
        let binary_document_manifest_path = artifact_dir.join(BINARY_DOCUMENT_MANIFEST_FILE_NAME);
        let runtime_artifact_path = artifact_dir.join(RUNTIME_ARTIFACT_FILE_NAME);

        if !manifest_path.is_file() || !runtime_artifact_path.is_file() {
            return Ok(CacheLookup::Miss);
        }

        let manifest = match std::fs::read_to_string(&manifest_path)
            .ok()
            .and_then(|input| serde_json::from_str::<WorkspaceCompileCacheManifest>(&input).ok())
        {
            Some(manifest) => manifest,
            None => return Ok(CacheLookup::Rejected("manifest is unreadable".to_string())),
        };

        if let Some(reason) = manifest_rejection_reason(&manifest, artifact_key, key, files) {
            return Ok(CacheLookup::Rejected(reason));
        }

        let binary_cache_source = binary_cache_source_bytes(key, files)?;
        let binary_document = KirDocument::from_valid_binary_cache_paths(
            &binary_document_path,
            &binary_document_manifest_path,
            &binary_cache_source,
        )
        .unwrap_or(None);
        let document = match binary_document {
            Some(document) => document,
            None => {
                if !document_path.is_file() {
                    return Ok(CacheLookup::Rejected(
                        "cached KIR text and binary cache are missing".to_string(),
                    ));
                }
                match KirDocument::from_path(&document_path) {
                    Ok(document) => document,
                    Err(err) => {
                        return Ok(CacheLookup::Rejected(format!(
                            "cached KIR is invalid: {err}"
                        )));
                    }
                }
            }
        };
        let runtime_artifact =
            match serde_json::from_str(&std::fs::read_to_string(&runtime_artifact_path)?) {
                Ok(artifact) => artifact,
                Err(err) => {
                    return Ok(CacheLookup::Rejected(format!(
                        "cached runtime artifact is invalid: {err}"
                    )));
                }
            };
        Ok(CacheLookup::Hit {
            document,
            runtime_artifact,
        })
    }

    fn write_compile_artifact(
        &self,
        artifact_key: &str,
        key: &WorkspaceCompileArtifactKey,
        files: &[WorkspaceSourceFileFingerprint],
        document: &KirDocument,
        runtime_artifact: &RuntimeArtifact,
    ) -> Result<(), KirError> {
        let final_dir = self.artifact_dir(artifact_key);
        if final_dir.is_dir() {
            return self.write_compile_artifact_files(
                &final_dir,
                artifact_key,
                key,
                files,
                document,
                runtime_artifact,
            );
        }

        let tmp_dir = self.root.join("tmp").join(format!(
            "{}-{}",
            process_id_segment(),
            unique_cache_nonce()
        ));
        std::fs::create_dir_all(&tmp_dir)?;

        self.write_compile_artifact_files(
            &tmp_dir,
            artifact_key,
            key,
            files,
            document,
            runtime_artifact,
        )?;

        if let Some(parent) = final_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        match std::fs::rename(&tmp_dir, &final_dir) {
            Ok(()) => Ok(()),
            Err(err) if final_dir.is_dir() => {
                let _ = std::fs::remove_dir_all(&tmp_dir);
                if err.kind() == std::io::ErrorKind::AlreadyExists {
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(KirError::Io(err)),
        }
    }

    fn write_compile_artifact_files(
        &self,
        dir: &Path,
        artifact_key: &str,
        key: &WorkspaceCompileArtifactKey,
        files: &[WorkspaceSourceFileFingerprint],
        document: &KirDocument,
        runtime_artifact: &RuntimeArtifact,
    ) -> Result<(), KirError> {
        std::fs::create_dir_all(dir)?;
        let manifest = WorkspaceCompileCacheManifest {
            cache_schema_version: CACHE_SCHEMA_VERSION,
            artifact_family: ARTIFACT_FAMILY_COMPILE.to_string(),
            artifact_key: artifact_key.to_string(),
            key: key.clone(),
            files: files.to_vec(),
            outputs: WorkspaceCompileCacheOutputs {
                kir: DOCUMENT_FILE_NAME.to_string(),
                binary_kir: Some(BINARY_DOCUMENT_FILE_NAME.to_string()),
                binary_kir_manifest: Some(BINARY_DOCUMENT_MANIFEST_FILE_NAME.to_string()),
                runtime_artifact: RUNTIME_ARTIFACT_FILE_NAME.to_string(),
            },
        };
        document.write_pretty_to_path(&dir.join(DOCUMENT_FILE_NAME))?;
        document.write_binary_cache_to_paths(
            &dir.join(BINARY_DOCUMENT_FILE_NAME),
            &dir.join(BINARY_DOCUMENT_MANIFEST_FILE_NAME),
            &binary_cache_source_bytes(key, files)?,
        )?;
        std::fs::write(
            dir.join(RUNTIME_ARTIFACT_FILE_NAME),
            serde_json::to_string_pretty(runtime_artifact)?,
        )?;
        std::fs::write(
            dir.join(MANIFEST_FILE_NAME),
            serde_json::to_string_pretty(&manifest)?,
        )?;

        let roundtrip_manifest: WorkspaceCompileCacheManifest =
            serde_json::from_str(&std::fs::read_to_string(dir.join(MANIFEST_FILE_NAME))?)?;
        if roundtrip_manifest != manifest {
            return Err(KirError::Model(
                "persistent cache manifest failed roundtrip validation".to_string(),
            ));
        }
        KirDocument::from_path(&dir.join(DOCUMENT_FILE_NAME))?;
        let binary_source = binary_cache_source_bytes(key, files)?;
        if KirDocument::from_valid_binary_cache_paths(
            &dir.join(BINARY_DOCUMENT_FILE_NAME),
            &dir.join(BINARY_DOCUMENT_MANIFEST_FILE_NAME),
            &binary_source,
        )?
        .is_none()
        {
            return Err(KirError::Model(
                "persistent cache binary KIR failed manifest validation".to_string(),
            ));
        }
        let _: RuntimeArtifact = serde_json::from_str(&std::fs::read_to_string(
            dir.join(RUNTIME_ARTIFACT_FILE_NAME),
        )?)?;
        Ok(())
    }
}

pub fn workspace_compile_artifact_key(
    source_documents: &[SourceDocument],
    library_context: &KirDocument,
    workspace_config_path: Option<&Path>,
) -> Result<
    (
        WorkspaceCompileArtifactKey,
        Vec<WorkspaceSourceFileFingerprint>,
    ),
    KirError,
> {
    let files = source_file_fingerprints(source_documents);
    let source_tree_digest = digest_source_file_fingerprints(&files);
    let workspace_config_digest = workspace_config_path
        .filter(|path| path.is_file())
        .map(digest_file)
        .transpose()?;
    let library_context_digest = digest_json(library_context)?;
    let mapping_rules_digest = mapping_rules_digest()?;

    Ok((
        WorkspaceCompileArtifactKey {
            source_authority: "local_files".to_string(),
            source_tree_digest,
            workspace_config_digest,
            compiler_digest: compiler_digest(),
            kir_schema_version: KIR_SCHEMA_VERSION.to_string(),
            library_context_digest,
            mapping_rules_digest,
        },
        files,
    ))
}

pub fn source_file_fingerprints(
    source_documents: &[SourceDocument],
) -> Vec<WorkspaceSourceFileFingerprint> {
    let mut files = source_documents
        .iter()
        .map(|source| WorkspaceSourceFileFingerprint {
            path: normalized_source_path(&source.path),
            size_bytes: source.content.len(),
            content_digest: digest_labeled_chunks([(
                "content".as_bytes(),
                source.content.as_bytes(),
            )]),
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.path.cmp(&right.path));
    files
}

fn manifest_rejection_reason(
    manifest: &WorkspaceCompileCacheManifest,
    artifact_key: &str,
    key: &WorkspaceCompileArtifactKey,
    files: &[WorkspaceSourceFileFingerprint],
) -> Option<String> {
    if manifest.cache_schema_version != CACHE_SCHEMA_VERSION {
        return Some("cache schema version changed".to_string());
    }
    if manifest.artifact_family != ARTIFACT_FAMILY_COMPILE {
        return Some("artifact family does not match compile cache".to_string());
    }
    if manifest.artifact_key != artifact_key {
        return Some("artifact key does not match manifest location".to_string());
    }
    if &manifest.key != key {
        return Some("artifact key inputs changed".to_string());
    }
    if manifest.files != files {
        return Some("source file fingerprints changed".to_string());
    }
    if manifest.outputs.kir != DOCUMENT_FILE_NAME {
        return Some("manifest output path is not recognized".to_string());
    }
    if manifest.outputs.binary_kir.as_deref() != Some(BINARY_DOCUMENT_FILE_NAME) {
        return Some("manifest binary KIR output path is not recognized".to_string());
    }
    if manifest.outputs.binary_kir_manifest.as_deref() != Some(BINARY_DOCUMENT_MANIFEST_FILE_NAME)
    {
        return Some("manifest binary KIR manifest path is not recognized".to_string());
    }
    if manifest.outputs.runtime_artifact != RUNTIME_ARTIFACT_FILE_NAME {
        return Some("manifest runtime artifact path is not recognized".to_string());
    }
    None
}

fn binary_cache_source_bytes(
    key: &WorkspaceCompileArtifactKey,
    files: &[WorkspaceSourceFileFingerprint],
) -> Result<Vec<u8>, KirError> {
    Ok(serde_json::to_vec(&(key, files))?)
}

fn runtime_artifact_for_document(
    document: &KirDocument,
    library_context: &KirDocument,
) -> Result<RuntimeArtifact, KirError> {
    let merged_document = KirDocument::merge([library_context.clone(), document.clone()])?;
    Runtime::from_document(merged_document)
        .map(Runtime::into_artifact)
        .map_err(|err| KirError::Model(format!("failed to build runtime artifact: {err}")))
}

fn artifact_key_digest(key: &WorkspaceCompileArtifactKey) -> Result<String, KirError> {
    digest_json(key)
}

fn digest_json<T: Serialize>(value: &T) -> Result<String, KirError> {
    Ok(digest_labeled_chunks([(
        "json".as_bytes(),
        serde_json::to_vec(value)?.as_slice(),
    )]))
}

fn digest_file(path: &Path) -> Result<String, KirError> {
    let bytes = std::fs::read(path)?;
    Ok(digest_labeled_chunks([(
        "file".as_bytes(),
        bytes.as_slice(),
    )]))
}

fn digest_source_file_fingerprints(files: &[WorkspaceSourceFileFingerprint]) -> String {
    digest_labeled_chunks(files.iter().flat_map(|file| {
        [
            ("path".as_bytes(), file.path.as_bytes()),
            ("content_digest".as_bytes(), file.content_digest.as_bytes()),
        ]
    }))
}

fn mapping_rules_digest() -> Result<String, KirError> {
    Ok(digest_labeled_chunks([(
        "mapping".as_bytes(),
        "language-service-provided".as_bytes(),
    )]))
}

fn compiler_digest() -> String {
    digest_labeled_chunks([
        ("crate".as_bytes(), "mercurio-foundation".as_bytes()),
        ("version".as_bytes(), env!("CARGO_PKG_VERSION").as_bytes()),
        ("kir_schema".as_bytes(), KIR_SCHEMA_VERSION.as_bytes()),
    ])
}

fn digest_labeled_chunks<'a, I>(chunks: I) -> String
where
    I: IntoIterator<Item = (&'a [u8], &'a [u8])>,
{
    let mut hash = FNV_OFFSET;
    for (label, bytes) in chunks {
        hash = digest_bytes(hash, &(label.len() as u64).to_le_bytes());
        hash = digest_bytes(hash, label);
        hash = digest_bytes(hash, &(bytes.len() as u64).to_le_bytes());
        hash = digest_bytes(hash, bytes);
    }
    format!("fnv1a64:{hash:016x}")
}

const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

fn digest_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn normalized_source_path(path: &str) -> String {
    path.replace('\\', "/")
}

fn safe_cache_segment(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn process_id_segment() -> String {
    std::process::id().to_string()
}

#[cfg(target_arch = "wasm32")]
fn process_id_segment() -> String {
    "wasm".to_string()
}

fn unique_cache_nonce() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or(0);
    nanos ^ counter.rotate_left(17)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use mercurio_language_contracts::LanguageRegistry;
    use mercurio_kir::BinaryKirCacheManifest;
    use serde_json::Value;

    use super::{
        BINARY_DOCUMENT_FILE_NAME, BINARY_DOCUMENT_MANIFEST_FILE_NAME, DOCUMENT_FILE_NAME,
        PersistentCacheStatus, PersistentWorkspaceCache, WorkspaceCompileCacheManifest,
        binary_cache_source_bytes, source_file_fingerprints, workspace_compile_artifact_key,
    };
    use crate::ir::{KIR_SCHEMA_VERSION, KirDocument, KirElement};
    use crate::runtime::Runtime;
    use crate::source_set::SourceDocument;

    #[test]
    fn persistent_compile_cache_reuses_unchanged_artifact() {
        let root = temp_dir("persistent_hit");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert_eq!(first.cache_status, PersistentCacheStatus::PersistentMiss);
        assert_eq!(second.cache_status, PersistentCacheStatus::PersistentHit);
        assert_eq!(first.document, second.document);
        assert!(second.cache_write_error.is_none());

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_hit_preserves_runtime_end_state() {
        let root = temp_dir("persistent_runtime_equivalence");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![
            SourceDocument::new("domain.model", "package Domain { part def Camera; }"),
            SourceDocument::new(
                "usage.model",
                "package Usage {
                  part camera : Domain.Camera;
                }",
            ),
        ];

        let miss = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let hit = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert_eq!(miss.cache_status, PersistentCacheStatus::PersistentMiss);
        assert_eq!(hit.cache_status, PersistentCacheStatus::PersistentHit);
        assert_eq!(miss.document, hit.document);

        let miss_runtime = Runtime::from_artifact(miss.runtime_artifact).unwrap();
        let hit_runtime = Runtime::from_artifact(hit.runtime_artifact).unwrap();

        assert_eq!(
            miss_runtime.graph().elements(),
            hit_runtime.graph().elements()
        );
        assert_eq!(miss_runtime.graph().edges(), hit_runtime.graph().edges());
        assert_eq!(miss_runtime.derived(), hit_runtime.derived());

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_invalidates_changed_source() {
        let root = temp_dir("persistent_changed_source");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();

        let first_sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];
        let second_sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def OtherThing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                first_sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let second = cache
            .compile_source_documents_with_registry(
                second_sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert_eq!(first.cache_status, PersistentCacheStatus::PersistentMiss);
        assert_eq!(second.cache_status, PersistentCacheStatus::PersistentMiss);
        assert_ne!(first.artifact_key, second.artifact_key);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_invalidates_changed_config() {
        let root = temp_dir("persistent_changed_config");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let config_path = root.join("workspace.json");
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        std::fs::write(&config_path, r#"{"version":1,"name":"A"}"#).unwrap();
        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                Some(&config_path),
                &test_language_registry(),
            )
            .unwrap();
        std::fs::write(&config_path, r#"{"version":1,"name":"B"}"#).unwrap();
        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                Some(&config_path),
                &test_language_registry(),
            )
            .unwrap();

        assert_eq!(first.cache_status, PersistentCacheStatus::PersistentMiss);
        assert_eq!(second.cache_status, PersistentCacheStatus::PersistentMiss);
        assert_ne!(first.artifact_key, second.artifact_key);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_rejects_corrupt_manifest_and_falls_back() {
        let root = temp_dir("persistent_corrupt_manifest");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let manifest_path = cache
            .root()
            .join("artifacts")
            .join(first.artifact_key.replace(':', "_"))
            .join("manifest.json");
        std::fs::write(&manifest_path, "{ not-json").unwrap();

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert!(matches!(
            second.cache_status,
            PersistentCacheStatus::PersistentRejected { .. }
        ));
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_rejects_corrupt_kir_and_falls_back() {
        let root = temp_dir("persistent_corrupt_kir");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let document_path = cache
            .root()
            .join("artifacts")
            .join(first.artifact_key.replace(':', "_"))
            .join("document.kir.json");
        std::fs::write(&document_path, "{ not-json").unwrap();
        let dir = artifact_dir(&cache, &first.artifact_key);
        mutate_binary_manifest(&dir, |manifest| {
            manifest.binary_digest = "fnv1a64:stale".to_string();
        });

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert!(matches!(
            second.cache_status,
            PersistentCacheStatus::PersistentRejected { .. }
        ));
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_can_hit_from_binary_when_text_kir_is_corrupt() {
        let root = temp_dir("persistent_binary_hit");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        std::fs::write(artifact_dir(&cache, &first.artifact_key).join(DOCUMENT_FILE_NAME), "{ bad")
            .unwrap();

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert_eq!(second.cache_status, PersistentCacheStatus::PersistentHit);
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_misses_stale_binary_source_digest() {
        let root = temp_dir("persistent_binary_stale_source");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let dir = artifact_dir(&cache, &first.artifact_key);
        mutate_binary_manifest(&dir, |manifest| {
            manifest.source_digest = "fnv1a64:stale".to_string();
        });
        std::fs::write(dir.join(DOCUMENT_FILE_NAME), "{ bad").unwrap();

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert!(matches!(
            second.cache_status,
            PersistentCacheStatus::PersistentRejected { .. }
        ));
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_misses_stale_binary_digest() {
        let root = temp_dir("persistent_binary_stale_binary");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let dir = artifact_dir(&cache, &first.artifact_key);
        mutate_binary_manifest(&dir, |manifest| {
            manifest.binary_digest = "fnv1a64:stale".to_string();
        });
        std::fs::write(dir.join(DOCUMENT_FILE_NAME), "{ bad").unwrap();

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert!(matches!(
            second.cache_status,
            PersistentCacheStatus::PersistentRejected { .. }
        ));
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_misses_missing_binary_manifest() {
        let root = temp_dir("persistent_binary_missing_manifest");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let dir = artifact_dir(&cache, &first.artifact_key);
        std::fs::remove_file(dir.join(BINARY_DOCUMENT_MANIFEST_FILE_NAME)).unwrap();
        std::fs::write(dir.join(DOCUMENT_FILE_NAME), "{ bad").unwrap();

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert!(matches!(
            second.cache_status,
            PersistentCacheStatus::PersistentRejected { .. }
        ));
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn persistent_compile_cache_falls_back_from_bad_binary_format_to_text_kir() {
        let root = temp_dir("persistent_binary_bad_format");
        let cache = PersistentWorkspaceCache::for_workspace_root(&root);
        let library_context = test_library_context();
        let sources = vec![SourceDocument::new(
            "demo.model",
            "package Demo { part def Thing; }",
        )];

        let first = cache
            .compile_source_documents_with_registry(
                sources.clone(),
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();
        let dir = artifact_dir(&cache, &first.artifact_key);
        let bad_binary = b"NOPE";
        std::fs::write(dir.join(BINARY_DOCUMENT_FILE_NAME), bad_binary).unwrap();
        let (key, files) =
            workspace_compile_artifact_key(&sources, &library_context, None).unwrap();
        let source_bytes = binary_cache_source_bytes(&key, &files).unwrap();
        let manifest = BinaryKirCacheManifest::for_bytes(&source_bytes, bad_binary);
        std::fs::write(
            dir.join(BINARY_DOCUMENT_MANIFEST_FILE_NAME),
            serde_json::to_string_pretty(&manifest).unwrap(),
        )
        .unwrap();

        let second = cache
            .compile_source_documents_with_registry(
                sources,
                &library_context,
                None,
                &test_language_registry(),
            )
            .unwrap();

        assert_eq!(second.cache_status, PersistentCacheStatus::PersistentHit);
        assert_eq!(first.document, second.document);

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn source_file_fingerprints_are_path_order_independent() {
        let left = vec![
            SourceDocument::new("b.model", "package B {}"),
            SourceDocument::new("a.model", "package A {}"),
        ];
        let right = vec![
            SourceDocument::new("a.model", "package A {}"),
            SourceDocument::new("b.model", "package B {}"),
        ];

        assert_eq!(
            source_file_fingerprints(&left),
            source_file_fingerprints(&right)
        );
    }

    #[test]
    fn manifest_roundtrip_keeps_required_key_fields() {
        let manifest = WorkspaceCompileCacheManifest {
            cache_schema_version: super::CACHE_SCHEMA_VERSION,
            artifact_family: "compile".to_string(),
            artifact_key: "fnv1a64:test".to_string(),
            key: super::WorkspaceCompileArtifactKey {
                source_authority: "local_files".to_string(),
                source_tree_digest: "fnv1a64:source".to_string(),
                workspace_config_digest: Some("fnv1a64:config".to_string()),
                compiler_digest: "fnv1a64:compiler".to_string(),
                kir_schema_version: "0.2".to_string(),
                library_context_digest: "fnv1a64:library".to_string(),
                mapping_rules_digest: "fnv1a64:mapping".to_string(),
            },
            files: Vec::new(),
            outputs: super::WorkspaceCompileCacheOutputs {
                kir: "document.kir.json".to_string(),
                binary_kir: Some("document.mkir".to_string()),
                binary_kir_manifest: Some("document.mkir.manifest.json".to_string()),
                runtime_artifact: "runtime-artifact.json".to_string(),
            },
        };

        let encoded = serde_json::to_string(&manifest).unwrap();
        let decoded: WorkspaceCompileCacheManifest = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded, manifest);
    }

    fn artifact_dir(cache: &PersistentWorkspaceCache, artifact_key: &str) -> std::path::PathBuf {
        cache
            .root()
            .join("artifacts")
            .join(artifact_key.replace(':', "_"))
    }

    fn mutate_binary_manifest(
        artifact_dir: &std::path::Path,
        mutate: impl FnOnce(&mut BinaryKirCacheManifest),
    ) {
        let path = artifact_dir.join(BINARY_DOCUMENT_MANIFEST_FILE_NAME);
        let mut manifest: BinaryKirCacheManifest =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        mutate(&mut manifest);
        std::fs::write(&path, serde_json::to_string_pretty(&manifest).unwrap()).unwrap();
    }

    fn test_library_context() -> KirDocument {
        KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                Value::String(KIR_SCHEMA_VERSION.to_string()),
            )]),
            elements: vec![
                KirElement {
                    id: "Parts::Part".to_string(),
                    kind: "Model::PartDefinition".to_string(),
                    layer: 1,
                    properties: BTreeMap::from([(
                        "qualified_name".to_string(),
                        Value::String("Parts.Part".to_string()),
                    )]),
                },
                KirElement {
                    id: "Items::Item::subparts".to_string(),
                    kind: "Model::PartUsage".to_string(),
                    layer: 1,
                    properties: BTreeMap::from([(
                        "qualified_name".to_string(),
                        Value::String("Items.Item.subparts".to_string()),
                    )]),
                },
            ],
        }
    }

    fn test_language_registry() -> LanguageRegistry {
        crate::test_support::toy_language::registry()
    }

    fn temp_dir(label: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "mercurio_workspace_cache_{label}_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        root
    }
}

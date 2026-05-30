use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ir::{KirDocument, KirError};
use crate::mutation::WorkspaceRevision;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ElementId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RelationshipId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConceptId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ProfileId(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct StdlibVersion(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpanRef {
    pub file: String,
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

macro_rules! semantic_id {
    ($type_name:ident) => {
        impl $type_name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl From<String> for $type_name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl From<&str> for $type_name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Display for $type_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

semantic_id!(ElementId);
semantic_id!(RelationshipId);
semantic_id!(ConceptId);
semantic_id!(PackageId);
semantic_id!(ProfileId);
semantic_id!(StdlibVersion);

pub fn workspace_revision_for_kir_document(
    document: &KirDocument,
) -> Result<WorkspaceRevision, KirError> {
    let bytes = serde_json::to_vec(document)?;
    Ok(WorkspaceRevision {
        fingerprint: stable_digest([("kir-document".as_bytes(), bytes.as_slice())]),
    })
}

pub fn stable_digest<'a, I>(chunks: I) -> String
where
    I: IntoIterator<Item = (&'a [u8], &'a [u8])>,
{
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for (label, bytes) in chunks {
        for byte in label
            .iter()
            .chain(&(bytes.len() as u64).to_le_bytes())
            .chain(bytes)
        {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }

    format!("fnv1a64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{KirDocument, KirElement};

    use super::*;

    #[test]
    fn kir_workspace_revision_is_stable() {
        let document = KirDocument {
            metadata: BTreeMap::new(),
            elements: vec![KirElement {
                id: "Demo".to_string(),
                kind: "Package".to_string(),
                layer: 2,
                properties: BTreeMap::new(),
            }],
        };

        let first = workspace_revision_for_kir_document(&document).unwrap();
        let second = workspace_revision_for_kir_document(&document).unwrap();

        assert_eq!(first, second);
        assert!(first.fingerprint.starts_with("fnv1a64:"));
    }
}

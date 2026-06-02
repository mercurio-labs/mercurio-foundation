use std::collections::{BTreeMap, HashMap};
use std::io::{Cursor, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

use crate::{
    KIR_SCHEMA_VERSION, KIR_SCHEMA_VERSION_METADATA_KEY, KirDocument, KirElement, KirError,
};

const MAGIC: &[u8; 4] = b"MKIR";
pub const BINARY_KIR_FORMAT_VERSION: u16 = 1;
pub const BINARY_KIR_GENERATOR: &str = concat!("mercurio-kir/", env!("CARGO_PKG_VERSION"));

const TAG_NULL: u8 = 0;
const TAG_BOOL: u8 = 1;
const TAG_I64: u8 = 2;
const TAG_U64: u8 = 3;
const TAG_F64: u8 = 4;
const TAG_STRING: u8 = 5;
const TAG_ARRAY: u8 = 6;
const TAG_OBJECT: u8 = 7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryKirCacheManifest {
    pub binary_format_version: u16,
    pub kir_schema_version: String,
    pub source_digest: String,
    pub binary_digest: String,
    pub generator: String,
}

impl BinaryKirCacheManifest {
    pub fn for_bytes(source_bytes: &[u8], binary_bytes: &[u8]) -> Self {
        Self {
            binary_format_version: BINARY_KIR_FORMAT_VERSION,
            kir_schema_version: KIR_SCHEMA_VERSION.to_string(),
            source_digest: stable_bytes_digest(source_bytes),
            binary_digest: stable_bytes_digest(binary_bytes),
            generator: BINARY_KIR_GENERATOR.to_string(),
        }
    }

    pub fn is_valid_for_bytes(&self, source_bytes: &[u8], binary_bytes: &[u8]) -> bool {
        self.binary_format_version == BINARY_KIR_FORMAT_VERSION
            && self.kir_schema_version == KIR_SCHEMA_VERSION
            && self.source_digest == stable_bytes_digest(source_bytes)
            && self.binary_digest == stable_bytes_digest(binary_bytes)
    }
}

impl KirDocument {
    pub fn write_binary_to_path(&self, path: &Path) -> Result<(), KirError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let bytes = self.to_binary_bytes()?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    pub fn write_binary_cache_to_paths(
        &self,
        binary_path: &Path,
        manifest_path: &Path,
        source_bytes: &[u8],
    ) -> Result<BinaryKirCacheManifest, KirError> {
        if let Some(parent) = binary_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if let Some(parent) = manifest_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let binary_bytes = self.to_binary_bytes()?;
        let manifest = BinaryKirCacheManifest::for_bytes(source_bytes, &binary_bytes);
        std::fs::write(binary_path, &binary_bytes)?;
        std::fs::write(manifest_path, serde_json::to_string_pretty(&manifest)?)?;
        Ok(manifest)
    }

    pub fn from_valid_binary_cache_paths(
        binary_path: &Path,
        manifest_path: &Path,
        source_bytes: &[u8],
    ) -> Result<Option<Self>, KirError> {
        let binary_bytes = match std::fs::read(binary_path) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(KirError::Io(err)),
        };
        let manifest_bytes = match std::fs::read(manifest_path) {
            Ok(bytes) => bytes,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(KirError::Io(err)),
        };
        let manifest: BinaryKirCacheManifest = serde_json::from_slice(&manifest_bytes)?;
        if !manifest.is_valid_for_bytes(source_bytes, &binary_bytes) {
            return Ok(None);
        }

        Self::from_binary_bytes(&binary_bytes).map(Some)
    }

    pub fn from_binary_path(path: &Path) -> Result<Self, KirError> {
        Self::from_binary_bytes(&std::fs::read(path)?)
    }

    pub fn to_binary_bytes(&self) -> Result<Vec<u8>, KirError> {
        let document = self.clone().normalized_for_persistence();
        document.validate_persisted()?;

        let string_table = StringTable::from_document(&document)?;
        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        write_u16(&mut out, BINARY_KIR_FORMAT_VERSION)?;
        write_string_table(&mut out, &string_table)?;
        write_properties(&mut out, &document.metadata, &string_table)?;
        write_u32(&mut out, document.elements.len())?;
        for element in &document.elements {
            write_string_id(&mut out, &string_table, &element.id)?;
            write_string_id(&mut out, &string_table, &element.kind)?;
            out.write_all(&[element.layer])?;
            write_properties(&mut out, &element.properties, &string_table)?;
        }
        Ok(out)
    }

    pub fn from_binary_bytes(bytes: &[u8]) -> Result<Self, KirError> {
        let mut cursor = Cursor::new(bytes);
        let mut magic = [0_u8; 4];
        cursor.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(KirError::Binary("invalid magic header".to_string()));
        }

        let format_version = read_u16(&mut cursor)?;
        if format_version != BINARY_KIR_FORMAT_VERSION {
            return Err(KirError::Binary(format!(
                "unsupported binary KIR format version {format_version}"
            )));
        }

        let strings = read_string_table(&mut cursor)?;
        let metadata = read_properties(&mut cursor, &strings)?;
        let element_count = read_u32(&mut cursor)? as usize;
        let mut elements = Vec::with_capacity(element_count);
        for _ in 0..element_count {
            let id = read_string_id(&mut cursor, &strings)?;
            let kind = read_string_id(&mut cursor, &strings)?;
            let mut layer = [0_u8; 1];
            cursor.read_exact(&mut layer)?;
            let properties = read_properties(&mut cursor, &strings)?;
            elements.push(KirElement {
                id,
                kind,
                layer: layer[0],
                properties,
            });
        }

        if cursor.position() != bytes.len() as u64 {
            return Err(KirError::Binary(
                "trailing bytes after document".to_string(),
            ));
        }

        let document = KirDocument { metadata, elements };
        document.validate_persisted()?;
        Ok(document)
    }
}

struct StringTable {
    strings: Vec<String>,
    by_value: HashMap<String, u32>,
}

impl StringTable {
    fn from_document(document: &KirDocument) -> Result<Self, KirError> {
        let mut table = Self {
            strings: Vec::new(),
            by_value: HashMap::new(),
        };
        table.intern(KIR_SCHEMA_VERSION_METADATA_KEY)?;
        table.intern(KIR_SCHEMA_VERSION)?;
        collect_property_strings(&document.metadata, &mut table)?;
        for element in &document.elements {
            table.intern(&element.id)?;
            table.intern(&element.kind)?;
            collect_property_strings(&element.properties, &mut table)?;
        }
        Ok(table)
    }

    fn intern(&mut self, value: &str) -> Result<(), KirError> {
        if self.by_value.contains_key(value) {
            return Ok(());
        }
        let id = u32::try_from(self.strings.len())
            .map_err(|_| KirError::Binary("too many strings for binary KIR".to_string()))?;
        self.strings.push(value.to_string());
        self.by_value.insert(value.to_string(), id);
        Ok(())
    }

    fn id(&self, value: &str) -> Result<u32, KirError> {
        self.by_value
            .get(value)
            .copied()
            .ok_or_else(|| KirError::Binary(format!("missing string table entry `{value}`")))
    }
}

fn collect_property_strings(
    properties: &BTreeMap<String, Value>,
    table: &mut StringTable,
) -> Result<(), KirError> {
    for (key, value) in properties {
        table.intern(key)?;
        collect_value_strings(value, table)?;
    }
    Ok(())
}

fn collect_value_strings(value: &Value, table: &mut StringTable) -> Result<(), KirError> {
    match value {
        Value::String(value) => table.intern(value),
        Value::Array(items) => {
            for item in items {
                collect_value_strings(item, table)?;
            }
            Ok(())
        }
        Value::Object(object) => {
            for (key, value) in object {
                table.intern(key)?;
                collect_value_strings(value, table)?;
            }
            Ok(())
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => Ok(()),
    }
}

fn write_string_table(out: &mut Vec<u8>, table: &StringTable) -> Result<(), KirError> {
    write_u32(out, table.strings.len())?;
    for value in &table.strings {
        write_u32(out, value.len())?;
        out.write_all(value.as_bytes())?;
    }
    Ok(())
}

fn read_string_table(cursor: &mut Cursor<&[u8]>) -> Result<Vec<String>, KirError> {
    let count = read_u32(cursor)? as usize;
    let mut strings = Vec::with_capacity(count);
    for _ in 0..count {
        let len = read_u32(cursor)? as usize;
        let mut bytes = vec![0_u8; len];
        cursor.read_exact(&mut bytes)?;
        let value = String::from_utf8(bytes)
            .map_err(|err| KirError::Binary(format!("invalid UTF-8 string: {err}")))?;
        strings.push(value);
    }
    Ok(strings)
}

fn write_properties(
    out: &mut Vec<u8>,
    properties: &BTreeMap<String, Value>,
    table: &StringTable,
) -> Result<(), KirError> {
    write_u32(out, properties.len())?;
    for (key, value) in properties {
        write_string_id(out, table, key)?;
        write_value(out, value, table)?;
    }
    Ok(())
}

fn read_properties(
    cursor: &mut Cursor<&[u8]>,
    strings: &[String],
) -> Result<BTreeMap<String, Value>, KirError> {
    let count = read_u32(cursor)? as usize;
    let mut properties = BTreeMap::new();
    for _ in 0..count {
        let key = read_string_id(cursor, strings)?;
        let value = read_value(cursor, strings)?;
        properties.insert(key, value);
    }
    Ok(properties)
}

fn write_value(out: &mut Vec<u8>, value: &Value, table: &StringTable) -> Result<(), KirError> {
    match value {
        Value::Null => out.write_all(&[TAG_NULL])?,
        Value::Bool(value) => {
            out.write_all(&[TAG_BOOL])?;
            out.write_all(&[u8::from(*value)])?;
        }
        Value::Number(value) => {
            if let Some(value) = value.as_i64() {
                out.write_all(&[TAG_I64])?;
                out.write_all(&value.to_le_bytes())?;
            } else if let Some(value) = value.as_u64() {
                out.write_all(&[TAG_U64])?;
                out.write_all(&value.to_le_bytes())?;
            } else if let Some(value) = value.as_f64() {
                out.write_all(&[TAG_F64])?;
                out.write_all(&value.to_le_bytes())?;
            } else {
                return Err(KirError::Binary("unsupported JSON number".to_string()));
            }
        }
        Value::String(value) => {
            out.write_all(&[TAG_STRING])?;
            write_string_id(out, table, value)?;
        }
        Value::Array(items) => {
            out.write_all(&[TAG_ARRAY])?;
            write_u32(out, items.len())?;
            for item in items {
                write_value(out, item, table)?;
            }
        }
        Value::Object(object) => {
            out.write_all(&[TAG_OBJECT])?;
            write_u32(out, object.len())?;
            for (key, value) in object {
                write_string_id(out, table, key)?;
                write_value(out, value, table)?;
            }
        }
    }
    Ok(())
}

fn read_value(cursor: &mut Cursor<&[u8]>, strings: &[String]) -> Result<Value, KirError> {
    let mut tag = [0_u8; 1];
    cursor.read_exact(&mut tag)?;
    match tag[0] {
        TAG_NULL => Ok(Value::Null),
        TAG_BOOL => {
            let mut value = [0_u8; 1];
            cursor.read_exact(&mut value)?;
            Ok(Value::Bool(value[0] != 0))
        }
        TAG_I64 => Ok(Value::Number(Number::from(read_i64(cursor)?))),
        TAG_U64 => Ok(Value::Number(Number::from(read_u64(cursor)?))),
        TAG_F64 => Number::from_f64(read_f64(cursor)?)
            .map(Value::Number)
            .ok_or_else(|| KirError::Binary("invalid floating point number".to_string())),
        TAG_STRING => Ok(Value::String(read_string_id(cursor, strings)?)),
        TAG_ARRAY => {
            let count = read_u32(cursor)? as usize;
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                items.push(read_value(cursor, strings)?);
            }
            Ok(Value::Array(items))
        }
        TAG_OBJECT => {
            let count = read_u32(cursor)? as usize;
            let mut object = Map::new();
            for _ in 0..count {
                let key = read_string_id(cursor, strings)?;
                let value = read_value(cursor, strings)?;
                object.insert(key, value);
            }
            Ok(Value::Object(object))
        }
        other => Err(KirError::Binary(format!("unknown value tag {other}"))),
    }
}

fn write_string_id(out: &mut Vec<u8>, table: &StringTable, value: &str) -> Result<(), KirError> {
    write_u32_raw(out, table.id(value)?)
}

fn read_string_id(cursor: &mut Cursor<&[u8]>, strings: &[String]) -> Result<String, KirError> {
    let id = read_u32(cursor)? as usize;
    strings
        .get(id)
        .cloned()
        .ok_or_else(|| KirError::Binary(format!("invalid string table id {id}")))
}

fn write_u16(out: &mut Vec<u8>, value: u16) -> Result<(), KirError> {
    out.write_all(&value.to_le_bytes())?;
    Ok(())
}

fn write_u32(out: &mut Vec<u8>, value: usize) -> Result<(), KirError> {
    let value = u32::try_from(value)
        .map_err(|_| KirError::Binary("value exceeds u32 binary KIR limit".to_string()))?;
    write_u32_raw(out, value)
}

fn write_u32_raw(out: &mut Vec<u8>, value: u32) -> Result<(), KirError> {
    out.write_all(&value.to_le_bytes())?;
    Ok(())
}

fn read_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16, KirError> {
    let mut bytes = [0_u8; 2];
    cursor.read_exact(&mut bytes)?;
    Ok(u16::from_le_bytes(bytes))
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, KirError> {
    let mut bytes = [0_u8; 4];
    cursor.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

fn read_i64(cursor: &mut Cursor<&[u8]>) -> Result<i64, KirError> {
    let mut bytes = [0_u8; 8];
    cursor.read_exact(&mut bytes)?;
    Ok(i64::from_le_bytes(bytes))
}

fn read_u64(cursor: &mut Cursor<&[u8]>) -> Result<u64, KirError> {
    let mut bytes = [0_u8; 8];
    cursor.read_exact(&mut bytes)?;
    Ok(u64::from_le_bytes(bytes))
}

fn read_f64(cursor: &mut Cursor<&[u8]>) -> Result<f64, KirError> {
    let mut bytes = [0_u8; 8];
    cursor.read_exact(&mut bytes)?;
    Ok(f64::from_le_bytes(bytes))
}

fn stable_bytes_digest(bytes: &[u8]) -> String {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x100000001b3;

    let mut hash = OFFSET;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;

    use serde_json::json;

    use crate::{KIR_SCHEMA_VERSION, KirDocument, KirElement};

    #[test]
    fn binary_kir_round_trips_document() {
        let document = KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: vec![
                KirElement {
                    id: "pkg.Demo".to_string(),
                    kind: "model.Package".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("qualified_name".to_string(), json!("Demo")),
                        ("members".to_string(), json!(["type.Demo.Vehicle"])),
                    ]),
                },
                KirElement {
                    id: "type.Demo.Vehicle".to_string(),
                    kind: "model.Type".to_string(),
                    layer: 2,
                    properties: BTreeMap::from([
                        ("qualified_name".to_string(), json!("Demo.Vehicle")),
                        ("declared_name".to_string(), json!("Vehicle")),
                        ("owner".to_string(), json!("pkg.Demo")),
                        ("is_abstract".to_string(), json!(false)),
                        ("metadata".to_string(), json!({"source_file": "demo.toy"})),
                    ]),
                },
            ],
        };

        let bytes = document.to_binary_bytes().unwrap();
        assert!(bytes.starts_with(b"MKIR"));
        let round_trip = KirDocument::from_binary_bytes(&bytes).unwrap();

        assert_eq!(round_trip, document.normalized_for_persistence());
    }

    #[test]
    fn binary_kir_rejects_wrong_magic() {
        let error = KirDocument::from_binary_bytes(b"NOPE").unwrap_err();
        assert!(error.to_string().contains("invalid magic header"));
    }

    #[test]
    fn binary_cache_manifest_validates_source_and_binary_bytes() {
        let document = KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: vec![KirElement {
                id: "type.Demo.A".to_string(),
                kind: "model.Type".to_string(),
                layer: 2,
                properties: BTreeMap::from([("qualified_name".to_string(), json!("Demo.A"))]),
            }],
        };
        let source_bytes = br#"{"demo":"source"}"#;
        let binary_bytes = document.to_binary_bytes().unwrap();
        let manifest = super::BinaryKirCacheManifest::for_bytes(source_bytes, &binary_bytes);

        assert!(manifest.is_valid_for_bytes(source_bytes, &binary_bytes));
        assert!(!manifest.is_valid_for_bytes(br#"{"demo":"changed"}"#, &binary_bytes));
        assert!(!manifest.is_valid_for_bytes(source_bytes, b"changed"));
    }

    #[test]
    fn binary_cache_load_returns_none_for_stale_manifest() {
        let temp_dir = std::env::temp_dir().join(format!(
            "mercurio_binary_cache_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp_dir).unwrap();
        let binary_path = temp_dir.join("model.mkir");
        let manifest_path = temp_dir.join("model.mkir.json");
        let source_bytes = br#"{"demo":"source"}"#;
        let document = KirDocument {
            metadata: BTreeMap::from([(
                "kir_schema_version".to_string(),
                json!(KIR_SCHEMA_VERSION),
            )]),
            elements: vec![KirElement {
                id: "type.Demo.A".to_string(),
                kind: "model.Type".to_string(),
                layer: 2,
                properties: BTreeMap::from([("qualified_name".to_string(), json!("Demo.A"))]),
            }],
        };

        document
            .write_binary_cache_to_paths(&binary_path, &manifest_path, source_bytes)
            .unwrap();
        let loaded =
            KirDocument::from_valid_binary_cache_paths(&binary_path, &manifest_path, source_bytes)
                .unwrap()
                .unwrap();
        assert_eq!(loaded, document.normalized_for_persistence());
        assert!(
            KirDocument::from_valid_binary_cache_paths(
                &binary_path,
                &manifest_path,
                br#"{"demo":"changed"}"#,
            )
            .unwrap()
            .is_none()
        );

        let _ = fs::remove_dir_all(temp_dir);
    }
}

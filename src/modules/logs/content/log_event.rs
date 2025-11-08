use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::logs::content::LogEventContent;

/// An entry from a [LogFile](super::super::LogFile). Most of the content can be found in the
/// [LogEventContent].
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct LogEvent {
    pub timestamp: DateTime<Utc>,

    #[serde(flatten)]
    pub content: LogEventContent,
}

#[cfg(feature = "schemars")]
#[cfg(test)]
mod integration_test {
    use std::{env::temp_dir, fs::File};

    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct Noop {}

    use super::LogEvent;

    /// Schemars will generate a JSON Schema that contains a oneOf List of element that contain both a $ref and inlined properties.
    /// This causes code-generators to panic. A work-around is to convert this list of oneOfs into a list of oneOfs that themselves contain a list of anyOfs.
    /// In this anyOf, we reference the $ref in a single block, and inlined properties in another.
    pub fn fix_oneof_refs(schema: &mut Value) {
        match schema {
            Value::Object(map) => {
                if let Some(Value::Array(items)) = map.get_mut("oneOf") {
                    for item in items {
                        if let Value::Object(obj) = item {
                            if let Some(Value::String(ref_ref)) = obj.get("$ref") {
                                // If there are other keys besides $ref
                                let other_keys: Vec<_> =
                                    obj.keys().filter(|k| *k != "$ref").cloned().collect();

                                if !other_keys.is_empty() {
                                    let mut extras = serde_json::Map::new();
                                    for key in other_keys {
                                        if let Some(v) = obj.get(&key) {
                                            extras.insert(key, v.clone());
                                        }
                                    }

                                    let new_obj = json!({
                                        "allOf": [
                                            { "$ref": ref_ref },
                                            Value::Object(extras)
                                        ]
                                    });

                                    *item = new_obj;
                                }
                            }
                        }
                    }
                }

                // Recurse into all properties
                for value in map.values_mut() {
                    fix_oneof_refs(value);
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    fix_oneof_refs(item);
                }
            }
            _ => {}
        }
    }

    /// A manual "test" that will output the schema into tmp dir.
    #[test]
    #[ignore]
    fn generate_schema() {
        let dir = temp_dir();
        let mut schema = schemars::schema_for!(LogEvent).to_value();
        fix_oneof_refs(&mut schema);

        eprintln!("path: {}", dir.display());

        let file = File::create(dir.join("schema.json")).unwrap();
        serde_json::to_writer_pretty(file, &schema).unwrap();
    }
}

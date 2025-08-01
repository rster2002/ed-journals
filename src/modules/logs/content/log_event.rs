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
mod test {
    use std::{env::temp_dir, fs::File};

    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    use crate::logs::{
        buy_micro_resource_event::BuyMicroResourceEvent, cargo_event::CargoEvent,
        ship_targeted_event::ShipTargetedEvent, LogEventContent,
    };

    #[derive(Serialize, Deserialize, JsonSchema)]
    struct Noop {}

    use super::LogEvent;

    pub fn fix_oneof_refs(schema: &mut Value) {
        match schema {
            Value::Object(map) => {
                // Handle oneOf transformation
                if let Some(one_of) = map.get_mut("oneOf") {
                    if let Value::Array(items) = one_of {
                        for i in 0..items.len() {
                            if let Value::Object(obj) = &items[i] {
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

                                        // Create allOf structure
                                        let new_obj = json!({
                                            "allOf": [
                                                { "$ref": ref_ref },
                                                Value::Object(extras)
                                            ]
                                        });

                                        items[i] = new_obj;
                                    }
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

    #[test]
    fn misc() {
        let mut dir = temp_dir();
        let mut schema = schemars::schema_for!(LogEvent).to_value();
        fix_oneof_refs(&mut schema);

        eprintln!("path: {}", dir.display());

        let file = File::create(dir.join(format!("out.json"))).unwrap();
        serde_json::to_writer_pretty(file, &schema).unwrap();
    }
}

use crate::Result;
use serde_json::{Map, Value};

/// Performs deep merge of JSON values
///
/// The `new_value` takes precedence over `existing_value` for conflicts.
/// Objects are merged recursively, arrays and primitives are replaced.
pub fn deep_merge_json(existing: &mut Value, new: Value) -> Result<()> {
    match (existing, new) {
        (Value::Object(existing_map), Value::Object(new_map)) => {
            merge_objects(existing_map, new_map)?;
        }
        (existing, new) => {
            // Replace existing value with new value
            *existing = new;
        }
    }
    Ok(())
}

/// Recursively merges two JSON objects
fn merge_objects(existing: &mut Map<String, Value>, new: Map<String, Value>) -> Result<()> {
    for (key, new_value) in new {
        match existing.get_mut(&key) {
            Some(existing_value) => {
                // Recursively merge if both are objects
                deep_merge_json(existing_value, new_value)?;
            }
            None => {
                // Insert new key-value pair
                existing.insert(key, new_value);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_deep_merge_nested_objects() {
        let mut existing = json!({
            "a": {
                "b": 1,
                "c": 2
            },
            "d": 3
        });

        let new = json!({
            "a": {
                "b": 10,
                "e": 5
            },
            "f": 6
        });

        deep_merge_json(&mut existing, new).unwrap();

        let expected = json!({
            "a": {
                "b": 10,
                "c": 2,
                "e": 5
            },
            "d": 3,
            "f": 6
        });

        assert_eq!(existing, expected);
    }

    #[test]
    fn test_deep_merge_complex() {
        let mut base = json!({
            "config_version": 1,
            "settings": {
                "retries": 3,
                "timeout_ms": 1000
            },
            "features": {
                "feature_a": true
            }
        });

        let update = json!({
            "config_version": 2,
            "settings": {
                "timeout_ms": 5000,
                "new_setting": "value"
            },
            "features": {
                "feature_b": true
            },
            "new_section": {
                "key": "value"
            }
        });

        deep_merge_json(&mut base, update).unwrap();

        let expected = json!({
            "config_version": 2,
            "settings": {
                "retries": 3,
                "timeout_ms": 5000,
                "new_setting": "value"
            },
            "features": {
                "feature_a": true,
                "feature_b": true
            },
            "new_section": {
                "key": "value"
            }
        });

        assert_eq!(base, expected);
    }

    #[test]
    fn test_merge_replace_primitives() {
        let mut existing = json!({
            "string": "old",
            "number": 42,
            "boolean": true
        });

        let new = json!({
            "string": "new",
            "number": 100,
            "boolean": false,
            "array": [1, 2, 3]
        });

        deep_merge_json(&mut existing, new).unwrap();

        let expected = json!({
            "string": "new",
            "number": 100,
            "boolean": false,
            "array": [1, 2, 3]
        });

        assert_eq!(existing, expected);
    }
}

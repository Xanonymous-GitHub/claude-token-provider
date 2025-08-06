use dirs;
use serde_json::{to_string_pretty, Value};
use std::fs;
use std::path::{Path, PathBuf};

use super::merger::deep_merge_json;
use crate::{Result, TokenProviderError};

/// Default configuration directory and file paths
const CONFIG_DIR: &str = ".claude";
const CONFIG_FILE: &str = "settings.json";

/// Gets the target configuration file path
pub fn get_config_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        TokenProviderError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    })?;

    Ok(home_dir.join(CONFIG_DIR).join(CONFIG_FILE))
}

/// Ensures the configuration directory exists
pub fn ensure_config_dir(config_path: &Path) -> Result<()> {
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

/// Reads existing configuration file if it exists and is valid JSON
pub fn read_existing_config(config_path: &Path) -> Result<Option<Value>> {
    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(config_path)?;

    // Try to parse as JSON
    match serde_json::from_str::<Value>(&content) {
        Ok(json) => Ok(Some(json)),
        Err(_) => {
            // File exists but is not valid JSON - will be overwritten
            eprintln!("Warning: Existing config file is not valid JSON and will be replaced");
            Ok(None)
        }
    }
}

/// Writes configuration to file with pretty formatting
pub fn write_config(config_path: &Path, config: &Value) -> Result<()> {
    let pretty_json = to_string_pretty(config)?;
    fs::write(config_path, pretty_json)?;
    Ok(())
}

/// Handles the complete file operation workflow
pub fn apply_config_update(new_config: Value) -> Result<()> {
    let config_path = get_config_path()?;

    // Ensure directory exists
    ensure_config_dir(&config_path)?;

    // Read existing configuration
    let final_config = match read_existing_config(&config_path)? {
        Some(mut existing) => {
            // Deep merge new config into existing
            deep_merge_json(&mut existing, new_config)?;
            existing
        }
        None => {
            // No existing config or invalid JSON - use new config directly
            new_config
        }
    };

    // Write the final configuration
    write_config(&config_path, &final_config)?;

    println!(
        "Configuration successfully updated at: {}",
        config_path.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_operations() {
        // Test with temporary file
        let mut temp_file = NamedTempFile::new().unwrap();
        let existing_config = json!({"version": 1, "data": "test"});

        writeln!(
            temp_file,
            "{}",
            serde_json::to_string_pretty(&existing_config).unwrap()
        )
        .unwrap();

        let read_config = read_existing_config(temp_file.path()).unwrap();
        assert_eq!(read_config, Some(existing_config));
    }

    #[test]
    fn test_corrupted_json_handling() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "not valid json").unwrap();

        let result = read_existing_config(temp_file.path()).unwrap();
        assert_eq!(result, None); // Should return None for invalid JSON
    }
}

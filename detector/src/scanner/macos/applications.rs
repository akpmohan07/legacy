use crate::scanner::{DiscoveredTool, ScanError, Scanner};
use std::fs;
use std::path::Path;

pub struct ApplicationsScanner;

impl Scanner for ApplicationsScanner {
    fn source_name(&self) -> &'static str {
        "application"
    }

    fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError> {
        let apps_dir = Path::new("/Applications");
        let mut results = Vec::new();

        for entry in fs::read_dir(apps_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("app") {
                continue;
            }

            let plist_path = path.join("Contents/Info.plist");
            let Ok(value) = plist::Value::from_file(&plist_path) else {
                continue;
            };

            let dict = value.as_dictionary();
            let identifier = dict
                .and_then(|d| d.get("CFBundleIdentifier"))
                .and_then(|v| v.as_string())
                .unwrap_or("unknown")
                .to_string();
            let version = dict
                .and_then(|d| d.get("CFBundleShortVersionString"))
                .and_then(|v| v.as_string())
                .map(|s| s.to_string());

            let name = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| identifier.clone());

            results.push(DiscoveredTool {
                identifier,
                name,
                version,
                path,
            });
        }

        Ok(results)
    }
}

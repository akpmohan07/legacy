use crate::scanner::{probe_path, DiscoveredTool, Probe, ScanError, Scanner};
use std::fs;
use std::path::Path;

const APPS_DIR: &str = "/Applications";

pub struct ApplicationsScanner;

impl Scanner for ApplicationsScanner {
    fn source_name(&self) -> &'static str {
        "application"
    }

    fn probe(&self) -> Probe {
        probe_path(Path::new(APPS_DIR))
    }

    fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError> {
        scan_dir(Path::new(APPS_DIR))
    }
}

fn scan_dir(apps_dir: &Path) -> Result<Vec<DiscoveredTool>, ScanError> {
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
            installed_at: None,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::temp_dir;

    fn write_app(dir: &Path, folder: &str, plist_body: &str) {
        let contents = dir.join(folder).join("Contents");
        fs::create_dir_all(&contents).unwrap();
        fs::write(contents.join("Info.plist"), plist_body).unwrap();
    }

    const GOOD_PLIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleIdentifier</key><string>com.test.app</string>
<key>CFBundleShortVersionString</key><string>1.2.3</string>
</dict></plist>"#;

    #[test]
    fn reads_id_and_version_and_skips_broken_bundles_and_non_apps() {
        let dir = temp_dir("apps");
        write_app(&dir, "Good.app", GOOD_PLIST);
        write_app(&dir, "Broken.app", "not a plist");
        fs::write(dir.join("notes.txt"), "ignore me").unwrap();

        let found = scan_dir(&dir).unwrap();

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].identifier, "com.test.app");
        assert_eq!(found[0].name, "Good");
        assert_eq!(found[0].version.as_deref(), Some("1.2.3"));
        assert_eq!(found[0].installed_at, None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn an_unreadable_apps_folder_is_an_error_not_an_empty_success() {
        let dir = temp_dir("apps-missing");
        assert!(scan_dir(&dir.join("does-not-exist")).is_err());
        let _ = fs::remove_dir_all(&dir);
    }
}

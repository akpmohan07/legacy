use crate::scanner::{DiscoveredTool, Probe, ScanError, Scanner, Trigger};
use serde::Deserialize;
use std::fs;
use std::path::{Component, Path, PathBuf};

const PREFIXES: [&str; 2] = ["/opt/homebrew", "/usr/local"];

pub struct HomebrewCellarScanner;

impl Scanner for HomebrewCellarScanner {
    fn source_name(&self) -> &'static str {
        "homebrew-cellar"
    }

    fn probe(&self) -> Probe {
        match find_cellar() {
            Ok(Some(_)) => Probe::Available { version: None },
            Ok(None) => Probe::NotPresent,
            Err(err) => Probe::Failed(err.into()),
        }
    }

    fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError> {
        let cellar = find_cellar()?.ok_or(ScanError::SourceMissing)?;
        scan_cellar(&cellar)
    }

    /// Both candidate Cellars; the watcher watches whichever exist.
    fn trigger(&self) -> Trigger {
        Trigger::Paths(PREFIXES.iter().map(|prefix| Path::new(prefix).join("Cellar")).collect())
    }

    fn is_relevant(&self, relative: &Path) -> bool {
        is_relevant_in_cellar(relative)
    }
}

/// Only what the scan reads: a formula folder, a version folder (an upgrade adds one), or a
/// version's `INSTALL_RECEIPT.json` (written when an install finishes). The files a bottle
/// unpacks inside a version folder are noise.
fn is_relevant_in_cellar(relative: &Path) -> bool {
    let parts: Vec<&str> = relative
        .components()
        .filter_map(|part| match part {
            Component::Normal(name) => name.to_str(),
            _ => None,
        })
        .collect();
    matches!(parts.as_slice(), [_] | [_, _] | [_, _, "INSTALL_RECEIPT.json"])
}

fn find_cellar() -> Result<Option<PathBuf>, std::io::Error> {
    for prefix in PREFIXES {
        let cellar = Path::new(prefix).join("Cellar");
        if cellar.try_exists()? {
            return Ok(Some(cellar));
        }
    }
    Ok(None)
}

fn scan_cellar(cellar: &Path) -> Result<Vec<DiscoveredTool>, ScanError> {
    let mut results = Vec::new();

    for formula_entry in fs::read_dir(cellar)?.flatten() {
        let formula_path = formula_entry.path();
        if !formula_path.is_dir() {
            continue;
        }
        let formula = formula_entry.file_name().to_string_lossy().to_string();

        let Ok(version_dirs) = fs::read_dir(&formula_path) else {
            continue;
        };

        for version_entry in version_dirs.flatten() {
            let version_path = version_entry.path();
            if !version_path.is_dir() {
                continue;
            }

            if let Some(tool) = discovered_tool(&formula, &version_entry, &version_path) {
                results.push(tool);
            }
        }
    }

    Ok(results)
}

fn discovered_tool(
    formula: &str,
    version_entry: &fs::DirEntry,
    version_path: &Path,
) -> Option<DiscoveredTool> {
    let receipt_path = version_path.join("INSTALL_RECEIPT.json");
    let data = fs::read(&receipt_path).ok()?;
    let receipt: InstallReceipt = serde_json::from_slice(&data).ok()?;

    // Only what the user actually asked for, not every transitive dependency Homebrew pulled in.
    if !receipt.installed_on_request {
        return None;
    }

    let tap = receipt.source.and_then(|s| s.tap);
    let identifier = match tap.as_deref() {
        Some(tap) if tap != "homebrew/core" => format!("{tap}/{formula}"),
        _ => formula.to_string(),
    };
    let version = version_entry.file_name().to_string_lossy().to_string();

    Some(DiscoveredTool {
        identifier,
        name: formula.to_string(),
        version: Some(version),
        path: version_path.to_path_buf(),
        installed_at: receipt.time.map(|seconds| seconds as i64),
    })
}

#[derive(Deserialize)]
struct InstallReceipt {
    installed_on_request: bool,
    time: Option<f64>,
    source: Option<ReceiptSource>,
}

#[derive(Deserialize)]
struct ReceiptSource {
    tap: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::temp_dir;

    fn write_receipt(cellar: &Path, formula: &str, version: &str, json: &str) {
        let dir = cellar.join(formula).join(version);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("INSTALL_RECEIPT.json"), json).unwrap();
    }

    #[test]
    fn keeps_only_requested_formulae_and_prefixes_non_core_taps() {
        let cellar = temp_dir("cellar");
        write_receipt(
            &cellar,
            "btop",
            "1.4.6",
            r#"{"installed_on_request":true,"time":1775808467,"source":{"tap":"homebrew/core"}}"#,
        );
        write_receipt(
            &cellar,
            "aom",
            "3.13.1",
            r#"{"installed_on_request":false,"time":1761137501,"source":{"tap":"homebrew/core"}}"#,
        );
        write_receipt(
            &cellar,
            "zsh-ai",
            "0.10.6",
            r#"{"installed_on_request":true,"source":{"tap":"matheusml/zsh-ai"}}"#,
        );
        fs::create_dir_all(cellar.join("no-receipt").join("1.0")).unwrap();

        let mut found = scan_cellar(&cellar).unwrap();
        found.sort_by(|a, b| a.identifier.cmp(&b.identifier));

        assert_eq!(found.len(), 2);
        assert_eq!(found[0].identifier, "btop");
        assert_eq!(found[0].version.as_deref(), Some("1.4.6"));
        assert_eq!(found[0].installed_at, Some(1775808467));
        assert_eq!(found[1].identifier, "matheusml/zsh-ai/zsh-ai");
        assert_eq!(found[1].installed_at, None);
        let _ = fs::remove_dir_all(&cellar);
    }

    #[test]
    fn only_formula_and_version_folders_and_receipts_are_relevant_changes() {
        for relevant in ["gh", "gh/2.9.0", "gh/2.9.0/INSTALL_RECEIPT.json"] {
            assert!(is_relevant_in_cellar(Path::new(relevant)), "{relevant}");
        }
        for noise in ["gh/2.9.0/bin/gh", "gh/2.9.0/share/man/man1/gh.1", "gh/2.9.0/README.md", ""] {
            assert!(!is_relevant_in_cellar(Path::new(noise)), "{noise}");
        }
    }

    #[test]
    fn an_unreadable_cellar_is_an_error_not_an_empty_success() {
        let dir = temp_dir("cellar-missing");
        assert!(scan_cellar(&dir.join("does-not-exist")).is_err());
        let _ = fs::remove_dir_all(&dir);
    }
}

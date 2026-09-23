use crate::scanner::{DiscoveredTool, ScanError, Scanner};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub struct HomebrewCellarScanner;

impl Scanner for HomebrewCellarScanner {
    fn source_name(&self) -> &'static str {
        "homebrew-cellar"
    }

    fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError> {
        let mut results = Vec::new();

        let Some(cellar) = cellar_path() else {
            return Ok(results); // Homebrew not installed on this machine
        };

        let Ok(formula_dirs) = fs::read_dir(&cellar) else {
            return Ok(results);
        };

        for formula_entry in formula_dirs.flatten() {
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

                let Some(tool) = discovered_tool(&formula, &version_entry, &version_path) else {
                    continue;
                };
                results.push(tool);
            }
        }

        Ok(results)
    }
}

fn discovered_tool(
    formula: &str,
    version_entry: &fs::DirEntry,
    version_path: &Path,
) -> Option<DiscoveredTool> {
    let receipt_path = version_path.join("INSTALL_RECEIPT.json");
    let data = fs::read(&receipt_path).ok()?;
    let receipt: InstallReceipt = serde_json::from_slice(&data).ok()?;

    // Matches Legacy's "installed_on_request" philosophy: only what the user
    // actually asked for, not every transitive dependency Homebrew pulled in.
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
    })
}

fn cellar_path() -> Option<PathBuf> {
    for prefix in ["/opt/homebrew", "/usr/local"] {
        let cellar = Path::new(prefix).join("Cellar");
        if cellar.is_dir() {
            return Some(cellar);
        }
    }
    None
}

#[derive(Deserialize)]
struct InstallReceipt {
    installed_on_request: bool,
    source: Option<ReceiptSource>,
}

#[derive(Deserialize)]
struct ReceiptSource {
    tap: Option<String>,
}

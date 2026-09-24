use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
pub mod macos;

/// The one canonical shape every Scanner normalizes into,
/// regardless of what source or platform produced it.
pub struct DiscoveredTool {
    pub identifier: String,
    pub name: String,
    pub version: Option<String>,
    pub path: PathBuf,
    /// When the source says it was really installed (Unix seconds), if it knows.
    pub installed_at: Option<i64>,
}

#[derive(Debug)]
pub enum ScanError {
    Io(std::io::Error),
    /// `scan()` was called for a source that isn't on this machine.
    SourceMissing,
}

impl From<std::io::Error> for ScanError {
    fn from(err: std::io::Error) -> Self {
        ScanError::Io(err)
    }
}

/// What a source's discovery step found.
#[derive(Debug)]
pub enum Probe {
    Available { version: Option<String> },
    NotPresent,
    Failed(ScanError),
}

/// Whether a tool we failed to find is really gone.
#[derive(Debug, PartialEq, Eq)]
pub enum Absence {
    Gone,
    StillThere,
    Unknown,
}

/// What Legacy already knows about a tool that this scan did not find.
pub struct KnownTool<'a> {
    pub identifier: &'a str,
    pub path: Option<&'a Path>,
}

pub trait Scanner {
    /// Must match a `sources.name` row seeded by the `seed_sources` migration.
    fn source_name(&self) -> &'static str;

    /// Is this source on the machine? Cheap: no full scan.
    fn probe(&self) -> Probe;

    fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError>;

    /// Called only for tools the scan did not find. Only `Gone` may lead to an uninstall.
    fn confirm_absent(&self, tool: &KnownTool) -> Absence {
        absence_by_path(tool.path)
    }
}

/// `Available` if the path exists, `NotPresent` if it doesn't, `Failed` if we can't tell.
pub fn probe_path(path: &Path) -> Probe {
    match path.try_exists() {
        Ok(true) => Probe::Available { version: None },
        Ok(false) => Probe::NotPresent,
        Err(err) => Probe::Failed(err.into()),
    }
}

/// A missing path is `Gone`; an existing one is `StillThere`; an unreadable or unknown one is `Unknown`.
pub fn absence_by_path(path: Option<&Path>) -> Absence {
    match path.map(Path::try_exists) {
        Some(Ok(true)) => Absence::StillThere,
        Some(Ok(false)) => Absence::Gone,
        Some(Err(_)) | None => Absence::Unknown,
    }
}

pub struct SourceProbe<'a> {
    pub scanner: &'a dyn Scanner,
    pub probe: Probe,
}

pub struct ScannerRegistry {
    scanners: Vec<Box<dyn Scanner>>,
}

impl ScannerRegistry {
    pub fn build() -> Self {
        let mut scanners: Vec<Box<dyn Scanner>> = Vec::new();

        #[cfg(target_os = "macos")]
        {
            scanners.push(Box::new(macos::ApplicationsScanner));
            scanners.push(Box::new(macos::HomebrewCellarScanner));
        }

        Self { scanners }
    }

    /// Source discovery: ask every scanner whether its source is on this machine.
    pub fn discover(&self) -> Vec<SourceProbe<'_>> {
        self.scanners
            .iter()
            .map(|scanner| SourceProbe {
                scanner: scanner.as_ref(),
                probe: scanner.probe(),
            })
            .collect()
    }

    /// Each result is paired with the `sources.name` it came from,
    /// since that association only exists at the scanner level.
    pub fn scan_all(&self) -> Vec<(&'static str, DiscoveredTool)> {
        let mut results = Vec::new();

        for scanner in &self.scanners {
            match scanner.scan() {
                Ok(tools) => {
                    let source_name = scanner.source_name();
                    results.extend(tools.into_iter().map(|tool| (source_name, tool)));
                }
                Err(err) => eprintln!("scanner failed: {:?}", err),
            }
        }

        results
    }
}

#[cfg(test)]
pub(crate) fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("legacy-test-{}-{}", std::process::id(), name));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_path_distinguishes_present_from_missing() {
        let dir = temp_dir("probe");
        assert!(matches!(probe_path(&dir), Probe::Available { version: None }));
        assert!(matches!(probe_path(&dir.join("nope")), Probe::NotPresent));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn absence_is_gone_only_when_the_path_is_confirmed_missing() {
        let dir = temp_dir("absence");
        let existing = dir.join("here");
        std::fs::write(&existing, "x").unwrap();

        assert_eq!(absence_by_path(Some(&existing)), Absence::StillThere);
        assert_eq!(absence_by_path(Some(&dir.join("gone"))), Absence::Gone);
        assert_eq!(absence_by_path(None), Absence::Unknown);
        let _ = std::fs::remove_dir_all(&dir);
    }
}

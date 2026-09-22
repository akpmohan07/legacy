use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub mod macos;

/// The one canonical shape every Scanner normalizes into,
/// regardless of what source or platform produced it.
pub struct DiscoveredTool {
    pub identifier: String,
    pub name: String,
    pub version: Option<String>,
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum ScanError {
    Io(std::io::Error),
}

impl From<std::io::Error> for ScanError {
    fn from(err: std::io::Error) -> Self {
        ScanError::Io(err)
    }
}

pub trait Scanner {
    /// Must match a `sources.name` row seeded by the `seed_sources` migration.
    fn source_name(&self) -> &'static str;
    fn scan(&self) -> Result<Vec<DiscoveredTool>, ScanError>;
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
        }

        Self { scanners }
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

use legacy_detector::domain::TriggeredBy;
use legacy_detector::run;
use legacy_detector::scanner::ScannerRegistry;
use legacy_detector::store::Store;
use legacy_detector::watch::{self, WatchError};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

/// Warnings and errors go to stderr by default; set `LEGACY_LOG=debug` for the details.
fn init_logging() {
    let filter = EnvFilter::builder()
        .with_env_var("LEGACY_LOG")
        .with_default_directive(LevelFilter::WARN.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();
}

fn main() {
    init_logging();

    let command = std::env::args().nth(1);
    match command.as_deref() {
        None | Some("scan") => scan(),
        Some("watch") => watch_mode(),
        Some(other) => {
            eprintln!("unknown command {other:?}\nusage: legacy-detector [scan | watch]");
            std::process::exit(64);
        }
    }
}

/// One manual scan of every source, then exit.
fn scan() {
    let mut store = Store::open();
    let registry = ScannerRegistry::build();

    match run::run(&mut store, &registry, TriggeredBy::Manual) {
        Ok(report) => {
            println!("{}", report.summary());
            std::process::exit(report.exit_code());
        }
        Err(err) => {
            eprintln!("fatal: {err}");
            std::process::exit(2);
        }
    }
}

/// Stay running and scan sources as they change. Stop with Ctrl-C.
fn watch_mode() {
    let mut store = Store::open();
    let registry = ScannerRegistry::build();

    let err = match watch::watch(&mut store, &registry) {
        Ok(()) => return,
        Err(err) => err,
    };
    eprintln!("fatal: {err}");
    let code = match err {
        WatchError::Database(_) => 2,
        _ => 3,
    };
    std::process::exit(code);
}

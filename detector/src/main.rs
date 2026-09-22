mod models;
mod scanner;
mod schema;
mod store;

use scanner::ScannerRegistry;

fn main() {
    let registry = ScannerRegistry::build();
    let discovered = registry.scan_all();

    let mut conn = store::open_store();

    for (source_name, tool) in &discovered {
        println!(
            "{}  id={}  version={}",
            tool.name,
            tool.identifier,
            tool.version.as_deref().unwrap_or("unknown")
        );
        store::upsert_tool(&mut conn, source_name, tool);
    }

    println!("\n{} tools discovered and stored", discovered.len());
}

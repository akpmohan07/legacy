use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct DeviceAttributes {
    chip: Option<String>,
    memory: Option<String>,
    os_name: Option<String>,
    os_version: Option<String>,
}

pub struct ResolvedIdentity {
    pub platform_uuid: String,
    pub device_name: Option<String>,
    pub username: Option<String>,
    pub attributes_json: String,
}

/// Reads real hardware/OS facts via `system_profiler`, `whoami`, and `sysinfo`.
/// Never reads `serial_number`/`provisioning_UDID` — only `platform_UUID`,
/// the least support/warranty-identifying of the three fields SPHardwareDataType exposes.
pub fn resolve() -> ResolvedIdentity {
    let output = Command::new("system_profiler")
        .args(["-json", "SPHardwareDataType"])
        .output()
        .expect("failed to run system_profiler");

    let json: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("invalid system_profiler JSON");
    let hw = &json["SPHardwareDataType"][0];

    let platform_uuid = hw["platform_UUID"]
        .as_str()
        .expect("system_profiler did not report platform_UUID")
        .to_string();
    let device_name = hw["machine_name"].as_str().map(String::from);
    let chip = hw["chip_type"].as_str().map(String::from);
    let memory = hw["physical_memory"].as_str().map(String::from);

    let username = whoami::username().ok();
    let os_name = sysinfo::System::name();
    let os_version = sysinfo::System::os_version();

    let attributes_json = serde_json::to_string(&DeviceAttributes {
        chip,
        memory,
        os_name,
        os_version,
    })
    .expect("failed to serialize device attributes");

    ResolvedIdentity {
        platform_uuid,
        device_name,
        username,
        attributes_json,
    }
}

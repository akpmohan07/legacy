---
feature: local-identity
status: built
issues: [13]
decisions: [0002, 0007]
---

# Local identity

## Purpose
Record which machine and which account this store belongs to, once, so that exports can later carry a
label for it, plus a heartbeat showing when Legacy last ran here.

## Rules
- The table `local_identity` holds **exactly one row**. It is written on the first run and never
  re-resolved afterward.
- Fields: `platform_uuid` (unique), `device_name`, `username`, `attributes` (JSON: chip, memory,
  `os_name`, `os_version`), `first_seen_at`, and `last_seen_at`.
- `last_seen_at` is updated once at the end of every run.
- Sources of the values: `system_profiler -json SPHardwareDataType` (`platform_UUID`, `machine_name`,
  `chip_type`, `physical_memory`), the `whoami` crate (`username`), and `sysinfo` (OS name and version;
  it reports `Darwin` on macOS).

## Invariants
1. The serial number is never read. `platform_UUID` is the machine label.
2. The identity stays in the private local store. Nothing publishes it automatically.
3. There are no per-device or per-user columns anywhere else in the schema.

## Code map
`src/identity.rs` (`resolve`), `src/store/mod.rs` (`ensure_local_identity`, `touch_identity`).

## Guarded by
- test: run::tests::the_run_updates_the_installations_heartbeat
- Not covered by a test: `resolve` calls the operating system. It was verified live on Apple Silicon.

## Verify by hand
`sqlite3 ~/Library/Application\ Support/Legacy/legacy.db "SELECT device_name, username, attributes, last_seen_at FROM local_identity"`
returns one row, and `last_seen_at` moves forward on each run.

## Known limits
`identity.rs` is not gated per operating system, so on another OS it would fail at run time. Windows
would need the SMBIOS UUID (`Win32_ComputerSystemProduct.UUID`); that is unverified.

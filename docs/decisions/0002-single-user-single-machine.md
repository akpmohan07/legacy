---
id: 0002
title: Single user, single machine; one local_identity row
status: accepted
date: 2026-09-22
issues: [13]
supersedes: []
related: [0007]
---

## Context
An early design had `users`, `devices`, `user_devices`, and a per-device state table, to track several
machines in one store. The everyday case is one person on one machine, and each of those tables
existed to solve a problem (which machine? whose version?) that cannot occur with one machine.

## Decision
- The local store describes **exactly one machine and one user.** Tools, scans, and events have no
  device or user column.
- `local_identity` is a **single row**: `platform_uuid` (unique), `device_name`, `username`,
  `attributes` (JSON: chip, memory, OS name and version), `first_seen_at`, `last_seen_at`. It is
  resolved once on first run and never re-resolved. `last_seen_at` is the installation's heartbeat (0007).
- Values come from `system_profiler -json SPHardwareDataType` (`platform_UUID`, `machine_name`,
  `chip_type`, `physical_memory`), the `whoami` crate (`username`), and `sysinfo` (OS). Note that
  `sysinfo` reports the OS name as `Darwin` on macOS.
- **`platform_UUID`, not the serial number.** The serial identifies the machine to Apple support and
  is never read. The UUID is a stable label that survives an OS reinstall. It stays in the private
  store. If it is ever exported, hashing it is a publish-time concern that is not built.
- **Several machines are out of scope for the local store.** If ever needed, combining them is a
  publish or aggregation problem with its own schema. Auto-increment ids cannot be trusted across
  separate databases, so any merge would have to use natural keys.
- Peripherals (mouse, keyboard, monitors) are a different, future concept: a scanned inventory, not
  part of `local_identity`.

## Consequences
- Nothing needs a device filter. `tools.version` and `tools.path` are per machine by construction.
- Multi-device history would need a new design; nothing here forecloses it.
- A future Windows equivalent of `platform_UUID` would be the SMBIOS UUID
  (`Win32_ComputerSystemProduct.UUID`), not the registry `MachineGuid`, which changes on every OS
  install. This is from general knowledge and has not been verified here.

## Alternatives rejected
- **Relational `devices`, `users`, `user_devices`, and a per-device tool-state table:** overhead for a
  single user, and it made a version conflict between devices a problem we did not have.
- **A `users` table:** a username is not a safe identity key across machines.
- **A randomly generated local id in place of `platform_UUID`:** considered, but the store is private,
  so the real identifier is fine and avoids inventing a fake one.

## Evidence
- Code: `detector/src/identity.rs`, `detector/src/store/mod.rs` (`ensure_local_identity`, `touch_identity`)
- Migrations: `2026-09-22-145022_create_local_identity`, and `last_seen_at` in `2026-09-23-232014_add_scans_change_events_and_lifecycle`
- test: run::tests::the_run_updates_the_installations_heartbeat
- Not covered by a test: resolving the identity calls the operating system. It was verified live on an Apple Silicon Mac.

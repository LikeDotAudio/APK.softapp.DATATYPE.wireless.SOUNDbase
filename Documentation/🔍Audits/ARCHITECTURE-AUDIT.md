# 🔍 Architecture audit — DATATYPE.wireless.SOUNDbase (2026-10-30)

Audited against the APK.audio multi-repo architecture standard: coupling, boundaries, duplication, contracts.
← [Audits](INDEX.md) · Improvements: [🛣Roadmap/IMPROVEMENTS.md](../%F0%9F%9B%A3Roadmap/IMPROVEMENTS.md)

## Bounded context
It owns the Soundbase CSV frequency-report parser (`soundbase_importer.rs`, pure) and the `Worker-WIRELESS_SOUNDBASE` container that answers parse jobs on the bus. Rust: the parser plus the worker crate `SRC/` (`lib.rs` 228 lines), with a CSV/PDF fixture. 25 tracked files.
**What I sampled:** every tracked source file, `Docker/Dockerfile`, `Docker/docker-compose.yml` and `display.json`. I compared it against the sibling workers `DATATYPE.wireless.{CSV,IAS,SHUREwirelessWorkbench}` and `MANAGEMENT.DataMigration/mod.rs`.

## Context map
| Talks to | How (topic / endpoint / file / import) | Relationship |
|---|---|---|
| Any job client (the Data Migration frame) | sub `APK.audio/System/Protocols/wireless_soundbase/outgoing/job`, pub `…/incoming/result` (`schema: 1`) | Published Language (job envelope, versioned) |
| APK.plugin.SDK (`apk-plugin-runner`, `apkaudio-contracts`) | Cargo `tag = "sdk-v0.7"`, pinned in Cargo.lock; job loop `apk_plugin_runner::jobs::JobWorker` | Conformist, by version |
| DATATYPE.wireless.CSV `SRC/kernel` (`apkaudio-wireless-kernel`) | Cargo git rev (band rule, status word, job envelope) | Shared kernel, one-way |
| MANAGEMENT.DataMigration | depends on `SRC/importer` (`apkaudio-wireless-soundbase`) by git rev | Supplier (crate) |
| POD.workers aggregator | `include:`s `Docker/docker-compose.yml`; project `workers` | Pod aggregator (house pattern) |

## Scorecard
| Category | Score (0–3) | Finding |
|---|---|---|
| Coupling & Cohesion | 3 | Builds from its own repository: SDK by tag and pinned git revs in Cargo.lock, image context `SRC/`; no other repository's file compiled or copied |
| Boundary Integrity | 3 | Pure parser; MQTT only through the runner's `JobWorker`; versioned answer with a pinned shape |
| Duplication Control | 3 | Job loop is the runner's, envelope and band rule the kernel's |
| Testing & Contracts | 3 | `SRC/check.sh`: worker (incl. self test, topics, shape), parser + fixture tests; DataMigration's frame-header contract test |
| Change management | 2 | Worker 0.2.0, `schema: 1`; nothing tagged yet (tags are the owner's) |
Score key: 3 meets the standard · 2 minor gaps · 1 significant gaps · 0 absent / violated.

## Findings
1. **Good — no port, hardened container.** `cap_drop: [ALL]`, `read_only`, `no-new-privileges`, non-root uid 10001, runner health probe.
2. **Good — topics from the contract.** `job_and_result_topics_are_this_modules` asserts the job/result topic strings.
3. **Good — pure core, thin shell.** `handle_job` is three lines over the kernel; the parser publishes nothing.

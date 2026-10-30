# DATATYPE.wireless.SOUNDbase

> 📑 **Soundbase importer** — reads a Soundbase frequency report exported as CSV and publishes its wireless channels, one record per frequency, on the APK.audio bus.
>
> 📚 **Wiki:** [Documentation/README.md](Documentation/README.md) — principles, architecture, delivery.

---

## 🧩 Architectural Role
* **Family**: DATATYPE — data-format importers and parsers (`PODS/POD.THIN/Softapps/DATATYPE/`)
* **Category**: wireless coordination — source 2 of the four Data Migration importers (IAS, Soundbase, WWB, CSV)
* **Pipeline Stage**: DATA — a report in someone else's format becomes the channel record the Data Migration tab renders

---

## ⚙️ Repository Overview

This repo holds source files, not a crate. They are compiled by somebody else:

| File | What it is | Built by |
| :-- | :-- | :-- |
| `soundbase_importer.rs` | The CSV parser and its MQTT publisher | `#[path]` module `soundbase_importer` of the `apkaudio-data-migration` crate (`Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration/mod.rs`), which `oaRustCore` re-exports as `data_migration` |
| `test_soundbase_importer.rs` | A standalone runner over the fixture | `[[bin]] test_soundbase_importer` in that crate's `Cargo.toml` |
| `test_data/SB PDF.pdf` | A Soundbase PDF report (20 frequencies) | — kept as the source of the CSV; nothing here reads PDF |
| `test_data/SB PDF.csv` | The same report as CSV: `Group,Model,Band,Name,Preset,Spacing,Frequency` | the runner |
| `display.json` | Launcher manifest (category `DATATYPE`) | the OS app tree (`OS/system/project-apps.json`) |

### What the parser does (`soundbase_importer.rs`)

- `parse_soundbase_report(path)` reads the file; `parse_soundbase_report_str(text, name)` parses text already in hand.
- The CSV has a header row and is read **by column position**, not by header name: columns 0–6 are group, model, band, name, preset, spacing (kept as text) and frequency in MHz. A row with fewer than seven columns, or whose frequency does not parse as a number, is skipped.
- The result (`SoundbaseParseResult`: `status`, `source_format` = `Soundbase_CSV_Report`, `file_path`, `mqtt_topic`, `total_channels`, `channels[]`) is published **retained, QoS 1** to `APK.audio/System/DataMigration/Soundbase/ImportedData` — built by `apkaudio_contracts::topics::data_migration("Soundbase", "ImportedData")`. It connects to the loopback broker (`MQTT_LOOPBACK_HOST`, `MQTT_DEFAULT_PORT`) with the credentials from `apkaudio_contracts::broker_auth`, waits up to 3 s for the ack, and returns the result either way.
- Each channel record: `group`, `model`, `band`, `name`, `preset`, `spacing`, `frequency_mhz`. The GUI frame's table shows six of them (`name`, `group`, `model`, `band`, `frequency_mhz`, `spacing`).

---

## 🚀 Usage & Integration
* **Web Portal Entry**: Data Migration → *Soundbase* (GUI frame `WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/2_Soundbase/Soundbase_Importer.json`). The frame publishes the chosen file on `APK.audio/System/DataMigration/Soundbase/FilePath` and draws the table from `.../Soundbase/ImportedData`.
* **Who calls the parser**: the Data Migration daemon in the orchestrator (`NODE.BareMetal/SRC/Core/ORCHESTRATOR:hunter:VISA/src/services/data_migration.rs`, started from `main.rs`) subscribes `APK.audio/System/DataMigration/+/FilePath`. On a `/Soundbase/` topic a JSON payload with `raw_content` (or base64 `raw_blob`) goes to `parse_soundbase_report_str`; anything else is taken as a path on disk.
* **Remote Repository**: https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.SOUNDbase

### Build and check

```bash
cd PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration
cargo check --bins --lib          # compiles this parser and test_soundbase_importer
```

`cargo run --bin test_soundbase_importer` parses the fixture **and publishes the result, retained, to the local broker** — it is not a dry run. It reads the fixture from `../../DATATYPE/DATATYPE.wireless.SOUNDbase/test_data/SB PDF.csv` (until 2026-10-30 it looked under the DataMigration crate, where the crate move left nothing, and exited 1).

---

## 🧭 Philosophy

- **One record out, whatever came in.** Four foreign formats feed the Data Migration tab; each importer emits the fields its GUI frame declares as headers (the rule in the `MANAGEMENT.DataMigration/mod.rs` header). Here: `SoundbaseChannel` against `Soundbase_Importer.json`.
- **Topics come from the contract.** `soundbase_mqtt_topic()` builds the topic with `apkaudio_contracts::topics::data_migration` rather than a literal.
- **Publish and let go.** The parser publishes retained and returns; the bus holds the latest import for any tab that opens later. Nothing is stored in this repo.
- **The PDF is the source, the CSV is what is parsed.** Soundbase's PDF is kept beside its CSV in `test_data/`; PDF reading belongs to `DATATYPE.wireless.PDF`, not here.

---

## 🗺 Plans

None in this repo. Plans live in `Documentation/🛣Roadmap/roadmap.htm` of the APK.audio repo.

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 12:51 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->

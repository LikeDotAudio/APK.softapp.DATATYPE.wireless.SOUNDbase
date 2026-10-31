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

The parser is source compiled by somebody else; `SRC/` is the worker crate that runs it on its own:

| File | What it is | Built by |
| :-- | :-- | :-- |
| `soundbase_importer.rs` | The CSV parser (pure; it publishes nothing) | `#[path]` module `soundbase_importer` of the `apkaudio-data-migration` crate (`Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration/mod.rs`), which `oaRustCore` re-exports as `data_migration` |
| `test_soundbase_importer.rs` | A standalone runner over the fixture | `[[bin]] test_soundbase_importer` in that crate's `Cargo.toml` |
| `test_data/SB PDF.pdf` | A Soundbase PDF report (20 frequencies) | — kept as the source of the CSV; nothing here reads PDF |
| `test_data/SB PDF.csv` | The same report as CSV: `Group,Model,Band,Name,Preset,Spacing,Frequency` | the runner |
| `display.json` | Launcher manifest (category `DATATYPE`) | the OS app tree (`OS/system/project-apps.json`) |
| `SRC/` | The worker crate `apk-worker-wireless-soundbase`: `soundbase_importer.rs` (by `#[path]`) behind a job loop on `apk-plugin-runner`, with its `config.ini` (section `[wireless_soundbase]`) | `cargo test --manifest-path SRC/Cargo.toml`; the image `worker-wireless-soundbase:local` |
| `Docker/` | The worker stack: container `Worker-WIRELESS_SOUNDBASE`, project `workers`, no port | included by `PODS/POD.workers/Docker/docker-compose.yml` |

### What the parser does (`soundbase_importer.rs`)

- `parse_soundbase_report(path)` reads the file; `parse_soundbase_report_str(text, name)` parses text already in hand. Both go through `classify_soundbase_str`, which has no side effect.
- The CSV's header row is read **by name** (case-insensitive: `Group`, `Model`, `Band`, `Name`, `Preset`, `Spacing`, `Frequency` — also `Freq`, `MHz`), so reordered or extra columns map right. A header with no frequency column is an error; any other missing column leaves that field empty.
- A row whose frequency does not parse, or falls outside 100–3000 MHz (the sibling importers' guard), is skipped and counted in `skipped_rows`; a blank line is not counted.
- The result (`SoundbaseParseResult`: `status` — `success`, or `empty` when no channel was read — `source_format` = `Soundbase_CSV_Report`, `file_path`, `total_channels`, `skipped_rows`, `channels[]`) is returned. The `wireless_soundbase` worker (`SRC/src/lib.rs`) wraps it as `{job_id, ok, result, error}` and answers, not retained, on `APK.audio/System/Protocols/wireless_soundbase/incoming/result`.
- Each channel record: `group`, `model`, `band`, `name`, `preset`, `spacing`, `frequency_mhz`. The GUI frame's table shows six of them (`name`, `group`, `model`, `band`, `frequency_mhz`, `spacing`).

---

### As a worker (`Worker-WIRELESS_SOUNDBASE`, POD.workers)

The same parser answers jobs on the bus (LOOSE-workers-convert-rest). A client subscribes to
`APK.audio/System/Protocols/wireless_soundbase/incoming/result`, then publishes
`{"job_id": "<yours>", "filename": "SB PDF.csv", ...}` to `APK.audio/System/Protocols/wireless_soundbase/outgoing/job`
with the file as "text". The answer carries the same `job_id`, `ok`, and the parse result of
`classify_soundbase_str` under `result`; a malformed job or one over `max_job_bytes` (8 MiB, `SRC/config.ini`) comes
back `ok: false` with an `error`. Nothing is retained and the worker opens no port. Measured
2026-10-30: the fixture `test_data/SB PDF.csv` sent as a job came back `ok: true` with 20 channels.

```bash
docker compose -f PODS/POD.workers/Docker/docker-compose.yml up -d --build wireless-soundbase
cargo test --manifest-path SRC/Cargo.toml        # handle_job on the fixture, the self test
```

## 🚀 Usage & Integration
* **Web Portal Entry**: Data Migration → *Soundbase* (GUI frame `WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/2_Soundbase/Soundbase_Importer.json`). The frame posts the chosen file's text as a job to `APK.audio/System/Protocols/wireless_soundbase/outgoing/job` and draws the table from `.../wireless_soundbase/incoming/result`.
* **Who calls the parser**: the `wireless_soundbase` worker, on each job the frame posts. (The orchestrator's DataMigration inbox that used to call it is retired: LOOSE-datamigration-frames-to-workers.)
* **Remote Repository**: https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.SOUNDbase

### Build and check

```bash
cd PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration
cargo check --bins --lib          # compiles this parser and test_soundbase_importer
cargo test --bin test_soundbase_importer   # the parser's 4 unit tests
```

`cargo run --bin test_soundbase_importer` parses the fixture and prints it (20 channels); it publishes nothing. It reads the fixture from `../../DATATYPE/DATATYPE.wireless.SOUNDbase/test_data/SB PDF.csv` (until 2026-10-30 it looked under the DataMigration crate, where the crate move left nothing, and exited 1).

---

## 🧭 Philosophy

- **One record out, whatever came in.** Four foreign formats feed the Data Migration tab; each importer emits the fields its GUI frame declares as headers (the rule in the `MANAGEMENT.DataMigration/mod.rs` header). Here: `SoundbaseChannel` against `Soundbase_Importer.json`.
- **Parse, then let the daemon publish.** The parser returns; the data-migration daemon publishes retained on its own broker, and the bus holds the latest import for any tab that opens later. Nothing is stored in this repo.
- **The PDF is the source, the CSV is what is parsed.** Soundbase's PDF is kept beside its CSV in `test_data/`; PDF reading belongs to `DATATYPE.wireless.PDF`, not here.

---

## 🗺 Plans

None in this repo. Plans live in `Documentation/🛣Roadmap/roadmap.htm` of the APK.audio repo.

<!-- wiki:smoke -->
**Smoke test** — ✅ PASS 1/1 since 2026-10-30 12:51 EDT — [results](Documentation/%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)
<!-- /wiki:smoke -->

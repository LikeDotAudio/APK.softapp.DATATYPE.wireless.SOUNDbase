# Architecture — DATATYPE.wireless.SOUNDbase

← [Wiki home](../README.md)

## Using it

Export the Soundbase frequency report as CSV, then pick it in the Data Migration tab's *Soundbase* frame (`WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/2_Soundbase/Soundbase_Importer.json`). The browser reads it and posts it as a job, `{job_id, filename, text}`, not retained, to `APK.audio/System/Protocols/wireless_soundbase/outgoing/job`; the table draws the worker's answer on `…/wireless_soundbase/incoming/result`. No host path is ever sent.

## How it works

```text
…/wireless_soundbase/outgoing/job ─▶ Worker-WIRELESS_SOUNDBASE (SRC/src/lib.rs handle_job)
                          └─▶ soundbase_importer, on the job's text / content_b64
                                └─▶ classify_soundbase_str (pure)
                                      header row read by name: group model band name preset spacing frequency
                                      → SoundbaseChannel; a frequency that does not parse or is outside
                                        100–3000 MHz is skipped and counted
                          worker answers {job_id, ok, result, error}, not retained → …/wireless_soundbase/incoming/result
```

`SoundbaseParseResult { status ("success" | "empty"), source_format: "Soundbase_CSV_Report", file_path, mqtt_topic, total_channels, skipped_rows, channels }`. A header with no frequency column is an `Err`.

## Files

| Path | What |
|---|---|
| `soundbase_importer.rs` | parser (no side effect) and its unit tests; module `soundbase_importer` of `MANAGEMENT.DataMigration` (`mod.rs`, `#[path]`) |
| `test_soundbase_importer.rs` | `[[bin]] test_soundbase_importer`: parses `test_data/SB PDF.csv`, prints a sample; publishes nothing |
| `test_data/SB PDF.csv`, `SB PDF.pdf` | one Soundbase report (20 frequencies) as CSV, and the PDF it came from |

## Checking it

```bash
cd PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration
cargo check --bins --lib
cargo test --bin test_soundbase_importer   # 4 unit tests
```

## As a worker

`SRC/src/lib.rs` `handle_job` decodes a job (`text` or `content_b64`), refuses one over
`max_job_bytes` (8 MiB), runs `classify_soundbase_str` and returns `{job_id, tool, ok, result, error, elapsed_ms}`
— always, so a caller waiting on a `job_id` is never left waiting. `run_job_agent` feeds it from
`APK.audio/System/Protocols/wireless_soundbase/outgoing/job` and answers on `…/incoming/result`, neither
retained. `apk-plugin-runner` adds the retained `incoming/api` description, the config, the will and
the self test (`SRC/src/self_test.rs`: a probe file compiled into the binary runs through
`handle_job`). The pod is `PODS/POD.workers`.

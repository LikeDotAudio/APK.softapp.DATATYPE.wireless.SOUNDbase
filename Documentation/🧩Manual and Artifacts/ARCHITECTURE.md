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

`SoundbaseParseResult { status ("success" | "empty"), source_format: "Soundbase_CSV_Report", file_path, total_channels, skipped_rows, channels }`. A header with no frequency column is an `Err`.

## Files

| Path | What |
|---|---|
| `SRC/importer/` | the parser crate `apkaudio-wireless-soundbase` and its unit tests (no publisher), `tests/fixtures.rs` over `test_data/SB PDF.csv`; MANAGEMENT.DataMigration depends on it by git rev |
| `SRC/check.sh` | the one local check: `cargo test --workspace --locked` (+ pytest for the Python edge) |
| `test_data/SB PDF.csv`, `SB PDF.pdf` | one Soundbase report (20 frequencies) as CSV, and the PDF it came from |

## Testing

```bash
bash PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.SOUNDbase/SRC/check.sh
```

## As a worker

`SRC/src/lib.rs` `handle_job` decodes a job (`text` or `content_b64`), refuses one over
`max_job_bytes` (8 MiB), runs `classify_soundbase_str` and returns `{schema, job_id, tool, ok, result, error, elapsed_ms}` (the wireless kernel's `job` envelope)
— always, so a caller waiting on a `job_id` is never left waiting. `run_job_agent` (the runner's `JobWorker`: session, ConnAck resubscribe, lane thread, busy answer) feeds it from
`APK.audio/System/Protocols/wireless_soundbase/outgoing/job` and answers on `…/incoming/result`, neither
retained. `apk-plugin-runner` adds the retained `incoming/api` description, the config, the will and
the self test (`SRC/src/self_test.rs`: a probe file compiled into the binary runs through
`handle_job`). The pod is `PODS/POD.workers`.

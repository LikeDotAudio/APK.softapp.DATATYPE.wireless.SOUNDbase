# Architecture — DATATYPE.wireless.SOUNDbase

← [Wiki home](../README.md)

## Using it

Export the Soundbase frequency report as CSV, then pick it in the Data Migration tab's *Soundbase* frame (`WEB.Frontend-Assets/SRC/FrontEnd/Gui_Frames/3_Data_Migration/2_Soundbase/Soundbase_Importer.json`). The frame publishes it on `APK.audio/System/DataMigration/Soundbase/FilePath` (a disk path, `raw_content`, or base64 `raw_blob`) and draws the table from `…/Soundbase/ImportedData`.

## How it works

```text
…/Soundbase/FilePath ─▶ NODE.BareMetal data-migration daemon (ORCHESTRATOR:hunter:VISA/src/services/data_migration.rs)
                          └─▶ oaRustCore::data_migration::soundbase_importer::parse_soundbase_report[_str]
                                CSV with a header row; each row by position:
                                  0 group  1 model  2 band  3 name  4 preset  5 spacing  6 frequency (MHz)
                                → SoundbaseChannel; rows short of 7 columns or with no number are skipped
                                └─▶ publish retained, QoS 1 → APK.audio/System/DataMigration/Soundbase/ImportedData
```

`SoundbaseParseResult { status, source_format: "Soundbase_CSV_Report", file_path, mqtt_topic, total_channels, channels }`. Publishing waits up to 3 s for the ack on the loopback broker, with `apkaudio_contracts::broker_auth` credentials when set; no broker is not an error.

## Files

| Path | What |
|---|---|
| `soundbase_importer.rs` | parser and publisher; module `soundbase_importer` of `MANAGEMENT.DataMigration` (`mod.rs`, `#[path]`) |
| `test_soundbase_importer.rs` | `[[bin]] test_soundbase_importer`: parses `test_data/SB PDF.csv`, prints a sample — **and publishes retained** |
| `test_data/SB PDF.csv`, `SB PDF.pdf` | one Soundbase report (20 frequencies) as CSV, and the PDF it came from |

## Checking it

```bash
cd PODS/POD.THIN/Softapps/MANAGEMENT_AND_UTILITIES/MANAGEMENT.DataMigration
cargo check --bins --lib
```

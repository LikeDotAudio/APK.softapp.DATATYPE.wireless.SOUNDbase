# Principles — DATATYPE.wireless.SOUNDbase

The rules the Soundbase report importer keeps, each with the code that keeps it (in `SRC/importer/src/lib.rs` unless named). Ecosystem rules are cited, not copied: [Refusal over fabrication](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/Doctrine/readme.Refusal%20over%20fabrication.md).

← [Wiki home](../README.md)

| Principle | Kept by | Where it breaks |
|---|---|---|
| **One record out, whatever came in.** `SoundbaseChannel` carries what the GUI frame `Soundbase_Importer.json` shows. | `SoundbaseChannel` | The frame shows six of its seven fields (`preset` is not drawn) |
| **Do not invent a value.** Every field is copied from its column; a column the header lacks stays empty. | `classify_soundbase_str` | — |
| **Keep going on a bad row, and count it.** A row whose frequency does not parse or is outside 100–3000 MHz (the siblings' guard) is skipped and counted in `skipped_rows`. | `usable_mhz`, `classify_soundbase_str` | — |
| **Parse here, publish in the daemon.** The parser has no side effect; the data-migration daemon publishes the result retained on its own broker. | `classify_soundbase_str`; `ORCHESTRATOR:hunter:VISA/src/services/data_migration.rs` | — |
| **Read by name, not by position.** The header row says where each field is; no frequency column is an error. | `Columns::from_header` | — |
| **The PDF is the source; the CSV is what is parsed.** PDF reading belongs to `DATATYPE.wireless.PDF`. | `test_data/SB PDF.pdf` kept beside `SB PDF.csv` | — |
| **One parser, one owner.** The parser is the crate `apkaudio-wireless-soundbase`; the worker links it by path, MANAGEMENT.DataMigration by git rev. | `SRC/importer/Cargo.toml`; `SRC/Cargo.toml` | — (until 2026-10-30 DataMigration compiled this file by `#[path]` and built its runner from here) |
| **What the four wireless importers share is written once.** The band rule, the status word and the job envelope are the wireless kernel's; the job loop is the runner's. | `apkaudio_wireless_kernel::{usable_mhz…, import_status, job}`; `apk_plugin_runner::jobs::JobWorker` (`SRC/src/lib.rs`) | — (until 2026-10-30 the loop and envelope were copied line for line from the CSV worker) |
| **The answer is versioned and its shape pinned.** | `schema` (`job::SCHEMA`); test `the_result_shape_is_pinned` | the result is the parser's struct serialised, held by the shape test rather than a second DTO type |
| **The SDK by tag, patched to the checkout locally.** | `SRC/Cargo.toml` (`sdk-v0.5`, kernel by rev); git-excluded `SRC/.cargo/config.toml`; the Dockerfile's `--config patch…` | a standalone clone waits for an SDK tag carrying `jobs` and the pushed kernel rev; the image builds from the repo root until then |

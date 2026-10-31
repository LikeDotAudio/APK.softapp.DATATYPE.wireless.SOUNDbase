# Principles — DATATYPE.wireless.SOUNDbase

The rules the Soundbase report importer keeps, each with the code that keeps it (in `soundbase_importer.rs` unless named). Ecosystem rules are cited, not copied: [Refusal over fabrication](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/Doctrine/readme.Refusal%20over%20fabrication.md).

← [Wiki home](../README.md)

| Principle | Kept by | Where it breaks |
|---|---|---|
| **One record out, whatever came in.** `SoundbaseChannel` carries what the GUI frame `Soundbase_Importer.json` shows. | `SoundbaseChannel` | The frame shows six of its seven fields (`preset` is not drawn) |
| **Do not invent a value.** Every field is copied from its column; a column the header lacks stays empty. | `classify_soundbase_str` | — |
| **Keep going on a bad row, and count it.** A row whose frequency does not parse or is outside 100–3000 MHz (the siblings' guard) is skipped and counted in `skipped_rows`. | `usable_mhz`, `classify_soundbase_str` | — |
| **Parse here, publish in the daemon.** The parser has no side effect; the data-migration daemon publishes the result retained on its own broker. | `classify_soundbase_str`; `ORCHESTRATOR:hunter:VISA/src/services/data_migration.rs` | — |
| **Read by name, not by position.** The header row says where each field is; no frequency column is an error. | `Columns::from_header` | — |
| **The PDF is the source; the CSV is what is parsed.** PDF reading belongs to `DATATYPE.wireless.PDF`. | `test_data/SB PDF.pdf` kept beside `SB PDF.csv` | — |

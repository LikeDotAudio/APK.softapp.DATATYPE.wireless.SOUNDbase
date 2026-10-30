# Principles — DATATYPE.wireless.SOUNDbase

The rules the Soundbase report importer keeps, each with the code that keeps it (in `soundbase_importer.rs` unless named). Ecosystem rules are cited, not copied: [Refusal over fabrication](../../../../../../../Documentation/%F0%9F%8F%9BPhilosophy/Doctrine/readme.Refusal%20over%20fabrication.md).

← [Wiki home](../README.md)

| Principle | Kept by | Where it breaks |
|---|---|---|
| **One record out, whatever came in.** `SoundbaseChannel` carries what the GUI frame `Soundbase_Importer.json` shows. | `SoundbaseChannel` | The frame shows six of its seven fields (`preset` is not drawn) |
| **Do not invent a value.** Every field is copied from its column; nothing is defaulted. | `parse_soundbase_report_str` (line 52) | — |
| **Keep going on a bad row.** A row with fewer than seven columns, or a frequency that does not parse, is skipped. | `rec.len() >= 7` (line 61), `parse::<f64>()` (line 69) | Skipped rows are not counted or reported, and there is no 100–3000 MHz guard as the sibling importers have |
| **Topics come from the contract.** | `soundbase_mqtt_topic()` (line 38) | — |
| **The bus is the API.** The result is published retained, QoS 1. | `publish_to_mqtt` (line 100) | Parsing and publishing are one call: there is no side-effect-free parse to unit-test, and the repo has no unit tests |
| **Read by position, not by name.** Columns 0–6 are group, model, band, name, preset, spacing, frequency. | `parse_soundbase_report_str` | A Soundbase export with reordered or extra leading columns is misread silently |
| **The PDF is the source; the CSV is what is parsed.** PDF reading belongs to `DATATYPE.wireless.PDF`. | `test_data/SB PDF.pdf` kept beside `SB PDF.csv` | — |

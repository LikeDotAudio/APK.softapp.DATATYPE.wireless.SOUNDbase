// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! Standalone Soundbase Importer module written in Rust.
//! Parses Soundbase PDF exports / CSV data (e.g., SB PDF.csv),
//! maps Group, Model, Band, Name, Preset, Spacing, Frequency fields,
//! and returns them. It does not publish: the data-migration daemon publishes the
//! result on the broker it was started with (LOOSE-datatype-datatype-wireless).

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use apkaudio_contracts::topics;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundbaseChannel {
    pub group: String,
    pub model: String,
    pub band: String,
    pub name: String,
    pub preset: String,
    pub spacing: String,
    pub frequency_mhz: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SoundbaseParseResult {
    pub status: String,
    pub source_format: String,
    pub file_path: String,
    pub mqtt_topic: String,
    pub total_channels: usize,
    /// Rows the export carried that could not be read as a channel.
    pub skipped_rows: usize,
    pub channels: Vec<SoundbaseChannel>,
}

/// `APK.audio/System/DataMigration/Soundbase/ImportedData`, built from the contract
/// rather than typed out — see `contracts/rust/src/topics.rs`, whose own header
/// names the DataMigration importers as the literals it exists to replace.
pub fn soundbase_mqtt_topic() -> String {
    topics::data_migration("Soundbase", "ImportedData")
        .expect("Soundbase/ImportedData are literal segments the contract accepts")
}

/// Parses a Soundbase report / PDF CSV export into structured data. No side effect.
pub fn parse_soundbase_report(file_path: impl AsRef<Path>) -> Result<SoundbaseParseResult, String> {
    let path = file_path.as_ref();
    let file_content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
    parse_soundbase_report_str(&file_content, &path.to_string_lossy())
}

/// Parses a raw Soundbase CSV content string into structured data. No side effect.
pub fn parse_soundbase_report_str(file_content: &str, source_name: &str) -> Result<SoundbaseParseResult, String> {
    let mut channels = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(file_content.as_bytes());

    let mut skipped_rows = 0usize;
    for record in rdr.records() {
        let Ok(rec) = record else {
            skipped_rows += 1;
            continue;
        };
        if rec.iter().all(|f| f.trim().is_empty()) {
            continue; // a blank line is not a dropped row
        }
        let freq = if rec.len() >= 7 { rec[6].trim().parse::<f64>().ok() } else { None };
        let Some(freq) = freq else {
            skipped_rows += 1;
            continue;
        };
        channels.push(SoundbaseChannel {
            group: rec[0].trim().to_string(),
            model: rec[1].trim().to_string(),
            band: rec[2].trim().to_string(),
            name: rec[3].trim().to_string(),
            preset: rec[4].trim().to_string(),
            spacing: rec[5].trim().to_string(),
            frequency_mhz: freq,
        });
    }

    let result = SoundbaseParseResult {
        status: status_for(channels.len()),
        source_format: "Soundbase_CSV_Report".to_string(),
        file_path: source_name.to_string(),
        mqtt_topic: soundbase_mqtt_topic(),
        total_channels: channels.len(),
        skipped_rows,
        channels,
    };

    Ok(result)
}

/// "success" only when the report yielded channels; a parse that found none is
/// "empty", never a success the panel would draw as an imported, blank table.
fn status_for(channels: usize) -> String {
    if channels > 0 { "success" } else { "empty" }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_without_a_frequency_are_counted_and_no_channels_is_empty() {
        let csv = "Group,Model,Band,Name,Preset,Spacing,Frequency\nA,UR4D,G50,Vox,1,25,not-a-number\n,,,,,,\n";
        let r = parse_soundbase_report_str(csv, "x").unwrap();
        assert_eq!((r.status.as_str(), r.total_channels, r.skipped_rows), ("empty", 0, 1));
        let ok = parse_soundbase_report_str(include_str!("test_data/SB PDF.csv"), "SB PDF.csv").unwrap();
        assert!(ok.total_channels > 0);
        assert_eq!(ok.status, "success");
    }
}

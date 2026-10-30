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

/// Parses a raw Soundbase CSV content string into structured data. No side effect
/// (see `classify_soundbase_str`).
pub fn parse_soundbase_report_str(file_content: &str, source_name: &str) -> Result<SoundbaseParseResult, String> {
    classify_soundbase_str(file_content, source_name)
}

/// A usable wireless frequency, in MHz — the guard the IAS, WWB and CSV importers
/// keep. A value outside it is a spacing, a channel number or a typo, not a carrier.
fn usable_mhz(text: &str) -> Option<f64> {
    text.trim().parse::<f64>().ok().filter(|f| *f > 100.0 && *f < 3000.0)
}

/// Where each field sits in this export, read from its header row by name, so a
/// report whose columns are reordered (or carry an extra one) still maps right.
struct Columns {
    group: Option<usize>,
    model: Option<usize>,
    band: Option<usize>,
    name: Option<usize>,
    preset: Option<usize>,
    spacing: Option<usize>,
    frequency: usize,
}

impl Columns {
    fn from_header(header: &csv::StringRecord) -> Result<Self, String> {
        let names: Vec<String> = header.iter().map(|h| h.trim().to_ascii_lowercase()).collect();
        let find = |wanted: &[&str]| names.iter().position(|n| wanted.contains(&n.as_str()));
        let frequency = find(&["frequency", "frequency (mhz)", "freq", "freq (mhz)", "mhz"])
            .ok_or_else(|| format!("no Frequency column in the header {:?}", names))?;
        Ok(Columns {
            group: find(&["group"]),
            model: find(&["model"]),
            band: find(&["band"]),
            name: find(&["name"]),
            preset: find(&["preset"]),
            spacing: find(&["spacing"]),
            frequency,
        })
    }
}

/// Classify raw Soundbase CSV content: header-mapped columns, frequencies held to
/// 100–3000 MHz, every row that is not blank and not a channel counted in
/// `skipped_rows`. Pure — it reads the string and returns; it publishes nothing.
pub fn classify_soundbase_str(file_content: &str, source_name: &str) -> Result<SoundbaseParseResult, String> {
    let mut channels = Vec::new();
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(file_content.as_bytes());

    let header = rdr.headers().map_err(|e| format!("Soundbase header unreadable: {}", e))?.clone();
    let cols = Columns::from_header(&header)?;
    let cell = |rec: &csv::StringRecord, at: Option<usize>| {
        at.and_then(|i| rec.get(i)).map(|v| v.trim().to_string()).unwrap_or_default()
    };

    let mut skipped_rows = 0usize;
    for record in rdr.records() {
        let Ok(rec) = record else {
            skipped_rows += 1;
            continue;
        };
        if rec.iter().all(|f| f.trim().is_empty()) {
            continue; // a blank line is not a dropped row
        }
        let Some(freq) = rec.get(cols.frequency).and_then(usable_mhz) else {
            skipped_rows += 1;
            continue;
        };
        channels.push(SoundbaseChannel {
            group: cell(&rec, cols.group),
            model: cell(&rec, cols.model),
            band: cell(&rec, cols.band),
            name: cell(&rec, cols.name),
            preset: cell(&rec, cols.preset),
            spacing: cell(&rec, cols.spacing),
            frequency_mhz: freq,
        });
    }

    Ok(SoundbaseParseResult {
        status: status_for(channels.len()),
        source_format: "Soundbase_CSV_Report".to_string(),
        file_path: source_name.to_string(),
        mqtt_topic: soundbase_mqtt_topic(),
        total_channels: channels.len(),
        skipped_rows,
        channels,
    })
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
        assert_eq!((ok.channels[0].band.as_str(), ok.channels[0].frequency_mhz), ("G57 - Narrowband", 566.6));
    }

    #[test]
    fn columns_are_read_by_header_name_not_position() {
        let csv = "Frequency,Name,Group,Extra,Band\n566.600,Vox,IEM,x,G57\n";
        let r = classify_soundbase_str(csv, "x").unwrap();
        let ch = &r.channels[0];
        assert_eq!((ch.group.as_str(), ch.name.as_str(), ch.band.as_str(), ch.model.as_str()), ("IEM", "Vox", "G57", ""));
        assert_eq!(ch.frequency_mhz, 566.6);
    }

    #[test]
    fn a_frequency_outside_100_to_3000_mhz_is_skipped() {
        let csv = "Group,Model,Band,Name,Preset,Spacing,Frequency\nA,M,B,N,1,0.5,0.925\nA,M,B,N,1,0.5,5800\nA,M,B,N,1,0.5,566.6\n";
        let r = classify_soundbase_str(csv, "x").unwrap();
        assert_eq!((r.total_channels, r.skipped_rows), (1, 2));
    }

    #[test]
    fn a_report_with_no_frequency_column_is_an_error() {
        assert!(classify_soundbase_str("Group,Model\nA,B\n", "x").is_err());
    }
}

// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! Standalone Test Runner for Soundbase Importer & MQTT pipeline.

use std::path::Path;

mod soundbase_importer;

fn main() {
    let report_path = concat!(env!("CARGO_MANIFEST_DIR"), "/DataMigration/2_Soundbase/test_data/SB PDF.csv");
    println!("[Soundbase Test Runner] Testing standalone Soundbase Importer with file: {}", report_path);

    match soundbase_importer::parse_soundbase_report(Path::new(report_path)) {
        Ok(result) => {
            println!("\n=== SOUNDBASE PARSE SUCCESS ===");
            println!("Status: {}", result.status);
            println!("Source Format: {}", result.source_format);
            println!("MQTT Topic: {}", result.mqtt_topic);
            println!("Total Channels Extracted: {}", result.total_channels);
            
            println!("\n--- Extracted Soundbase Channels ---");
            for (idx, ch) in result.channels.iter().enumerate() {
                println!("[{:02}] Group: '{}' | Model: '{}' | Band: '{}' | Spacing: {} MHz | Freq: {:.3} MHz",
                    idx + 1, ch.group, ch.model, ch.band, ch.spacing, ch.frequency_mhz);
            }
            println!("=====================================");
        }
        Err(err) => {
            eprintln!("[Soundbase Test Runner] ERROR: {}", err);
            std::process::exit(1);
        }
    }
}

// Part of the APK.audio project — http://APK.audio — made by Anthony Kuzub
// MIT Licence. Free, for everyone, for ever. Full text in LICENSE at the root.
//! The worker's self test: the runner answers `outgoing/selftest_run` with it.
//!
//! All `Depth::Own` — a worker has no hardware. The decisive check runs a real
//! job through [`crate::handle_job`] on a report compiled into the binary, so
//! a pass means the parser in THIS image reads a Soundbase report.

use apkaudio_contracts::self_test::{Check, Depth, Report};

use crate::{CONFIG_INI, MODULE};

/// The Soundbase header and three rows, one of them out of band (skipped).
const PROBE_CSV: &str = "Group,Model,Band,Name,Preset,Spacing,Frequency
\
IEM,P10T,G10,Lead,1,25,470.125
\
IEM,P10T,G10,Spare,1,25,5800
\
RF,AD4Q,G57,Host,2,25,566.600
";

pub fn self_test(depth: Depth) -> Report {
    let mut report = Report::new(MODULE, depth);
    report.record(Check::from_result(
        "landing_spots_build",
        Depth::Own,
        "job, result and the self-test leaves address legal topics",
        landing_spots(),
    ));
    report.record(Check::from_result(
        "config_section_still_names_this_module",
        Depth::Own,
        "config.ini declares this module under the name the bus knows it by",
        config_declares_module(),
    ));
    report.record(Check::from_result(
        "a_job_round_trips_through_the_parser",
        Depth::Own,
        "a Soundbase CSV job comes back ok with 2 channels and 1 skipped row",
        probe_job(),
    ));
    report
}

fn landing_spots() -> Result<(), String> {
    use apkaudio_contracts::{baremetal_io, self_test as st};
    baremetal_io::module_job(MODULE).map_err(|e| format!("{e:?}"))?;
    baremetal_io::module_result(MODULE).map_err(|e| format!("{e:?}"))?;
    st::run_topic(MODULE).map_err(|e| format!("{e:?}"))?;
    Ok(())
}

fn config_declares_module() -> Result<(), String> {
    let wanted = format!("[{MODULE}]");
    if CONFIG_INI.lines().any(|line| line.trim() == wanted) {
        Ok(())
    } else {
        Err(format!("config.ini does not declare {wanted}"))
    }
}

fn probe_job() -> Result<(), String> {
    let job = serde_json::json!({"job_id": "selftest", "filename": "probe.csv", "text": PROBE_CSV});
    let out = crate::handle_job(job.to_string().as_bytes(), 1 << 16);
    let r = &out["result"];
    match (out["ok"].as_bool(), r["total_channels"].as_u64(), r["skipped_rows"].as_u64()) {
        (Some(true), Some(2), Some(1)) => Ok(()),
        _ => Err(format!("unexpected answer: {out}")),
    }
}

pub fn self_test_json(depth_word: &str) -> String {
    apkaudio_contracts::self_test::run_as_json(MODULE, depth_word, self_test)
}

#[cfg(test)]
mod tests {
    use super::*;
    use apkaudio_contracts::self_test::Verdict;

    #[test]
    fn the_self_test_passes_against_this_build() {
        let report = self_test(Depth::Own);
        assert_eq!(report.verdict(), Verdict::Pass, "{}", report.headline());
    }
}

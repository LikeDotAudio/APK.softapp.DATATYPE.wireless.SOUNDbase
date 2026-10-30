# Delivery — DATATYPE.wireless.SOUNDbase

← [Wiki home](../README.md)

No build of its own and no live URL. The importer ships compiled into the orchestrator:

| What | Built by | Lands |
|---|---|---|
| `soundbase_importer.rs` | `MANAGEMENT.DataMigration` (`apkaudio-data-migration`) compiles it by `#[path]`; `oaRustCore` re-exports that crate as `data_migration`; `apkaudio-orchestrator` links it. `NODE.BareMetal/Docker/Dockerfile` copies `PODS/POD.THIN/Softapps` so the path resolves in the image. | the **Node-BareMetal** container (`node-baremetal:local`); after an edit, rebuild that stack (`MANAGER.Docktor/K8/up-stack.sh NODE.BareMetal`) |
| the files | `PODS/POD.THIN/Softapps` copied into **WEB.Frontend-Assets**; the monorepo's FTPS job uploads the checkout to apk.audio | nothing opens them there |

Remote: `https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.SOUNDbase.git` (no workflow of its own).

**Tested:** `cargo test --bin test_soundbase_importer` in MANAGEMENT.DataMigration runs the parser's 4 unit tests (header mapping, range guard, missing frequency column, the fixture); `cargo run --bin test_soundbase_importer` prints the fixture's 20 channels. `.apk.scripts/smoke.py` checks only that HEAD is pushed ([SMOKE](../%F0%9F%A7%A9Manual%20and%20Artifacts/SMOKE.md)).

**Not automated:** no CI runs the tests; the Node-BareMetal rebuild is by hand.

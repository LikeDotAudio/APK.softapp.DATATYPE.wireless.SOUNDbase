# Smoke test — DATATYPE.wireless.SOUNDbase

Is `DATATYPE.wireless.SOUNDbase` deployed? Written by `.apk.scripts/smoke.py` from the module's own shape; what each check means is in that script's header.

← [Wiki home](../README.md)

**❌ FAIL** — 2/3 checks, since 2026-11-12 00:16 EST

| Check | Target | Result | Detail |
|---|---|---|---|
| pushed | [APK.softapp.DATATYPE.wireless.SOUNDbase](https://github.com/LikeDotAudio/APK.softapp.DATATYPE.wireless.SOUNDbase.git) | ❌ | 1 commit(s) not pushed |
| container | `Worker-WIRELESS_SOUNDBASE` | ✅ | running, healthy |
| web | [SRC/importer/src/lib.rs](https://apk.audio/APK%3ASoftapps/DATATYPE/DATATYPE.wireless.SOUNDbase/SRC/importer/src/lib.rs) | ✅ | HTTP 200 |

Re-run: `python3 .apk.scripts/smoke.py PODS/POD.THIN/Softapps/DATATYPE/DATATYPE.wireless.SOUNDbase`

<!-- smoke:sig fdb66eb699af since 2026-11-12 00:16 EST -->

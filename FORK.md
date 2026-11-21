# 本 fork 與官方的差異

上游：[zerx-lab/FluxDown](https://github.com/zerx-lab/FluxDown)。本 fork 只跟官方**穩定版 tag**（官方的 `stable` 分支），不跟開發中的 `main`，也不跟 `-rc` 預覽版。

目前同步到：`v0.6.8`（2026-11-16）。

## 本 fork 多做的事

| 改動 | 位置 | 目的 |
|---|---|---|
| 匿名統計預設關閉（opt-in） | `native/agent/src/analytics.rs`、`native/engine/src/db.rs` 與各端設定目錄 | 沒同意就不上報 |
| aria2 JSON-RPC 未設 token 時拒絕 `dir` 選項 | `native/api/src/jsonrpc.rs` | 能連到埠的人不能把檔案寫到任意目錄 |
| Windows 安裝檔比對 Authenticode 簽章 | `native/agent/src/update/install/{authenticode,windows}.rs` | 簽章被竄改的更新包不安裝 |
| 更新站點可在建置期用 `FLUXDOWN_UPDATE_BASE_URLS` 覆寫 | `native/agent/src/update/release.rs` | fork 自行發版時不回官方站抓更新 |
| `dash-mpd` 升到 0.20 | `native/engine/Cargo.toml`、`Cargo.lock` | 換掉 quick-xml 0.39（RUSTSEC-2026-0194 / 0195，惡意 DASH 清單可拖垮解析） |

除了上表與本檔，其餘內容與官方該版本逐位元相同。檢查方式：

```bash
git diff --stat v0.6.8 HEAD    # 應只列出上表的檔案、Cargo.lock 與 FORK.md
```

## 這些改動的限制

- **Authenticode 比對目前擋不到什麼**：官方 Windows 安裝檔沒有簽章，自己編譯的也沒有。規則是「目前程式沒簽章 → 新安裝檔沒簽章也放行」，只會擋下「有簽章但驗證失敗」的檔案。真正的完整性檢查是更新器原本就有的 SHA-256 比對，而清單與安裝檔來自同一個站點。
- **`dash-mpd` 0.20 的代價**：多編進 9 個官方版沒有的套件（`mp4-atom`、`oxideav-core`、`oxideav-pixfmt`、`oxideav-subtitle`、`pastey`、`strip-ansi-escapes`、`vte`、`xmlparser`、`edit-distance`）。其中 `oxideav-*` 是 2026-04 才出現的新套件，版本固定在本 fork 一直使用的那一版，不要順手升級。若不想多帶這些套件，把 `native/engine/Cargo.toml` 的 `dash-mpd` 改回 `0.19`、`Cargo.lock` 改回官方版即可，代價是那兩個 quick-xml 通報回來。

## 已知未修的相依套件通報（與官方 v0.6.8 相同）

`cargo audit` 在本 fork 與官方 v0.6.8 都會報下列項目。修它們要跨大版本升級並改程式，不在「同步官方」的範圍內，等官方處理：

| 套件 | 通報 | 影響面 |
|---|---|---|
| `zip` 2.6.1 | RUSTSEC-2026-0367 / 0368 / 0369 / 0429 / 0545 / 0548 | 解壓縮惡意 zip 時可寫到目標目錄外、沿用危險權限或中止程序 |
| `suppaftp` 6.3.0 | RUSTSEC-2026-0271 | FTP 指令參數含換行時可注入指令 |
| `libsqlite3-sys` 0.37.0 | RUSTSEC-2026-0459 | 內含的 SQLite 有 CVE |
| `rsa` 0.10.0-rc.19 | RUSTSEC-2023-0071 | 計時側信道（Marvin Attack） |

使用上的對應做法：不要用它下載並自動解壓來源不明的壓縮檔，不要連不信任的 FTP 伺服器。

## 更新時要注意

- 本 fork 沒有 Release。Windows 版由 `.github/workflows/fork-windows-build.yml` 在 GitHub 的機器上編譯，見下方「下載 Windows 版」。
- 建置時沒設 `FLUXDOWN_UPDATE_BASE_URLS`，App 內的更新會安裝**官方版**並覆蓋本 fork。要保留上表的改動，就不要在 App 內按更新，改成「同步官方 → 重新編譯」。
- 「自動檢查更新」可以留著開：它只提示官方有新版，正好當作該同步的提醒。
- 靜默自動更新（`general.silent_auto_update`）官方預設關閉，請維持關閉，否則閒置時會自動換成官方版。
- 不要推送 `v*` tag：會觸發 `.github/workflows/release.yml` 的發版流程。`git push` 預設不推 tag，不要加 `--tags`。

## 下載 Windows 版

1. 到本 repo 的 Actions 頁，左側選「Fork Windows build」，按「Run workflow」（分支選 `main`）。約 35 分鐘。
2. 只用**綠色勾勾**的那次執行。點進去，頁面最下方 Artifacts 下載 `FluxDown-fork-windows-x64-portable`。
3. 解開後會有一個 zip 和 `SHA256SUMS.txt`。在 PowerShell 比對：`Get-FileHash .\FluxDown-fork-*.zip -Algorithm SHA256`，結果要和 `SHA256SUMS.txt` 第一行、以及該次執行摘要頁顯示的值相同。
4. 把裡面的 zip 解到固定位置（例如 `D:\Apps\FluxDown`），執行 `fluxdown-desktop.exe`。

這是免安裝版：資料夾裡有 `portable` 標記檔，設定與下載紀錄寫在同資料夾的 `portable_data`。瀏覽器橋接會在目前使用者的登錄檔登記一筆。不用時刪掉整個資料夾即可。更新方式是重跑一次建置、下載新 zip、把舊的 `portable_data` 搬過去。

每次建置會自動做三件事，任何一件失敗該次執行就會標紅：從 zip 解壓後啟動並確認介面、agent、daemon 都在執行；確認服務只監聽本機位址；編譯並測試 Windows 專用的更新器程式碼。

Artifacts 保留 30 天，過期後重跑即可。執行檔沒有程式碼簽章，第一次執行 Windows 會跳 SmartScreen 警告。

## 同步官方新版

```bash
git remote add upstream https://github.com/zerx-lab/FluxDown.git   # 只需一次
git fetch upstream tag vX.Y.Z
git checkout -b sync/upstream-vX.Y.Z main
git merge vX.Y.Z

# Cargo.lock 衝突時：採用官方的，再讓 cargo 只補上本 fork 需要的差異
git checkout --theirs Cargo.lock
cargo metadata --format-version 1 > /dev/null
# 把 oxideav-* 固定回本 fork 原本的版本（版本號以合併前 main 的 Cargo.lock 為準）
cargo update -p oxideav-subtitle --precise 0.1.4
cargo update -p oxideav-pixfmt --precise 0.1.11
cargo update -p oxideav-core --precise 0.1.42

# 驗證
cargo fmt --check
cargo audit
cargo test -p fluxdown_api --lib
cargo test -p fluxdown_agent --lib update::
cargo test -p fluxdown_engine --lib dash
git diff --stat vX.Y.Z HEAD
```

Windows 專用的更新程式碼（`windows.rs`）在 Linux / macOS 上不會被編譯，同步後要在 Windows 上實際編譯一次才算驗過。

合併後把本檔的「目前同步到」改成新版本。

## 建議的下載設定

避免被目標網站限流或封鎖帳號。這些是使用設定，不是程式預設值：

- `auto_max_connections`（預設 16）：一般站降到 4–8，敏感站 1–2。
- `max_concurrent_tasks`（預設 5）：對同一個站同時 1–2 個。
- `speed_limit_bytes`（預設 0 = 不限速）：設一個全域上限。
- `max_auto_retries`（預設 3）與 `auto_retry_delay_secs`（預設 5）：維持預設，不要調高。
- 管理 API 與 MCP 維持關閉；要開就先設 token。

# 在一台新的 Windows 電腦上安裝本 fork

這份文件是給「之後要在別台電腦再裝一次」的自己看的。照順序做，每一步都寫了做完應該看到什麼，對不上就停下來。

本 fork 與官方的差異、已知風險與同步方式見 [FORK.md](FORK.md)。

```
① 編譯（GitHub 代勞）→ ② 下載兩個 Artifact → ③ 驗證 → ④ 安裝桌面程式 → ⑤ 調設定 → ⑥ 裝 Chrome 擴充功能 → ⑦ 試下載
   約 35 分鐘              需登入 GitHub      不執行任何程式
```

需要：Windows 10 / 11（x64）、Chrome、能登入本 repo 的 GitHub 帳號。不需要安裝任何開發工具。

## ① 讓 GitHub 編譯

1. 開本 repo 的 **Actions** 頁，左側選 **Fork Windows build**。
2. 如果清單最上面已經有一筆**綠色勾勾**、而且之後沒有同步過官方新版，可以直接用那一筆，跳到 ②。Artifact 只保留 30 天，過期就要重跑。
3. 否則按右側 **Run workflow**，分支選 `main`，再按綠色的 **Run workflow**。等它跑完，約 35 分鐘。

**應該看到**：該次執行的兩個工作 `Build Windows x64 portable` 與 `Build Chrome extension` 都是綠色勾勾。只要有一個是紅色，就不要用那次的產物。

每次編譯會自動檢查：從 zip 解壓後能啟動（介面、agent、daemon 都在執行）、服務只監聽本機位址、Windows 專用更新器程式碼的測試、擴充功能的 ID 與版本正確且不帶自動更新位址。

## ② 下載

點進那次執行，拉到頁面最下方 **Artifacts**，下載兩個：

| Artifact | 內容 | 大小 |
|---|---|---|
| `FluxDown-fork-windows-x64-portable` | 桌面程式（免安裝） | 約 70 MB |
| `FluxDown-fork-chrome-extension` | Chrome 擴充功能 | 約 0.5 MB |

**Chrome 可能顯示「這個檔案不安全」並擋下桌面程式的 zip。** 原因是檔案剛編出來、沒人下載過，裡面的執行檔也沒有程式碼簽章（官方版同樣沒有）。處理方式：

- 到 `chrome://downloads`，在那一筆選「下載不安全的檔案」或「保留」。
- 沒有這個選項就改用 Edge 或 Firefox 下載。
- 先不要執行，做完 ③ 再說。

## ③ 驗證（不會執行任何程式）

1. **解開外層**。兩個 Artifact 下載後各是一個 zip，解開後裡面是「真正的 zip」加一個 `SHA256SUMS` 文字檔。
2. **比對雜湊**。在解開的資料夾空白處按住 Shift 再按右鍵，選「在終端機中開啟」，執行：
   ```powershell
   Get-FileHash .\FluxDown-fork-*.zip -Algorithm SHA256
   ```
   輸出的 Hash 要和同資料夾的 `SHA256SUMS.txt` 第一行相同（大小寫不影響），該次執行頁面的 Summary 區塊也會顯示同一個值。要在**解開外層之後**的資料夾執行；資料夾裡同時有桌面程式與擴充功能兩個 zip 時會各印一行，擴充功能的對照 `SHA256SUMS-extension.txt`。相同代表檔案就是 GitHub 編出來的那份，下載途中沒被換掉。不同就刪掉重來。
3. **上傳 [VirusTotal](https://www.virustotal.com)**。把桌面程式的內層 zip（`FluxDown-fork-<版本>-<commit>-windows-x64-portable.zip`）拖進去。

   | 結果 | 判讀 |
   |---|---|
   | 0 家報毒 | 繼續 |
   | 1–3 家，名稱是 `Generic`、`Heur`、`ML`、`Suspicious` 這類 | 未簽章新檔案的常見誤判，繼續 |
   | 多家報毒且指名同一個具體家族（例如 `SilverFox`、`ValleyRAT`、`Winos`） | **停止，不要執行** |

4. **Defender 掃描**。對解開的資料夾按右鍵 →「顯示其他選項」→「使用 Microsoft Defender 掃描」，應顯示沒有威脅。

雜湊相符只證明「檔案沒被調包」，掃描才是在檢查「內容有沒有問題」，兩者都要做。

## ④ 安裝桌面程式

1. 把桌面程式的內層 zip 解到固定位置，建議 `C:\Apps`，解完得到 `C:\Apps\FluxDown`。**不要放在下載資料夾或桌面，之後也不要搬動**：設定與下載紀錄都存在裡面的 `portable_data`。
2. 點兩下 `C:\Apps\FluxDown\fluxdown-desktop.exe`。
3. 跳出藍色的「Windows 已保護您的電腦」時，按「其他資訊」→ 確認程式名稱是 `fluxdown-desktop.exe` →「仍要執行」。只有第一次會出現。
4. 想方便就對 `fluxdown-desktop.exe` 按右鍵 →「傳送到」→「桌面（建立捷徑）」。

**應該看到**：主視窗出現，右下角狀態列顯示版本號（與 [FORK.md](FORK.md) 的「目前同步到」相同），系統匣有圖示。

`C:\Apps\FluxDown` 裡應該有：`fluxdown-desktop.exe`、`fluxdown-agent.exe`、`fluxdownd.exe`、`fluxdown_nmh.exe`、`flux_down.exe`、`msvcp140*.dll`、`vcruntime140*.dll`、`portable`、`BUILD-INFO.txt`（記錄這份是從哪個 commit、哪次執行編出來的）。

## ⑤ 調設定

點左下角齒輪。

| 位置 | 設定 | 調成 | 原因 |
|---|---|---|---|
| 一般 | 匿名使用統計 | 確認是關的（本 fork 預設關） | 不對外回報 |
| 關於 | 靜默自動更新 | 關 | 否則閒置時會被換成官方版 |
| 下載 | 自動模式執行緒上限 | 預設 16 → 4 到 8（敏感站 1 到 2） | 連線太多會被網站限流或封鎖 |
| 下載 | 最大同時下載數 | 預設 5 → 1 到 2 | 同上 |
| 下載 | 下載限速 | 設一個上限 | 同上 |
| API 服務 | 管理 API、MCP 端點 | 維持關閉 | 不開多餘的入口 |
| 帳號 | 登入雲端 | 不登入（除非真的需要跨裝置同步） | 資料不離開本機 |

「自動檢查更新」可以留著開，它只是提示官方有新版，正好當作該同步的提醒。**但不要按 App 內的「更新」**，那會安裝官方版並覆蓋本 fork。

## ⑥ 裝 Chrome 擴充功能

用途：在 Chrome 登入網站後直接點下載，由 FluxDown 接手，並帶上登入狀態。

1. 把擴充功能的內層 zip 解到固定位置，建議 `C:\Apps`，解完得到 `C:\Apps\FluxDown-fork-<版本>-<commit>-chrome-extension`。**之後不要搬動或刪除**，Chrome 是直接從這個資料夾載入的。
2. Chrome 開 `chrome://extensions`，打開右上角 **開發人員模式**。
3. 按 **載入未封裝項目**，選那個含有 `manifest.json` 的資料夾。
4. 確認擴充功能的 ID 是 `meleenglfggcmcajknpeeeiobnpfmahc`。桌面程式只放行這個 ID。
5. FluxDown 桌面程式要開著（或在系統匣）。點工具列上的擴充功能圖示，應顯示已連線。

注意：

- **不要同時安裝商店版**，兩者 ID 相同會互相覆蓋。
- Chrome 啟動時可能提醒「停用開發人員模式的擴充功能」，選保留即可。
- 自編版不會自動更新。這是刻意的：程式碼固定在本 repo 的版本，不讓外部遠端更換。
- 某次下載不想讓它接手（例如公司內部系統、含機密的檔案），在擴充功能的選項裡把該站點設為不接管，或暫停接管後用瀏覽器原本的方式下載。交給 FluxDown 的下載會連同該網址的 Cookie 一起傳給本機桌面程式。

擴充功能做什麼、不做什麼，見 [FORK.md](FORK.md) 的「自編的 Chrome 擴充功能」。

## ⑦ 試下載

1. **貼網址**：按左上角「＋ 下載任務」，貼上
   `https://raw.githubusercontent.com/ZLO-25/FluxDown/main/FORK.md`，確認任務跑到「已完成」。
2. **瀏覽器接手**：在 Chrome 任一網頁點一個下載連結，任務應出現在 FluxDown。
3. **登入下載**：到一個需要登入才能下載的網站，登入後點下載，確認能下載完成而不是下載到登入頁。

## 附加工具：影片轉 MP3

[`fork-tools/轉成MP3.bat`](fork-tools/轉成MP3.bat) 把影片的聲音另存成 MP3，與 FluxDown 本身無關，可單獨使用。在 GitHub 上打開該檔案，按右上角的下載按鈕（Download raw file）取得。

| 用法 | 做法 | 結果 |
|---|---|---|
| 拖放 | 把一個或多個影片檔拖到 `轉成MP3.bat` 的圖示上放開 | 只轉拖進來的檔案，任何 FFmpeg 支援的影片格式都可以 |
| 點兩下 | 把 `轉成MP3.bat` 放進有影片的資料夾，點兩下 | 轉換**該資料夾內**所有 `.mp4`（不含子資料夾） |

- MP3 產生在影片旁邊，檔名相同；原本的影片不會被更動。
- 已有同名 MP3 的會跳過，所以同一個資料夾可以重複執行，只補轉新的。
- 第一次執行若電腦沒有 FFmpeg，會用 `winget` 自動安裝（`Gyan.FFmpeg`），裝完再執行一次。
- 第一次執行可能跳出「Windows 已保護您的電腦」，按「其他資訊」→「仍要執行」。這是純文字腳本，可用記事本打開檢視。
- 檔名含 `%`、`^` 等特殊符號時批次檔可能處理失敗，先改檔名再轉。

## 日常使用

- 關掉視窗後程式仍在系統匣繼續下載；要完全結束，在系統匣圖示上按右鍵退出。
- 對同一個網站大量下載前，先把執行緒與同時下載數再調低，網盤類尤其要注意（被封的可能是帳號，不只是 IP）。
- 不要使用第三方「加速」外掛：它會把下載導向別人的節點，同時帶來封號與被竄改的風險。
- 公司的電腦與公司網路不要安裝。
- 防毒軟體擋下時，先確認 ③ 的雜湊相符，再只放行那幾個執行檔；不要把整個資料夾加入排除清單。

## 更新到新版

官方發新的穩定版時：

1. 照 [FORK.md](FORK.md) 的「同步官方新版」把新版併進 `main`，並更新「目前同步到」。
2. 重做本文件的 ① ② ③。
3. 桌面程式（PowerShell，一行一行執行，出現紅字就停）：
   ```powershell
   # 先關掉 Chrome，再結束所有 FluxDown 程式；只關視窗不夠，背景服務會占用資料夾
   Get-Process fluxdown-desktop, fluxdown-agent, fluxdownd, fluxdown_nmh, flux_down -ErrorAction SilentlyContinue | Stop-Process -Force
   Start-Sleep 2
   Get-Process fluxdown*, flux_down -ErrorAction SilentlyContinue   # 應該什麼都不印

   Rename-Item C:\Apps\FluxDown C:\Apps\FluxDown-old
   # 寫完整檔名，不要用 * 萬用字元：資料夾裡有兩個 zip 時會失敗
   Expand-Archive <內層 zip 的完整路徑> -DestinationPath C:\Apps
   Copy-Item C:\Apps\FluxDown-old\portable_data C:\Apps\FluxDown\ -Recurse
   ```
   開啟新的 `fluxdown-desktop.exe`，確認版本號、設定與下載紀錄都在，再刪掉 `C:\Apps\FluxDown-old`。
4. 擴充功能：把新資料夾的內容覆蓋到原本的擴充功能資料夾，再到 `chrome://extensions` 按該擴充功能的重新載入。

## 搬到另一台電腦

照本文件從 ① 做起即可。想保留設定與下載紀錄，就把舊電腦的 `C:\Apps\FluxDown\portable_data` 複製到新電腦同一個位置（兩邊的 FluxDown 都要先結束）。

## 移除

1. 在系統匣結束 FluxDown。
2. `chrome://extensions` 移除擴充功能。
3. 刪除 `C:\Apps\FluxDown` 與擴充功能資料夾。

## 常見狀況

| 狀況 | 原因與處理 |
|---|---|
| Actions 頁沒有 Run workflow 按鈕，或顯示 Actions 已停用 | repo 的 Settings → Actions → General 被設成停用。改回允許執行。 |
| 只想關掉官方的排程工作 `Chrome Store CRX` | Actions 頁左側點它 → 右上角「⋯」→ Disable workflow。不要停用整個 repo 的 Actions。 |
| Chrome 顯示「這個檔案不安全」 | 見 ②。 |
| 雙擊後跳「Windows 已保護您的電腦」 | 執行檔沒有程式碼簽章，見 ④ 第 3 步。 |
| 擴充功能顯示未連線 | 桌面程式沒開；或擴充功能 ID 不是 ⑥ 第 4 步那個；或同時裝了商店版。 |
| 登入後下載到的是登入頁或 403 | 該站不接受帶 Cookie 的多連線下載。把該任務的連線數降到 1，或改用瀏覽器下載。 |
| 下載到一半被限速或封鎖 | 連線數太高，見 ⑤。 |
| App 提示有新版 | 不要按更新，見「更新到新版」。 |

不要對本 repo 推送 `v*` tag：那會觸發官方的發版流程。

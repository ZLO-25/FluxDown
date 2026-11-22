@echo off
chcp 65001 >nul
setlocal EnableExtensions DisableDelayedExpansion
title 轉成 MP3

where ffmpeg >nul 2>nul
if errorlevel 1 goto install

set "COUNT=0"
if "%~1"=="" goto folder

rem ── 用法一：把影片拖到這個檔案上 ──
:next
if "%~1"=="" goto done
call :convert "%~f1"
shift
goto next

rem ── 用法二：直接點兩下，轉換這個檔案所在資料夾內的 mp4（不含子資料夾）──
:folder
echo 沒有拖入檔案，改為轉換這個資料夾內的所有 mp4：
echo   %~dp0
echo.
for %%F in ("%~dp0*.mp4") do call :convert "%%~fF"
goto done

:convert
set /a COUNT+=1
if exist "%~dpn1.mp3" goto skip
echo 轉換中：%~nx1
ffmpeg -hide_banner -loglevel error -stats -i "%~f1" -vn -c:a libmp3lame -q:a 2 "%~dpn1.mp3"
if errorlevel 1 goto failed
echo 完成：%~n1.mp3
echo.
exit /b
:skip
echo 已存在，跳過：%~n1.mp3
echo.
exit /b
:failed
echo 失敗：%~nx1
if exist "%~dpn1.mp3" del "%~dpn1.mp3"
echo.
exit /b

:done
if "%COUNT%"=="0" echo 這個資料夾裡沒有 mp4 檔。
if not "%COUNT%"=="0" echo 全部處理完畢，共 %COUNT% 個檔案。
pause
exit /b

:install
echo 這台電腦還沒有 FFmpeg，現在幫你安裝（只需要一次）...
winget install --id Gyan.FFmpeg -e --accept-source-agreements --accept-package-agreements
echo.
echo 安裝完成後，請關掉這個視窗，再執行一次。
pause
exit /b

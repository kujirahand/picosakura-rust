@echo off
setlocal enabledelayedexpansion

rem スクリプトディレクトリの取得
set SCRIPT_DIR=%~dp0
set TARGET=%SCRIPT_DIR%picosakura-pack
set ZIPFILE=win-picosakura-pack.zip

rem --- build ---
rem set root
cd /d %SCRIPT_DIR%

rem cargo build
cargo build --release

rem --- build tools ---
cd /d %SCRIPT_DIR%tools\mml2wav
cargo build --release

rem --- copy ---
cd /d %SCRIPT_DIR%
mkdir %TARGET%
mkdir %TARGET%\fonts
mkdir %TARGET%\samples

copy /y .\target\release\picosakura.exe %TARGET%\
copy /y .\tools\mml2wav\target\release\mml2wav.exe %TARGET%\
xcopy /y /e /i .\fonts\* %TARGET%\fonts\
xcopy /y /e /i .\samples\* %TARGET%\samples\
copy /y README.md %TARGET%\
copy /y LICENSE %TARGET%\

rem zip
rem if exist %ZIPFILE% del %ZIPFILE%
rem powershell Compress-Archive -Path %TARGET% -DestinationPath %ZIPFILE% -Force -Exclude "*.DS_Store", "*__MACOSX*"
echo ok

pause


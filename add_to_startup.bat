@echo off
setlocal

set "fileName=Altccents"

if exist %~dp0%fileName%.exe (
    mklink "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\%fileName%" "%~dp0%fileName%.exe"
) else (
    echo File "%fileName%.exe" not found in "%~dp0".
)

endlocal
pause

@echo off
setlocal

set "fileName=Altccents"

if exist %fileName%.exe (
    mklink "%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\%fileName%" "%cd%\%fileName%.exe"
) else (
    echo File "%fileName%.exe" not found in "%cd%\".
)

endlocal
pause

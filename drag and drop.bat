@echo off

if "%~1" == "" (
    echo Drag and drop a Game Pass save folder onto this batch file.
    pause >nul
    exit
)

cd /d "%~dp0"
set "cmd=%~dp0gpts.exe -i "%~1""

%cmd%

if %errorlevel% neq 0 (
    echo Not OK. Press enter to exit.
    pause >nul
    exit
)

echo OK. Press enter to exit.
pause >nul
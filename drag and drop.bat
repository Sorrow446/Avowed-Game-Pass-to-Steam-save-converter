@echo off
setlocal enabledelayedexpansion

if "%~1" == "" (
    echo Drag and drop a Game Pass save folder onto this batch file.
    pause >nul
    exit
)

cd "%~dp0"
set "cmd=gpts.exe --in-path "!%~1!""
%cmd%

if %errorlevel% neq 0 (
    echo Not OK. Press enter to exit.
    pause >nul
    exit
)

echo OK. Press enter to exit.
pause >nul
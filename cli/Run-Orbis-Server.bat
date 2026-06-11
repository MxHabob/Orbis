@echo off
:: =============================================================================
:: Orbis School Server Setup Launcher (Self-Elevating Admin bat Wrapper)
:: =============================================================================

:: Check for Administrator privileges
net session >nul 2>&1
if %errorLevel% == 0 (
    goto :run
) else (
    echo [Orbis Launcher] Requesting Administrator privileges to start server...
    powershell -Command "Start-Process '%~f0' -Verb RunAs"
    exit /b
)

:run
title Orbis School Server Launcher
cd /d "%~dp0"
cls
powershell -NoProfile -ExecutionPolicy Bypass -File "start-school-server.ps1"
echo.
echo =====================================================================
echo [Orbis Launcher] Execution completed. Press any key to close.
echo =====================================================================
pause >nul

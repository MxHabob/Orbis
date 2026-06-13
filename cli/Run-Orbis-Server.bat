@echo off
:: =============================================================================
:: Orbis School Server Setup Launcher (Direct bat Wrapper)
:: =============================================================================

title Orbis School Server Launcher
cd /d "%~dp0"
cls
powershell -NoProfile -ExecutionPolicy Bypass -File "start-school-server.ps1"
echo.
echo =====================================================================
echo [Orbis Launcher] Execution completed. Press any key to close.
echo =====================================================================
pause >nul

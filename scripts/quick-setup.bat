@echo off
REM Windows batch version of the setup script

echo 🚗 Vehicle Net Quick Setup for Windows
echo ========================================
echo.

echo 🔧 Step 1: Starting Anvil (if not running)...
start /B anvil
timeout /t 3 /nobreak >nul

echo 🔧 Step 2: Setting up contracts and vehicles...
bash scripts/complete-setup.sh

echo.
echo ✅ Setup complete! 
echo 🌐 Start the web frontend with: bash scripts/run-web.sh
echo 📱 Then visit http://localhost:3001
pause
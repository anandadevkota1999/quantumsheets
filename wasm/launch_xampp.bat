@echo off
echo 🚀 SETTING UP QUANTUM SHEETS FOR XAMPP
echo ======================================

echo 1. Creating Quantum Sheets directory in XAMPP...
mkdir "C:\xampp\htdocs\quantumsheets" 2>nul

echo 2. Copying WebAssembly files...
xcopy /Y /E "www\*" "C:\xampp\htdocs\quantumsheets\"
copy "pkg\quantum_sheets_wasm.js" "C:\xampp\htdocs\quantumsheets\quantum_sheets.js"
copy "pkg\quantum_sheets_wasm_bg.wasm" "C:\xampp\htdocs\quantumsheets\quantum_sheets_bg.wasm"

echo 3. Creating index.php for easy access...
echo ^<?php header('Location: index.html'); ?^> > "C:\xampp\htdocs\quantumsheets\index.php"

echo.
echo ✅ QUANTUM SHEETS SETUP COMPLETE!
echo.
echo 🌐 Access Quantum Sheets at:
echo    http://localhost/quantumsheets/
echo    http://localhost/quantumsheets/index.html
echo    http://localhost/quantumsheets/demo.html
echo.
echo 🚀 Start XAMPP Apache if not running:
echo    1. Open XAMPP Control Panel
echo    2. Start Apache service
echo    3. Open browser to http://localhost/quantumsheets/
echo.
pause
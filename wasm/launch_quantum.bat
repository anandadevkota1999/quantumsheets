@echo off
echo 🚀 LAUNCHING QUANTUM SHEETS v0.6.0
echo ===================================

echo 1. Copying WebAssembly files...
copy pkg\quantum_sheets_wasm.js www\quantum_sheets.js >nul
copy pkg\quantum_sheets_wasm_bg.wasm www\quantum_sheets_bg.wasm >nul

echo 2. Starting web server...
cd www
echo.
echo ✅ QUANTUM SHEETS IS READY!
echo.
echo 🌐 Open in browser:
echo    • Main Interface: http://localhost:8000/index.html
echo    • Simple Demo: http://localhost:8000/demo.html
echo.
echo 🎯 Features to test:
echo    • AI Assistant: Type "add A1 and B2"
echo    • Nepal Phone Generator: Click "Generate Data" tab
echo    • Custom Operations: Register your own functions
echo.
python -m http.server 80
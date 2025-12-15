#!/bin/bash

# Run this to build and test Quantum Sheets WASM

echo "🔧 Building Quantum Sheets WASM..."
chmod +x build.bat
./build.bat

echo ""
echo "🌐 Starting web server..."
echo "Open your browser and navigate to:"
echo "1. http://localhost:8000/wasm/www/index.html  (Full Interface)"
echo "2. http://localhost:8000/wasm/www/demo.html   (Quick Demo)"

# Start Python web server
cd wasm
python3 -m http.server 8000
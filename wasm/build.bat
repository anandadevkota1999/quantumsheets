@echo off
echo 🚀 Building Quantum Sheets for WebAssembly...

REM Build the engine first
cd ..\engine
echo Building Rust engine...
cargo build --release
cd ..\wasm

REM Build WASM
echo Building WebAssembly...
cargo build --release --target wasm32-unknown-unknown

REM Install wasm-bindgen-cli if needed
where wasm-bindgen >nul 2>nul
if errorlevel 1 (
    echo Installing wasm-bindgen-cli...
    cargo install wasm-bindgen-cli
)

REM Generate JavaScript bindings
echo Generating JavaScript bindings...
wasm-bindgen target\wasm32-unknown-unknown\release\quantum_sheets_wasm.wasm --out-dir .\www --target web --no-typescript

echo ✅ Build complete! Open wasm\www\index.html in your browser
pause
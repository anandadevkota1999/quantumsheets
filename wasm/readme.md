# Quantum Sheets WebAssembly

This is the WebAssembly version of Quantum Sheets, running in your browser.

## Quick Start

### Prerequisites
- Rust (latest stable)
- wasm-bindgen-cli: `cargo install wasm-bindgen-cli`
- Modern web browser (Chrome/Firefox/Edge)

### Build & Run

```bash
# Clone repository
git clone https://github.com/anandadevkota1999/quantumsheets
cd quantumsheets/wasm

# Build WASM
./build.sh

# Start web server
python3 -m http.server 8000
# or
npm install -g serve
serve www

# Open browser to:
# http://localhost:8000/index.html
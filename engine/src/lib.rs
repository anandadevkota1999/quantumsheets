//! Quantum Sheets - Core Engine
//! Day 5: Complete Operation System

// Declare modules
pub mod api;
pub mod operations;
pub mod compute;
pub mod grid;
pub mod formula;
pub mod ai;
pub mod export;
pub mod excel;
pub mod storage;

// Re-export commonly used types
pub use grid::QuantumGrid;
pub use excel::CellRef;
pub use storage::QuantumColumn;
pub use formula::ast::Formula;
pub use api::QuantumAPI;  // NEW: Main user API


// For WASM compatibility
#[cfg(feature = "wasm")]
pub use wasm_bindgen;
#[cfg(feature = "wasm")]
pub use js_sys;

/// Initialize the Quantum Engine
pub fn init() {
    println!("🚀 Quantum Sheets Engine v0.5.0");
    println!("   Now with complete operation system!");
}

/// Get version
pub fn version() -> &'static str {
    "0.5.0"
}
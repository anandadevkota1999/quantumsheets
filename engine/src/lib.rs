//! Quantum Sheets - Core Engine
//! Day 4: AI Data Generator & Advanced Formula Parsing

// Declare modules
pub mod compute;
pub mod excel;
pub mod grid;
pub mod formula;
pub mod ai;
pub mod storage;
pub mod export;  // NEW

// Re-export commonly used types
pub use grid::QuantumGrid;
pub use excel::CellRef;
pub use storage::QuantumColumn;
pub use formula::ast::Formula;

/// Initialize the Quantum Engine
pub fn init() {
    println!("🚀 Quantum Sheets Engine v0.4.0");
    println!("   Now with AI Data Generator and advanced formulas!");
}

/// Get version
pub fn version() -> &'static str {
    "0.4.0"
}
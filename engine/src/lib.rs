//! Quantum Sheets - Core Engine
//! Day 3: Adding formula parsing and AI features

// Declare modules
pub mod compute;
pub mod excel;
pub mod grid;
pub mod formula;
pub mod ai;
pub mod storage;

// Re-export commonly used types
pub use grid::QuantumGrid;
pub use excel::CellRef;
pub use storage::QuantumColumn;
pub use formula::ast::Formula;

/// Initialize the Quantum Engine
pub fn init() {
    println!("🚀 Quantum Sheets Engine v0.3.0");
    println!("   Now with formula parsing and AI!");
}

/// Get version
pub fn version() -> &'static str {
    "0.3.0"
}
//! Export functionality for Quantum Sheets
//! Supports CSV, JSON, and future Excel export

use std::fs;
use std::io::Write;

/// Export data to different formats
pub struct Exporter;

impl Exporter {
    /// Export grid data to CSV
    pub fn grid_to_csv(grid: &crate::grid::QuantumGrid, filename: &str) -> Result<(), String> {
        let mut file = fs::File::create(filename)
            .map_err(|e| format!("Failed to create CSV file: {}", e))?;
        
        // Simple implementation - we'll enhance this later
        writeln!(file, "Column,Sum,Count")
            .map_err(|e| format!("Failed to write CSV header: {}", e))?;
        
        for (col_idx, column) in grid.columns() {  // Use the public getter
            let col_name = if *col_idx < 26 {
                ((b'A' + *col_idx as u8) as char).to_string()
            } else {
                format!("Col{}", col_idx)
            };
            
            let sum: f64 = column.sum();  // Explicit type annotation
            let count: usize = column.count();  // Explicit type annotation
            
            writeln!(file, "{},{:.2},{}", col_name, sum, count)
                .map_err(|e| format!("Failed to write CSV row: {}", e))?;
        }
        
        Ok(())
    }
    
    /// Export data to JSON
    pub fn to_json<T: serde::Serialize>(data: &T, filename: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
        
        fs::write(filename, json)
            .map_err(|e| format!("Failed to write JSON file: {}", e))?;
        
        Ok(())
    }
    
    /// Quick export for testing
    pub fn quick_export(data: &str, filename: &str) -> Result<(), String> {
        fs::write(filename, data)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        println!("✅ Exported to: {}", filename);
        Ok(())
    }
}
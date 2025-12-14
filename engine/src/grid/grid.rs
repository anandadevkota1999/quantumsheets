use super::CellRef;
use crate::storage::QuantumColumn;
use std::collections::HashMap;

/// The main spreadsheet grid
pub struct QuantumGrid {
    columns: HashMap<u32, QuantumColumn>,
}

impl QuantumGrid {
    /// Create a new empty grid
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }
    
    /// Set a cell value
    pub fn set_cell(&mut self, col: u32, row: u32, value: f64) -> Result<(), String> {
        let column = self.columns
            .entry(col)
            .or_insert_with(|| QuantumColumn::new_float(&format!("Col{}", col), 1000));
        
        // For now, we'll just push (in real implementation, we'd set at specific row)
        column.push(Some(value))
    }
    
    /// Get sum of a column
    pub fn column_sum(&self, col: u32) -> Option<f64> {
        self.columns.get(&col).and_then(|c| c.sum())
    }
    
    /// Print memory report
    pub fn memory_report(&self) {
        let total_cells: usize = self.columns.values().map(|c| c.count()).sum();
        let total_memory: usize = self.columns.values().map(|c| c.memory_used()).sum();
        
        if total_cells > 0 {
            let memory_per_cell = total_memory as f64 / total_cells as f64;
            let excel_memory_per_cell = 40.0; // Excel uses ~40 bytes per cell
            
            println!("\n📊 MEMORY EFFICIENCY REPORT:");
            println!("   Total cells: {}", total_cells);
            println!("   Total memory: {} bytes", total_memory);
            println!("   Our memory per cell: {:.1} bytes", memory_per_cell);
            println!("   Excel memory per cell: {} bytes", excel_memory_per_cell);
            println!("   Improvement: {:.1}x more efficient", 
                     excel_memory_per_cell / memory_per_cell);
        }
    }
}
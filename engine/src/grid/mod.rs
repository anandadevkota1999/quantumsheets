//! QuantumGrid - Multiple columns spreadsheet with formula support

use crate::excel::CellRef;
use crate::formula::ast::Formula;
use crate::storage::QuantumColumn; // Updated import
use std::collections::HashMap; // Updated import

/// Main spreadsheet grid
pub struct QuantumGrid {
    columns: HashMap<u32, QuantumColumn>,
    formulas: HashMap<CellRef, Formula>,
}

impl QuantumGrid {
    /// Create a new empty grid
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            formulas: HashMap::new(),
        }
    }

    /// Set a cell value by Excel reference (e.g., "A1", "B2")
    pub fn set_cell(&mut self, reference: &str, value: f64) -> Result<(), String> {
        let cell_ref = CellRef::parse(reference)?;

        if !cell_ref.is_valid() {
            return Err(format!("Cell reference out of Excel bounds: {}", reference));
        }

        let (_row_idx, col_idx) = cell_ref.to_zero_based();

        // Get or create column
        let column = self
            .columns
            .entry(col_idx as u32)
            .or_insert_with(|| QuantumColumn::new(&format!("Col{}", col_idx)));

        // For simplicity, just push (real implementation would insert at row)
        column.push(value);

        Ok(())
    }

    /// Set a formula in a cell
    pub fn set_formula(&mut self, reference: &str, formula: &str) -> Result<(), String> {
        let cell_ref = CellRef::parse(reference)?;

        if !cell_ref.is_valid() {
            return Err(format!("Cell reference out of Excel bounds: {}", reference));
        }

        let parsed_formula = Formula::parse(formula)?;
        self.formulas.insert(cell_ref, parsed_formula);

        Ok(())
    }

    /// Set formula from natural language
    pub fn set_formula_natural(
        &mut self,
        reference: &str,
        natural_text: &str,
    ) -> Result<(), String> {
        use crate::ai::nlp::NaturalLanguageTranslator;

        let translator = NaturalLanguageTranslator::new();

        // translator.translate() returns Option<String>
        match translator.translate(natural_text) {
            Some(formula) => self.set_formula(reference, &formula),
            None => Err(format!("Could not understand: '{}'", natural_text)),
        }
    }

    /// Get sum of a column by letter
    pub fn column_sum(&self, col_letter: char) -> Option<f64> {
        let col_upper = col_letter.to_ascii_uppercase();
        if !col_upper.is_ascii_alphabetic() {
            return None;
        }

        let col_idx = (col_upper as u32) - ('A' as u32);
        self.columns.get(&col_idx).map(|c| c.sum())
    }

    /// Get sum of a range
    pub fn sum_range(&self, range: &str) -> Result<f64, String> {
        if let Some(col_letter) = range.chars().next() {
            if col_letter.is_ascii_alphabetic() {
                self.column_sum(col_letter)
                    .ok_or_else(|| format!("Column {} not found or empty", col_letter))
            } else {
                Err(format!("Invalid range format: '{}'", range))
            }
        } else {
            Err("Empty range".to_string())
        }
    }

    /// Print statistics
    pub fn print_stats(&self) {
        println!("📊 Quantum Grid Statistics:");
        println!("   Columns: {}", self.columns.len());
        println!("   Formulas: {}", self.formulas.len());

        let total_cells: usize = self.columns.values().map(|c| c.count()).sum();
        println!("   Total cells: {}", total_cells);

        for (col_idx, column) in &self.columns {
            let col_name = if *col_idx < 26 {
                ((b'A' + *col_idx as u8) as char).to_string()
            } else {
                format!("Col{}", col_idx)
            };
            println!(
                "   Column {}: {} cells, sum={:.1}",
                col_name,
                column.count(),
                column.sum()
            );
        }

        // Show formulas
        if !self.formulas.is_empty() {
            println!("\n   Formulas:");
            for (cell_ref, formula) in &self.formulas {
                println!("     {}: {}", cell_ref, formula.to_excel());
            }
        }
    }

    pub fn columns(&self) -> &HashMap<u32, QuantumColumn> {
        &self.columns
    }

    /// Get formulas (for export)
    pub fn formulas(&self) -> &HashMap<CellRef, Formula> {
        &self.formulas
    }
    /// Get cell value
    pub fn get_cell(&self, reference: &str) -> Result<f64, String> {
        // Parse cell reference like "A1"
        let (col_str, row_str) = reference.split_at(1);
        let col = col_str.chars().next().unwrap() as usize - 'A' as usize;
        let row: usize = row_str.parse().map_err(|e| format!("Invalid row: {}", e))?;

        // Convert usize to u32 for HashMap lookup
        let col_u32 = col as u32;

        // Get column
        if let Some(column) = self.columns.get(&col_u32) {
            if row < column.data.len() {
                Ok(column.data[row])
            } else {
                Err(format!("Row {} out of bounds", row))
            }
        } else {
            Err(format!("Column {} not found", col_str))
        }
    }
}

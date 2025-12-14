//! Columnar storage system

mod columnar;
pub use columnar::QuantumColumn;

/// Simple column statistics
#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub sum: Option<f64>,
    pub count: usize,
    pub null_count: usize,
}

impl ColumnStats {
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
            sum: None,
            count: 0,
            null_count: 0,
        }
    }
}
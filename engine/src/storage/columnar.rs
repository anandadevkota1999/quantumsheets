//! QuantumColumn - Beats Excel's memory usage

use super::ColumnStats;

/// QuantumColumn - Our efficient column storage
pub struct QuantumColumn {
    _name: String,  // Underscore indicates intentionally unused
    data: Vec<f64>,  // Changed from Option<f64> to f64 for simplicity
    stats: ColumnStats,
}

impl QuantumColumn {
    /// Create a new column
    pub fn new(name: &str) -> Self {
        Self {
            _name: name.to_string(),
            data: Vec::new(),
            stats: ColumnStats::new(),
        }
    }
    
    /// Add a value to the column
    pub fn push(&mut self, value: f64) {
        self.data.push(value);
        
        // Update statistics
        self.stats.count += 1;
        self.stats.min = Some(self.stats.min.map(|m| m.min(value)).unwrap_or(value));
        self.stats.max = Some(self.stats.max.map(|m| m.max(value)).unwrap_or(value));
        self.stats.sum = Some(self.stats.sum.unwrap_or(0.0) + value);
    }
    
    /// Sum all values in the column
    pub fn sum(&self) -> f64 {
        self.stats.sum.unwrap_or(0.0)
    }
    
    /// Average of values (Excel-compatible)
    pub fn average(&self) -> f64 {
        if self.stats.count == 0 {
            0.0
        } else {
            self.sum() / self.stats.count as f64
        }
    }
    
    /// Count of values
    pub fn count(&self) -> usize {
        self.stats.count
    }
    
    /// Get memory usage in bytes
    pub fn memory_used(&self) -> usize {
        std::mem::size_of::<Self>() + 
        (self.data.capacity() * std::mem::size_of::<f64>())
    }
    
    /// Get memory usage per value
    pub fn memory_per_value(&self) -> f64 {
        if self.stats.count > 0 {
            self.memory_used() as f64 / self.stats.count as f64
        } else {
            0.0
        }
    }
    
    /// Minimum value
    pub fn min(&self) -> Option<f64> {
        self.stats.min
    }
    
    /// Maximum value
    pub fn max(&self) -> Option<f64> {
        self.stats.max
    }
}

// Excel-compatible functions
impl QuantumColumn {
    /// Excel SUM function
    pub fn excel_sum(&self) -> f64 {
        self.sum()
    }
    
    /// Excel AVERAGE function
    pub fn excel_average(&self) -> f64 {
        self.average()
    }
    
    /// Excel COUNT function
    pub fn excel_count(&self) -> f64 {
        self.count() as f64
    }
    
    /// Excel MIN function
    pub fn excel_min(&self) -> f64 {
        self.min().unwrap_or(0.0)
    }
    
    /// Excel MAX function
    pub fn excel_max(&self) -> f64 {
        self.max().unwrap_or(0.0)
    }
    
    /// Get data slice
    pub fn data(&self) -> &[f64] {
        &self.data
    }
}
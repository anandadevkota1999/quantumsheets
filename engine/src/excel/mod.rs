//! Excel compatibility - Parse A1, B2, AA100 style references

use std::fmt;

/// Excel-style cell reference (e.g., A1, B2, AA100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellRef {
    pub row: u32,    // 1-based row number
    pub col: u32,    // 1-based column number
}

impl CellRef {
    /// Parse from Excel notation (e.g., "A1", "B2", "AA100")
    pub fn parse(excel_ref: &str) -> Result<Self, String> {
        let excel_ref = excel_ref.trim();
        if excel_ref.is_empty() {
            return Err("Empty cell reference".to_string());
        }
        
        let mut chars = excel_ref.chars().peekable();
        let mut col_str = String::new();
        
        // Parse column letters (A, B, ..., Z, AA, AB, etc.)
        while let Some(&c) = chars.peek() {
            if c.is_ascii_alphabetic() {
                col_str.push(c.to_ascii_uppercase());
                chars.next(); // Consume the character
            } else {
                break;
            }
        }
        
        // Parse row number (rest of the string)
        let row_str: String = chars.collect();
        
        if col_str.is_empty() {
            return Err(format!("No column letters in '{}'", excel_ref));
        }
        
        if row_str.is_empty() {
            return Err(format!("No row number in '{}'", excel_ref));
        }
        
        // Convert column letters to number (A=1, B=2, ..., Z=26, AA=27, etc.)
        let col = col_str
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let digit = (c as u32) - ('A' as u32) + 1;
                digit * 26u32.pow(i as u32)
            })
            .sum();
        
        let row = row_str.parse::<u32>()
            .map_err(|_| format!("Invalid row number '{}' in '{}'", row_str, excel_ref))?;
        
        if row == 0 {
            return Err("Row number must be at least 1".to_string());
        }
        
        Ok(Self { row, col })
    }
    
    /// Convert to Excel notation
    pub fn to_excel(&self) -> String {
        let mut col = self.col;
        let mut col_str = String::new();
        
        // Convert column number to letters
        while col > 0 {
            col -= 1;
            let digit = (col % 26) as u8;
            col_str.insert(0, (b'A' + digit) as char);
            col /= 26;
        }
        
        format!("{}{}", col_str, self.row)
    }
    
    /// Create from row and column indices (1-based)
    pub fn new(row: u32, col: u32) -> Self {
        Self { row, col }
    }
    
    /// Convert to 0-based indices for internal use
    pub fn to_zero_based(&self) -> (usize, usize) {
        ((self.row - 1) as usize, (self.col - 1) as usize)
    }
    
    /// Check if this is a valid Excel reference
    pub fn is_valid(&self) -> bool {
        self.row >= 1 && self.row <= 1048576 &&  // Excel row limit
        self.col >= 1 && self.col <= 16384       // Excel column limit (XFD)
    }
}

impl fmt::Display for CellRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_excel())
    }
}

/// Excel range (e.g., "A1:B10")
#[derive(Debug, Clone)]
pub struct CellRange {
    pub start: CellRef,
    pub end: CellRef,
}

impl CellRange {
    /// Parse Excel range notation (e.g., "A1:B10")
    pub fn parse(range: &str) -> Result<Self, String> {
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid range format: '{}' (expected format: A1:B10)", range));
        }
        
        let start = CellRef::parse(parts[0])?;
        let end = CellRef::parse(parts[1])?;
        
        if !start.is_valid() {
            return Err(format!("Invalid start cell in range: {}", start));
        }
        
        if !end.is_valid() {
            return Err(format!("Invalid end cell in range: {}", end));
        }
        
        Ok(Self { start, end })
    }
    
    /// Convert to Excel notation
    pub fn to_excel(&self) -> String {
        format!("{}:{}", self.start.to_excel(), self.end.to_excel())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cell_parsing() {
        let test_cases = vec![
            ("A1", (1, 1)),
            ("B2", (2, 2)),
            ("Z26", (26, 26)),
            ("AA27", (27, 27)),
            ("AB100", (100, 28)),
            ("XFD1048576", (1048576, 16384)),
        ];
        
        for (excel_ref, (expected_row, expected_col)) in test_cases {
            let cell = CellRef::parse(excel_ref).unwrap();
            assert_eq!(cell.row, expected_row, "Row mismatch for {}", excel_ref);
            assert_eq!(cell.col, expected_col, "Col mismatch for {}", excel_ref);
            
            // Round trip test
            let round_trip = cell.to_excel();
            assert_eq!(round_trip, excel_ref.to_uppercase(), 
                       "Round trip failed for {} -> {}", excel_ref, round_trip);
            
            println!("✅ {} -> R{}C{} -> {}", 
                     excel_ref, cell.row, cell.col, round_trip);
        }
    }
    
    #[test]
    fn test_invalid_cells() {
        let invalid_cases = vec![
            "",
            "A",
            "1",
            "A0",      // Row 0 is invalid
            "A1048577", // Beyond Excel limit
            "@1",      // Invalid character
        ];
        
        for case in invalid_cases {
            assert!(CellRef::parse(case).is_err(), 
                    "Should reject invalid cell: '{}'", case);
            println!("✅ Correctly rejected: '{}'", case);
        }
    }
}
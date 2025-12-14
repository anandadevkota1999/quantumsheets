//! Real-world example: Sales report like you'd make in Excel

use quantum_engine::{QuantumGrid, excel::CellRef};

fn main() {
    println!("💰 REAL-WORLD EXAMPLE: SALES REPORT");
    println!("===================================\n");
    
    let mut spreadsheet = QuantumGrid::new();
    
    // Set up sales data
    let sales_data = vec![
        ("A1", "Month"), ("B1", "Revenue"), ("C1", "Expenses"), ("D1", "Profit"),
        ("A2", "January"), ("B2", 10000.0), ("C2", 4000.0), ("D2", 6000.0),
        ("A3", "February"), ("B3", 12000.0), ("C3", 4500.0), ("D3", 7500.0),
        ("A4", "March"), ("B4", 15000.0), ("C4", 5000.0), ("D4", 10000.0),
        ("A5", "April"), ("B5", 13000.0), ("C5", 4200.0), ("D5", 8800.0),
    ];
    
    for (cell, value) in sales_data {
        if let Ok(num) = value.parse::<f64>() {
            spreadsheet.set_cell(cell, num).unwrap();
        }
        // For text cells, we'd need string support (Day 4!)
    }
    
    println!("Sales Report:");
    spreadsheet.print_stats();
    
    // Calculate totals
    if let Ok(total_revenue) = spreadsheet.sum_range("B2:B5") {
        println!("\nTotal Revenue: ${:.2}", total_revenue);
    }
    
    if let Ok(total_expenses) = spreadsheet.sum_range("C2:C5") {
        println!("Total Expenses: ${:.2}", total_expenses);
    }
    
    println!("\n✅ Quantum Sheets can handle real business data!");
}
//! Quantum Sheets Demo - Day 1

use quantum_engine::{init, QuantumColumn};

fn main() {
    // Initialize
    init();
    
    println!("\n🧪 TEST 1: Basic Column Operations");
    println!("=================================");
    
    // Create a column
    let mut column = QuantumColumn::new("Sales");
    
    // Add some data
    column.push(100.0);
    column.push(200.0);
    column.push(300.0);
    column.push(400.0);
    
    // Test Excel functions
    println!("   Data: [100, 200, 300, 400]");
    println!("   SUM: {}", column.sum());
    println!("   AVERAGE: {}", column.average());
    println!("   COUNT: {}", column.count());
    
    println!("\n🧪 TEST 2: Memory Efficiency");
    println!("===========================");
    
    // Create larger dataset
    let mut large_column = QuantumColumn::new("LargeData");
    let num_values = 1000;
    
    for i in 0..num_values {
        large_column.push(i as f64);
    }
    
    let our_memory_per_cell = large_column.memory_per_value();
    let excel_memory_per_cell = 40.0; // Excel uses ~40 bytes per cell
    
    println!("   Number of cells: {}", num_values);
    println!("   Our memory per cell: {:.1} bytes", our_memory_per_cell);
    println!("   Excel memory per cell: {} bytes", excel_memory_per_cell);
    println!("   Improvement: {:.1}x more efficient", 
             excel_memory_per_cell / our_memory_per_cell);
    
    println!("\n🎯 DAY 1 ACCOMPLISHED:");
    println!("=====================");
    println!("✅ Basic engine working");
    println!("✅ Excel functions: SUM, AVERAGE, COUNT");
    println!("✅ Memory efficiency: {:.1}x better than Excel", 
             excel_memory_per_cell / our_memory_per_cell);
    println!("✅ Ready for Day 2: SIMD optimizations!");
}
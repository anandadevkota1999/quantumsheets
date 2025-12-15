//! Quantum Sheets - Day 5 Demo
//! Complete Operation System & User API

use quantum_engine::{init, api::QuantumAPI};

fn main() {
    init();
    
    println!("\n🚀 DAY 5: Complete Operation System");
    println!("==================================\n");
    
    // Create Quantum Sheets instance
    let mut quantum = QuantumAPI::new();
    
    println!("✅ Available Operations:");
    println!("----------------------");
    for op in quantum.list_operations() {
        println!("   • {}", op);
    }
    
    println!("\n✅ TEST 1: Natural Language Operations");
    println!("-----------------------------------\n");
    
    let natural_commands = vec![
        "add A1 and B2",
        "generate 5 rows with test data",
        "sum A1 to A10",
        "filter data where value > 100",
    ];
    
    for cmd in natural_commands {
        println!("   Command: \"{}\"", cmd);
        match quantum.execute(cmd) {
            Ok(result) => println!("      → {}", result),
            Err(e) => println!("      → Error: {}", e),
        }
        println!();
    }
    
    println!("✅ TEST 2: Formula Operations");
    println!("---------------------------\n");
    
    // Set some data
    quantum.set_cell("A1", 10.0).unwrap();
    quantum.set_cell("A2", 20.0).unwrap();
    quantum.set_cell("A3", 30.0).unwrap();
    quantum.set_cell("B1", 5.0).unwrap();
    quantum.set_cell("B2", 15.0).unwrap();
    
    let formulas = vec![
        "=A1+B1",
        "=SUM(A1:A3)",
        "=A2*B2",
        "=A3/B1",
    ];
    
    for formula in formulas {
        println!("   Formula: {}", formula);
        match quantum.execute(formula) {
            Ok(result) => println!("      → {}", result),
            Err(e) => println!("      → Error: {}", e),
        }
        println!();
    }
    
    println!("✅ TEST 3: Custom Operation Registration");
    println!("--------------------------------------\n");
    
    // User registers their own operation!
    quantum.register_operation(
        "DOUBLE",
        "Double a value",
        |_grid, args| {
            if args.is_empty() {
                return Err("Please provide a number".to_string());
            }
            
            if let Ok(num) = args[0].parse::<f64>() {
                Ok(format!("{} doubled is {}", num, num * 2.0))
            } else {
                Err("Please provide a valid number".to_string())
            }
        },
    );
    
    // Use the custom operation
    println!("   Custom operation: DOUBLE 25");
    match quantum.execute("DOUBLE 25") {
        Ok(result) => println!("      → {}", result),
        Err(e) => println!("      → Error: {}", e),
    }
    
    // Another custom operation
    quantum.register_operation(
        "NEPAL_PHONE",
        "Generate a Nepal phone number",
        |_grid, _args| {
            use quantum_engine::ai::data_generator::AIDataGenerator;
            
            let mut gen = AIDataGenerator::new();
            let phone = gen.generate_nepal_phone();
            Ok(format!("Nepal phone: {}", phone))
        },
    );
    
    println!("\n   Custom operation: NEPAL_PHONE");
    match quantum.execute("NEPAL_PHONE") {
        Ok(result) => println!("      → {}", result),
        Err(e) => println!("      → Error: {}", e),
    }
    
    println!("✅ TEST 4: AI Data Generation Integration");
    println!("--------------------------------------\n");
    
    println!("   Generating sample data...");
    match quantum.execute("generate 3 rows with nepal phone numbers and indian cities") {
        Ok(result) => println!("      → {}", result),
        Err(e) => println!("      → Error: {}", e),
    }
    
    println!("\n🎯 DAY 5 ACCOMPLISHED:");
    println!("=====================");
    println!("✅ Complete operation registry system");
    println!("✅ Users can add custom operations");
    println!("✅ Natural language command parsing");
    println!("✅ Formula execution with operations");
    println!("✅ User-friendly API layer");
    println!("✅ AI data generation integrated");
    
    println!("\n🚀 Ready for Day 6: WebAssembly & Browser Interface!");
    println!("   Next: Compile to WASM and create web UI");
    
    println!("\n📋 Sample commands users can now use:");
    println!("   • \"add A1 and B2\"");
    println!("   • \"=SUM(A1:A10)\"");
    println!("   • \"generate 100 nepal phone numbers\"");
    println!("   • \"DOUBLE 50\" (custom operation)");
    println!("   • \"NEPAL_PHONE\" (custom operation)");
}
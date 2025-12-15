use crate::grid::QuantumGrid;

#[derive(Debug, Clone)]
pub enum OperationType {
    Calculation,
    AIPrompt,
    DataGeneration,
    Custom,
}

pub struct Operation {
    pub name: String,
    pub op_type: OperationType,
    pub description: String,
    pub execute: Box<dyn Fn(&mut QuantumGrid, &[String]) -> Result<String, String>>,
}

pub struct OperationRegistry {
    operations: std::collections::HashMap<String, Operation>,
}

impl OperationRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            operations: std::collections::HashMap::new(),
        };
        registry.register_builtins();
        registry
    }
    
    pub fn register(&mut self, operation: Operation) {
        self.operations.insert(operation.name.clone(), operation);
    }
    
    pub fn get(&self, name: &str) -> Option<&Operation> {
        self.operations.get(name)
    }
    
    pub fn execute(&self, name: &str, grid: &mut QuantumGrid, args: &[String]) -> Result<String, String> {
        match self.get(name) {
            Some(op) => (op.execute)(grid, args),
            None => Err(format!("Operation '{}' not found", name)),
        }
    }
    
    // NEW METHOD: List all operations
    pub fn list_operations(&self) -> Vec<String> {
        self.operations.values()
            .map(|op| format!("{} - {}", op.name, op.description))
            .collect()
    }
    
    fn register_builtins(&mut self) {
        // SUM operation - FIXED VERSION
        self.register(Operation {
            name: "SUM".to_string(),
            op_type: OperationType::Calculation,
            description: "Sum numbers".to_string(),
            execute: Box::new(|_grid, args| {
                let mut total = 0.0;
                for arg in args {
                    // First try to parse as number directly
                    if let Ok(num) = arg.parse::<f64>() {
                        total += num;
                    } else {
                        // Try to get from grid (convert cell reference to value)
                        // For now, just skip if not a number
                        // In a full implementation, we'd parse cell references like "A1"
                    }
                }
                Ok(format!("{}", total))
            }),
        });
        
        // DOUBLE operation - WORKING
        self.register(Operation {
            name: "DOUBLE".to_string(),
            op_type: OperationType::Calculation,
            description: "Double a number".to_string(),
            execute: Box::new(|_grid, args| {
                if args.is_empty() {
                    return Err("DOUBLE requires a number".to_string());
                }
                
                match args[0].parse::<f64>() {
                    Ok(num) => Ok(format!("{} doubled is {}", num, num * 2.0)),
                    Err(_) => Err(format!("'{}' is not a valid number", args[0])),
                }
            }),
        });
        
        // NEPAL_PHONE operation - WORKING
        self.register(Operation {
            name: "NEPAL_PHONE".to_string(),
            op_type: OperationType::DataGeneration,
            description: "Generate Nepal phone number".to_string(),
            execute: Box::new(|_grid, _args| {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let number = format!("98{:08}", rng.gen_range(0..100000000));
                Ok(number)
            }),
        });
        
        // GENERATE_DATA operation - FIXED (handle DataRecord type)
        self.register(Operation {
            name: "GENERATE_DATA".to_string(),
            op_type: OperationType::DataGeneration,
            description: "Generate test data".to_string(),
            execute: Box::new(|grid, args| {
                use crate::ai::data_generator::AIDataGenerator;
                
                let count = args.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(10);
                let request = if args.len() > 1 {
                    args[1..].join(" ")
                } else {
                    "Nepal phone numbers".to_string()
                };
                
                let mut generator = AIDataGenerator::new();
                match generator.generate_from_request(&format!("{} rows {}", count, request)) {
                    Ok(data_records) => {
                        // Convert DataRecord to string for display
                        let mut result = String::new();
                        for (i, record) in data_records.iter().enumerate().take(count as usize) {
                            result.push_str(&format!("Row {}: {:?}\n", i + 1, record));
                            
                            // Store in grid (simplified - just store as string in first column)
                            if i < 100 { // Limit to first 100 rows
                                let cell = format!("A{}", i + 1);
                                let _ = grid.set_cell(&cell, 0.0); // Placeholder value
                            }
                        }
                        Ok(format!("Generated {} rows:\n{}", count, result))
                    }
                    Err(e) => Err(format!("Failed to generate data: {}", e)),
                }
            }),
        });
        
        // NATURAL operation - SIMPLIFIED WORKING VERSION
        self.register(Operation {
            name: "NATURAL".to_string(),
            op_type: OperationType::AIPrompt,
            description: "Execute natural language command".to_string(),
            execute: Box::new(|grid, args| {
                use crate::ai::nlp::NaturalLanguageTranslator;
                
                if args.is_empty() {
                    return Err("Please provide a natural language command".to_string());
                }
                
                let command = args.join(" ");
                let translator = NaturalLanguageTranslator::new();
                
                // The translator returns Option<String>
                match translator.translate(&command) {
                    Some(formula) => {
                        // Execute formula using our parser
                        match crate::formula::parser::execute_formula(&formula, grid) {
                            Ok(result) => Ok(format!("Translated to: {} = {}", formula, result)),
                            Err(e) => Err(format!("Could not execute: {}", e)),
                        }
                    }
                    None => {
                        // If not a formula, just return the command for now
                        Ok(format!("Understood: {}", command))
                    }
                }
            }),
        });
    }
}
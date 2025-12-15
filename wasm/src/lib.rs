use wasm_bindgen::prelude::*;
use js_sys::Function;
use quantum_engine::api::QuantumAPI;

// WASM Interface for Quantum Sheets
#[wasm_bindgen]
pub struct QuantumSheetsWasm {
    api: QuantumAPI,
}

#[wasm_bindgen]
impl QuantumSheetsWasm {
    /// Create new Quantum Sheets instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set up panic hook for better error messages in browser
        // We'll add this dependency later if needed
        // console_error_panic_hook::set_once();
        
        Self {
            api: QuantumAPI::new(),
        }
    }
    
    /// Execute a command (formula, natural language, or operation)
    #[wasm_bindgen]
    pub fn execute(&mut self, command: &str) -> Result<String, JsError> {
        match self.api.execute(command) {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Error: {}", e))),
        }
    }
    
    /// Generate data using natural language
    #[wasm_bindgen]
    pub fn generate_data(&mut self, request: &str, count: u32) -> Result<String, JsError> {
        let full_request = format!("{} rows {}", count, request);
        match self.api.execute(&full_request) {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Generation error: {}", e))),
        }
    }
    
    /// Get cell value
    #[wasm_bindgen]
    pub fn get_cell(&self, cell_ref: &str) -> Result<String, JsError> {
        match self.api.get_cell(cell_ref) {
            Ok(value) => Ok(value.to_string()),  // Convert f64 to String
            Err(e) => Err(JsError::new(&format!("Cell error: {}", e))),
        }
    }
    
    /// Set cell value
    #[wasm_bindgen]
    pub fn set_cell(&mut self, cell_ref: &str, value: &str) -> Result<(), JsError> {
        // Parse string to f64
        match value.parse::<f64>() {
            Ok(num) => {
                self.api.set_cell(cell_ref, num)
                    .map_err(|e| JsError::new(&format!("Set cell error: {}", e)))
            }
            Err(_) => {
                // Try to execute as formula if it starts with '='
                if value.starts_with('=') {
                    self.api.execute(value)
                        .map(|_| ())
                        .map_err(|e| JsError::new(&format!("Formula error: {}", e)))
                } else {
                    // Store as string (simplified - convert to 0 for now)
                    self.api.set_cell(cell_ref, 0.0)
                        .map_err(|e| JsError::new(&format!("Set cell error: {}", e)))
                }
            }
        }
    }
    
    /// Register a custom operation
    #[wasm_bindgen]
    pub fn register_operation(
        &mut self,
        name: &str,
        description: &str,
        handler: Function,
    ) -> Result<(), JsError> {
        // Create wrapper that converts JavaScript function to Rust closure
        let closure = move |_grid: &mut quantum_engine::grid::QuantumGrid, args: &[String]| -> Result<String, String> {
            // Prepare arguments for JavaScript
            let js_args = js_sys::Array::new();
            for arg in args {
                js_args.push(&JsValue::from_str(arg));
            }
            
            // Call JavaScript function
            match handler.call1(&JsValue::NULL, &js_args.into()) {
                Ok(result) => {
                    if let Some(string) = result.as_string() {
                        Ok(string)
                    } else {
                        Err("Handler must return a string".to_string())
                    }
                }
                Err(e) => {
                    let error_msg = if let Some(err) = e.as_string() {
                        err
                    } else {
                        "Unknown JavaScript error".to_string()
                    };
                    Err(error_msg)
                }
            }
        };
        
        self.api.register_operation(name, description, closure)
            .map_err(|e| JsError::new(&format!("Operation registration error: {}", e)))
    }
    
    /// Evaluate a formula
    #[wasm_bindgen]
    pub fn evaluate_formula(&mut self, formula: &str) -> Result<String, JsError> {
        match self.api.execute(formula) {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Evaluation error: {}", e))),
        }
    }
    
    /// Translate natural language to formula
    #[wasm_bindgen]
    pub fn translate_natural_language(&mut self, text: &str) -> Result<String, JsError> {
        match self.api.execute(&format!("translate: {}", text)) {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Translation error: {}", e))),
        }
    }
    
    /// Export grid to CSV
    #[wasm_bindgen]
    pub fn export_csv(&mut self) -> Result<String, JsError> {
        match self.api.execute("export csv") {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Export error: {}", e))),
        }
    }
    
    /// Export grid to JSON
    #[wasm_bindgen]
    pub fn export_json(&mut self) -> Result<String, JsError> {
        match self.api.execute("export json") {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Export error: {}", e))),
        }
    }
    
    /// Clear the grid
    #[wasm_bindgen]
    pub fn clear_grid(&mut self) -> Result<(), JsError> {
        match self.api.execute("clear") {
            Ok(_) => Ok(()),
            Err(e) => Err(JsError::new(&format!("Clear error: {}", e))),
        }
    }
    
    /// Get performance stats
    #[wasm_bindgen]
    pub fn get_stats(&mut self) -> Result<String, JsError> {
        match self.api.execute("stats") {
            Ok(result) => Ok(result),
            Err(e) => Err(JsError::new(&format!("Stats error: {}", e))),
        }
    }
}

// Helper function to initialize logging in browser
#[wasm_bindgen(start)]
pub fn start() {
    // We'll add this later when we add the dependency
    // console_error_panic_hook::set_once();
}

// Re-export console.log functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(args: &js_sys::Array);
}
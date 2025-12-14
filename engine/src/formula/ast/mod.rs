//! Abstract Syntax Tree for Excel formulas

use crate::excel::CellRef;

/// Excel formula expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Number literal: 42, 3.14
    Number(f64),
    
    /// Cell reference: A1, B2
    CellRef(CellRef),
    
    /// Cell range: A1:A10
    Range(CellRef, CellRef),
    
    /// Binary operation: A1 + B2
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    
    /// Function call: SUM(A1:A10)
    Function(String, Vec<Expr>),
    
    /// Parentheses: (A1 + B2)
    Group(Box<Expr>),
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Power,     // ^
}

/// Excel function
#[derive(Debug, Clone, PartialEq)]
pub struct Formula {
    pub expression: Expr,
}

impl Formula {
    /// Create a new formula
    pub fn new(expression: Expr) -> Self {
        Self { expression }
    }
    
    /// Parse from Excel formula string (e.g., "=A1+B2")
    pub fn parse(formula: &str) -> Result<Self, String> {
        if !formula.starts_with('=') {
            return Err("Formula must start with '='".to_string());
        }
        
        // Remove leading '='
        let expr_str = &formula[1..];
        
        // For Day 3, we'll implement a simple parser
        // Full nom-based parser will come later
        if expr_str.contains('+') {
            // Simple addition: =A1+B2
            let parts: Vec<&str> = expr_str.split('+').collect();
            if parts.len() == 2 {
                let left = CellRef::parse(parts[0])?;
                let right = CellRef::parse(parts[1])?;
                return Ok(Self::new(Expr::Binary(
                    Box::new(Expr::CellRef(left)),
                    BinaryOp::Add,
                    Box::new(Expr::CellRef(right)),
                )));
            }
        }
        
        Err(format!("Could not parse formula: {}", formula))
    }
    
    /// Convert back to Excel formula string
    pub fn to_excel(&self) -> String {
        format!("={}", self.expr_to_string(&self.expression))
    }
    
        fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::CellRef(cell) => cell.to_excel(),
            Expr::Range(start, end) => format!("{}:{}", start.to_excel(), end.to_excel()),
            Expr::Binary(left, op, right) => {
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Subtract => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                    BinaryOp::Power => "^",
                };
                format!("{} {} {}", 
                    self.expr_to_string(left), 
                    op_str, 
                    self.expr_to_string(right))
            }
            Expr::Function(name, args) => {
                let args_str = args.iter()
                    .map(|arg| self.expr_to_string(arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{}({})", name, args_str)
            }
            Expr::Group(inner) => format!("({})", self.expr_to_string(inner)),
        }
    }
    pub fn parse_advanced(formula: &str) -> Result<Self, String> {
        use crate::formula::parser::parse_formula_safe;
        
        if !formula.starts_with('=') {
            return Err("Formula must start with '='".to_string());
        }
        
        parse_formula_safe(formula)
    }
}
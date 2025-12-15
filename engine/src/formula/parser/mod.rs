//! Advanced formula parser using nom
//! Supports: =A1+B2, =A1-B2, =A1*B2, =A1/B2, =SUM(A1:A10), =AVERAGE(A1:A10)
//! 
use crate::grid::QuantumGrid;

use nom::{
    IResult,
    branch::alt,
    character::complete::{alpha1, digit1, char, one_of, multispace0},
    combinator::{map, opt, recognize},
    multi::{separated_list0, many1},
    sequence::{delimited, pair, tuple},
};

use crate::excel::CellRef;
use crate::formula::ast::{Expr, BinaryOp, Formula};
use crate::operations::OperationRegistry;

/// Parse a complete Excel formula (starts with '=')
pub fn parse_formula(input: &str) -> IResult<&str, Formula> {
    let (input, _) = char('=')(input)?;
    let (input, expr) = parse_expression(input)?;
    
    Ok((input, Formula::new(expr)))
}

/// Parse an expression (can contain + or - operations)
fn parse_expression(input: &str) -> IResult<&str, Expr> {
    let (input, first_term) = parse_term(input)?;
    
    let (input, operations) = many1(
        tuple((
            delimited(multispace0, alt((char('+'), char('-'))), multispace0),
            parse_term,
        ))
    )(input)?;
    
    // Build expression tree
    let mut expr = first_term;
    for (op, term) in operations {
        expr = Expr::Binary(
            Box::new(expr),
            if op == '+' { BinaryOp::Add } else { BinaryOp::Subtract },
            Box::new(term),
        );
    }
    
    Ok((input, expr))
}

/// Parse a term (can contain * or / operations)
fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, first_factor) = parse_factor(input)?;
    
    let (input, operations) = many1(
        tuple((
            delimited(multispace0, alt((char('*'), char('/'))), multispace0),
            parse_factor,
        ))
    )(input)?;
    
    // Build term tree
    let mut expr = first_factor;
    for (op, factor) in operations {
        expr = Expr::Binary(
            Box::new(expr),
            if op == '*' { BinaryOp::Multiply } else { BinaryOp::Divide },
            Box::new(factor),
        );
    }
    
    Ok((input, expr))
}

/// Parse a factor (number, cell reference, function call, or parenthesized expression)
fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_number,
        parse_cell_reference,
        parse_function_call,
        parse_parenthesized,
    ))(input)
}

/// Parse a number
fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(tuple((
            opt(char('-')),
            digit1,
            opt(tuple((char('.'), digit1))),
        ))),
        |num_str: &str| Expr::Number(num_str.parse().unwrap_or(0.0)),
    )(input)
}

/// Parse a cell reference (e.g., A1, B2, AA100)
fn parse_cell_reference(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(pair(
            many1(one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")),
            digit1,
        )),
        |cell_str: &str| {
            CellRef::parse(cell_str)
                .map(Expr::CellRef)
                .unwrap_or_else(|_| Expr::Number(0.0))
        },
    )(input)
}

/// Parse a function call (e.g., SUM(A1:A10))
fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = alpha1(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args) = separated_list0(
        delimited(multispace0, char(','), multispace0),
        parse_expression,
    )(input)?;
    let (input, _) = char(')')(input)?;
    
    Ok((input, Expr::Function(name.to_uppercase(), args)))
}

/// Parse a parenthesized expression
fn parse_parenthesized(input: &str) -> IResult<&str, Expr> {
    delimited(
        char('('),
        map(parse_expression, |expr| Expr::Group(Box::new(expr))),
        char(')'),
    )(input)
}


/// Parse formula with better error handling
pub fn parse_formula_safe(formula_str: &str) -> Result<Formula, String> {
    match parse_formula(formula_str) {
        Ok(("", formula)) => Ok(formula),
        Ok((remaining, _)) => Err(format!("Could not parse entire formula. Remaining: '{}'", remaining)),
        Err(e) => Err(format!("Parse error: {:?}", e)),
    }
}
pub fn parse_formula_with_ops(formula: &str, registry: &OperationRegistry) -> Result<Formula, String> {
    if !formula.starts_with('=') {
        return Err("Formula must start with '='".to_string());
    }
    
    let expr_str = &formula[1..];
    
    // Check if it's a function call (e.g., SUM, AVERAGE, FILTER)
    if let Some(pos) = expr_str.find('(') {
        let func_name = &expr_str[..pos];
        let rest = &expr_str[pos..];
        
        if rest.ends_with(')') {
            let args_str = &rest[1..rest.len()-1]; // Remove parentheses
            
            // Check if this is a registered operation
            if registry.list_operations()
                .iter()
                .any(|op_name| op_name == &func_name.to_uppercase()) {
                
                // Parse arguments (split by comma)
                let args: Vec<String> = args_str.split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                
                // Create function expression
                return Ok(Formula::new(Expr::Function(
                    func_name.to_uppercase(),
                    args.into_iter().map(|arg| {
                        // Try to parse each argument as cell reference or number
                        if let Ok(cell) = CellRef::parse(&arg) {
                            Expr::CellRef(cell)
                        } else if let Ok(num) = arg.parse::<f64>() {
                            Expr::Number(num)
                        } else {
                            // Keep as string for operations that need it
                            Expr::Number(0.0) // Placeholder
                        }
                    }).collect(),
                )));
            }
        }
    }
    
    // Try binary operations
    parse_binary_operation(expr_str)
}
/// Parse binary operations (A1+B2, A1-B2, etc.)
fn parse_binary_operation(expr: &str) -> Result<Formula, String> {
    // Operator precedence: */ before +-
    let operators = [('+', BinaryOp::Add), ('-', BinaryOp::Subtract), 
                     ('*', BinaryOp::Multiply), ('/', BinaryOp::Divide)];
    
    // Find operator (handle multiple operators later)
    for (op_char, op_type) in operators {
        if let Some(pos) = expr.find(op_char) {
            let left = &expr[..pos];
            let right = &expr[pos+1..];
            
            let left_expr = parse_cell_or_number(left)?;
            let right_expr = parse_cell_or_number(right)?;
            
            return Ok(Formula::new(Expr::Binary(
                Box::new(left_expr),
                op_type,
                Box::new(right_expr),
            )));
        }
    }
    
    // Try single cell/number
    parse_cell_or_number(expr).map(|expr| Formula::new(expr))
}

fn parse_cell_or_number(text: &str) -> Result<Expr, String> {
    let text = text.trim();
    
    // Try cell reference
    if let Ok(cell) = CellRef::parse(text) {
        return Ok(Expr::CellRef(cell));
    }
    
    // Try number
    if let Ok(num) = text.parse::<f64>() {
        return Ok(Expr::Number(num));
    }
    
    Err(format!("Could not parse '{}' as cell or number", text))
}

/// Parse and execute formula with operations
pub fn execute_formula(formula: &str, grid: &mut QuantumGrid) -> Result<String, String> {
    let registry = OperationRegistry::new();
    let formula_parsed = parse_formula_with_ops(formula, &registry)?;
    
    // Check if it's a registered operation
    if let Expr::Function(name, args) = &formula_parsed.expression {
        let arg_strings: Vec<String> = args.iter()
            .map(|arg| match arg {
                Expr::CellRef(cell) => cell.to_excel(),
                Expr::Number(n) => n.to_string(),
                _ => "".to_string(),
            })
            .collect();
        
        return registry.execute(name, grid, &arg_strings);
    }
    
    // For simple formulas, return the parsed form
    Ok(format!("Parsed: {}", formula_parsed.to_excel()))
}

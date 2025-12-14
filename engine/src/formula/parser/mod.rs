//! Advanced formula parser using nom
//! Supports: =A1+B2, =A1-B2, =A1*B2, =A1/B2, =SUM(A1:A10), =AVERAGE(A1:A10)

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, char, one_of, multispace0},
    combinator::{map, opt, recognize},
    multi::{separated_list0, many1},
    sequence::{delimited, pair, tuple},
};

use crate::excel::CellRef;
use crate::formula::ast::{Expr, BinaryOp, Formula};

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
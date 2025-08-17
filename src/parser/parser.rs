use pest::Parser;
use pest_derive::Parser;

use super::ast::errors::SyntaxError;

use super::ast::BinaryOperation;

use super::ast::Expression;
use super::ast::core::{Boolean, Integer, Identifier, Operator};

#[derive(Parser)]
#[grammar = "src/parser/nix.pest"]
pub struct NixParser;

pub fn parse(input: &str) -> Result<Vec<Expression>, SyntaxError> {
    let pairs = NixParser::parse(Rule::expr, input)
        .map_err(|e| SyntaxError {message: e.to_string()})?;
    
    let mut expressions = Vec::new();
    for pair in pairs {
        let expr = parse_expr(pair)?;
        expressions.push(expr);
    }
    Ok(expressions)
}

pub fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expression, SyntaxError> {
    match pair.as_rule() {
        Rule::integer => {
            Ok(Expression::Integer(Integer::new(pair.as_str().parse().unwrap())))
        }
        Rule::boolean => {
            Ok(Expression::Boolean(Boolean::new(pair.as_str() == "true")))
        }
        Rule::identifier => {
            Ok(Expression::Identifier(Identifier::new(pair.as_str().to_string())))
        }
        Rule::binary => {
            let mut inner = pair.into_inner();
            let first = parse_expr(inner.next().unwrap());
            let mut expr = first;
            while let Some(op_pair) = inner.next() {
                let op = parse_operator(op_pair);
                let right = parse_expr(inner.next().unwrap());
                let operation = BinaryOperation::new(
                    expr?,
                    op,
                    right?,
                )?;
                expr = Ok(Expression::BinaryOperation(operation));
            }
            expr
        }
        Rule::primary | Rule::expr => {
            parse_expr(pair.into_inner().next().unwrap())
        }
        _ => unreachable!(),
    }
}

fn parse_operator(pair: pest::iterators::Pair<Rule>) -> Operator {
    use super::ast::operators::*;
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::op_add => Operator::Addition(Addition),
        Rule::op_sub => Operator::Subtraction(Subtraction),
        Rule::op_mul => Operator::Multiplication(Multiplication),
        Rule::op_div => Operator::Division(Division),
        Rule::op_eq  => Operator::EqualTo(EqualTo),
        Rule::op_neq => Operator::NotEqualTo(NotEqualTo),
        Rule::op_gt  => Operator::GreaterThan(GreaterThan),
        Rule::op_gte => Operator::GreaterThanOrEqualTo(GreaterThanOrEqualTo),
        Rule::op_lt  => Operator::LessThan(LessThan),
        Rule::op_lte => Operator::LessThanOrEqualTo(LessThanOrEqualTo),
        Rule::op_and => Operator::LogicalAnd(LogicalAnd),
        Rule::op_or  => Operator::LogicalOr(LogicalOr),
        Rule::op_concat => Operator::Concatenation(Concatenation),
        Rule::op_impl   => Operator::Implication(Implication),
        Rule::op_update => Operator::Update(Update),
        Rule::op_not => Operator::Not(Not),
        _ => unreachable!("Unhandled operator {:?}", inner.as_rule()),
    }
}


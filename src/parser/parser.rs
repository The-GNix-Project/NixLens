use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

use crate::parser::grammar;
use grammar::{Addition, Subtraction, Multiplication, Division, EqualTo, NotEqualTo, GreaterThan, GreaterThanOrEqualTo, LessThan, LessThanOrEqualTo, LogicalAnd, LogicalOr, Not, Concatenation, Implication};

#[derive(Parser)]
#[grammar = "src/parser/nix.pest"]
pub struct NixParser;

fn to_span(span: pest::Span) -> grammar::Span {
    let start_pos = span.start_pos();
    let end_pos = span.end_pos();
    grammar::Span {
        start: grammar::Position { 
            line: start_pos.line_col().0 as i64, 
            column: start_pos.line_col().1 as i64 
        },
        end: grammar::Position { 
            line: end_pos.line_col().0 as i64, 
            column: end_pos.line_col().1 as i64 
        },
    }
}

fn parse_identifier(pair: Pair<Rule>) -> grammar::Identifier {
    let span = to_span(pair.as_span());
    grammar::Identifier::new_span(pair.as_str().to_string(), span)
}

fn parse_function_head_destructured_argument(pair: Pair<Rule>) -> Result<grammar::FunctionHeadDestructuredArgument, String> {
    let mut inner = pair.into_inner();
    let ident = inner.next()
        .ok_or_else(|| "Expected identifier in function head argument".to_string())?
        .as_str()
        .to_string();

    let default = if let Some(default_expr_pair) = inner.next() {
        Some(parse_expr(default_expr_pair)?)
    } else {
        None
    };

    Ok(grammar::FunctionHeadDestructuredArgument::new(ident, default))
}

fn parse_function_head(pair: Pair<Rule>) -> Result<grammar::FunctionHead, String> {
    let span = to_span(pair.as_span());

    // We clone the rule value BEFORE calling `.into_inner()`, because `.into_inner()` moves `pair`.
    let rule = pair.as_rule();
    let mut inner = pair.into_inner();

    match rule {
        Rule::function_head_simple => {
            let ident_pair = inner.next()
                .ok_or_else(|| "Expected identifier in function_head_simple".to_string())?;
            let ident = parse_identifier(ident_pair);
            Ok(grammar::FunctionHead::FunctionHeadSimple(
                grammar::FunctionHeadSimple::new_span(ident, span),
            ))
        }
        Rule::function_head_destructured => {
            let ellipsis = false; // You can improve this later based on actual grammar.

            let ident_pair = inner.next()
                .ok_or_else(|| "Expected identifier in function_head_destructured".to_string())?;
            let ident = parse_identifier(ident_pair);

            let mut args = Vec::new();
            for arg_pair in inner {
                args.push(parse_function_head_destructured_argument(arg_pair)?);
            }

            Ok(grammar::FunctionHead::FunctionHeadDestructured(
                grammar::FunctionHeadDestructured::new_span(ellipsis, ident, args, span),
            ))
        }
        _ => Err(format!("Unexpected function_head variant {:?}", rule)),
    }
}

fn parse_expr(pair: Pair<Rule>) -> Result<grammar::Expression, String> {
    let span = to_span(pair.as_span());

    match pair.as_rule() {
        Rule::identifier => Ok(grammar::Expression::Identifier(parse_identifier(pair))),
        Rule::integer => Ok(grammar::Expression::Integer(
            grammar::Integer::new_span(pair.as_str().to_string(), span),
        )),
        Rule::float => Ok(grammar::Expression::Float(
            grammar::Float::new_span(pair.as_str().to_string(), span),
        )),
        Rule::function => {
            let mut inner = pair.into_inner();
            let head_pair = inner.next()
                .ok_or_else(|| "Expected function head".to_string())?;
            let body_pair = inner.next()
                .ok_or_else(|| "Expected function body".to_string())?;

            let head_expr = parse_expr(head_pair)?;
            let body_expr = parse_expr(body_pair)?;

            Ok(grammar::Expression::Function(
                grammar::Function::new_span(head_expr, body_expr, span),
            ))
        }
        Rule::function_head => {
            // You could parse function_head and transform to Expression if needed
            Err("FunctionHead cannot be directly parsed as Expression".to_string())
        }
        Rule::function_application => {
            let mut inner = pair.into_inner();
            let func = parse_expr(inner.next()
                .ok_or_else(|| "Expected function in application".to_string())?)?;
            let args = inner.map(parse_expr).collect::<Result<Vec<_>, _>>()?;

            Ok(grammar::Expression::FunctionApplication(
                grammar::FunctionApplication::new_span(func, args, span),
            ))
        }
        Rule::binary_operation => {
            let mut inner = pair.into_inner();
            let left = parse_expr(inner.next()
                .ok_or_else(|| "Expected left operand".to_string())?)?;
            let op_pair = inner.next()
                .ok_or_else(|| "Expected operator".to_string())?;
            let right = parse_expr(inner.next()
                .ok_or_else(|| "Expected right operand".to_string())?)?;

            let operator = parse_operator(op_pair)?;

            Ok(grammar::Expression::BinaryOperation(
                grammar::BinaryOperation::new_span(left, operator, right, span),
            ))
        }
        Rule::unary_operation => {
            let mut inner = pair.into_inner();
            let op_pair = inner.next()
                .ok_or_else(|| "Expected operator".to_string())?;
            let operand = parse_expr(inner.next()
                .ok_or_else(|| "Expected operand".to_string())?)?;

            let operator = parse_operator(op_pair)?;

            Ok(grammar::Expression::UnaryOperation(
                grammar::UnaryOperation::new_span(operator, operand, span),
            ))
        }
        Rule::if_then_else => {
            let mut inner = pair.into_inner();
            let predicate = parse_expr(inner.next()
                .ok_or_else(|| "Expected predicate".to_string())?)?;
            let then = parse_expr(inner.next()
                .ok_or_else(|| "Expected then branch".to_string())?)?;
            let else_ = parse_expr(inner.next()
                .ok_or_else(|| "Expected else branch".to_string())?)?;

            Ok(grammar::Expression::IfThenElse(
                grammar::IfThenElse::new_span(predicate, then, else_, span),
            ))
        }
        Rule::assert => {
            let mut inner = pair.into_inner();
            let condition = parse_expr(inner.next()
                .ok_or_else(|| "Expected assert condition".to_string())?)?;
            let target = parse_expr(inner.next()
                .ok_or_else(|| "Expected assert target".to_string())?)?;

            Ok(grammar::Expression::Assert(
                grammar::Assert::new_span(condition, target, span),
            ))
        }
        Rule::with_expr => {
            let mut inner = pair.into_inner();
            let expression = parse_expr(inner.next()
                .ok_or_else(|| "Expected with expression".to_string())?)?;
            let target = parse_expr(inner.next()
                .ok_or_else(|| "Expected with target".to_string())?)?;

            Ok(grammar::Expression::With(
                grammar::With::new_span(expression, target, span),
            ))
        }
        Rule::let_in => {
            let mut inner = pair.into_inner();
            let mut bindings = Vec::new();
            let mut target = None;
            
            for item in inner {
                if item.as_rule() == Rule::binding {
                    bindings.push(parse_binding(item)?);
                } else if item.as_rule() == Rule::expr {
                    target = Some(parse_expr(item)?);
                }
            }
            
            let target = target.ok_or_else(|| "Expected target expression in let-in".to_string())?;
            
            Ok(grammar::Expression::LetIn(
                grammar::LetIn::new_span(bindings, target, span),
            ))
        }
        Rule::list => {
            let elements = pair.into_inner()
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(grammar::Expression::List(
                grammar::List::new_span(elements, span),
            ))
        }
        Rule::attr_set | Rule::rec_attr_set => {
            let recursive = pair.as_rule() == Rule::rec_attr_set;
            let bindings = pair.into_inner()
                .filter(|p| p.as_rule() == Rule::binding)
                .map(parse_binding)
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(grammar::Expression::Map(
                grammar::Map::new_span(recursive, bindings, span),
            ))
        }
        Rule::path => {
            let parts = pair.into_inner()
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(grammar::Expression::Path(
                grammar::Path::new_span(parts, span),
            ))
        }
        Rule::search_path => {
            Ok(grammar::Expression::SearchNixPath(
                grammar::SearchNixPath::new_span(pair.as_str().to_string(), span),
            ))
        }
        Rule::uri => {
            Ok(grammar::Expression::Uri(
                grammar::Uri::new_span(pair.as_str().to_string(), span),
            ))
        }
        Rule::string => {
            let parts = pair.into_inner()
                .map(|part| match part.as_rule() {
                    Rule::string_content => {
                        Ok(grammar::Expression::PartRaw(
                            grammar::PartRaw::new_span(part.as_str().to_string(), to_span(part.as_span()))
                        ))
                    }
                    Rule::interpolation => {
                        let expr = part.into_inner().next()
                            .ok_or_else(|| "Expected expression in interpolation".to_string())?;
                        parse_expr(expr).map(|e| 
                            grammar::Expression::PartInterpolation(
                                grammar::PartInterpolation::new_span(e, to_span(part.as_span()))
                        ))
                    }
                    _ => Err(format!("Unexpected string part: {:?}", part.as_rule())),
                })
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(grammar::Expression::NixString(
                grammar::NixString::new_span(parts, span),
            ))
        }
        Rule::indented_string => {
            let parts = pair.into_inner()
                .map(|part| match part.as_rule() {
                    Rule::indented_string_content => {
                        Ok(grammar::Expression::PartRaw(
                            grammar::PartRaw::new_span(part.as_str().to_string(), to_span(part.as_span()))
                        ))
                    }
                    Rule::interpolation => {
                        let expr = part.into_inner().next()
                            .ok_or_else(|| "Expected expression in interpolation".to_string())?;
                        parse_expr(expr).map(|e| 
                            grammar::Expression::PartInterpolation(
                                grammar::PartInterpolation::new_span(e, to_span(part.as_span()))
                            ))
                    }
                    _ => Err(format!("Unexpected indented string part: {:?}", part.as_rule())),
                })
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(grammar::Expression::IndentedString(
                grammar::IndentedString::new_span(parts, span),
            ))
        }
        Rule::property_access => {
            let mut inner = pair.into_inner();
            let expression = parse_expr(inner.next()
                .ok_or_else(|| "Expected expression in property access".to_string())?)?;
            
            let attribute_path = inner.next()
                .ok_or_else(|| "Expected attribute path".to_string())?
                .into_inner()
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            
            let default = inner.next().map(|d| parse_expr(d)).transpose()?;
            
            Ok(grammar::Expression::PropertyAccess(
                grammar::PropertyAccess::new_span(expression, attribute_path, default, span),
            ))
        }
        Rule::has_attribute => {
            let mut inner = pair.into_inner();
            let expression = parse_expr(inner.next()
                .ok_or_else(|| "Expected expression in has_attribute".to_string())?)?;
            
            let attribute_path = inner.next()
                .ok_or_else(|| "Expected attribute path".to_string())?
                .into_inner()
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            
            Ok(grammar::Expression::HasAttribute(
                grammar::HasAttribute::new_span(expression, attribute_path, span),
            ))
        }
        _ => Err(format!("Unimplemented parse for rule {:?}", pair.as_rule())),
    }
}

fn parse_binding(pair: Pair<Rule>) -> Result<grammar::Expression, String> {
    let span = to_span(pair.as_span());
    
    match pair.as_rule() {
        Rule::inherit_attr => {
            let mut inner = pair.into_inner();
            let from_ = if let Some(expr_pair) = inner.next() {
                if expr_pair.as_rule() == Rule::expr {
                    Some(parse_expr(expr_pair)?)
                } else {
                    None
                }
            } else {
                None
            };
            
            let attributes = inner.map(|attr| {
                let attr_span = to_span(attr.as_span());
                grammar::Expression::Identifier(grammar::Identifier::new_span(attr.as_str().to_string(), attr_span))
            }).collect::<Vec<_>>();
            
            if attributes.is_empty() {
                return Err("Expected at least one attribute in inherit".to_string());
            }
            
            // For simplicity, we'll create multiple bindings if there are multiple attributes
            // This might need to be adjusted based on your AST design
            let bindings = attributes.into_iter().map(|attr| {
                grammar::Expression::BindingInherit(
                    grammar::BindingInherit::new_span(
                        from_.clone().map(Clone::clone),
                        attr,
                        span.clone()
                    )
                )
            }).collect::<Vec<_>>();
            
            // If there's only one, return it directly
            if bindings.len() == 1 {
                Ok(bindings.into_iter().next().unwrap())
            } else {
                // For multiple, create a block expression (you might need to create this in your AST)
                // For now, just return the first one as a placeholder
                Ok(bindings.into_iter().next().unwrap())
            }
        }
        Rule::attr => {
            let mut inner = pair.into_inner();
            let key_pair = inner.next()
                .ok_or_else(|| "Expected key in attribute".to_string())?;
            let value = parse_expr(inner.next()
                .ok_or_else(|| "Expected value in attribute".to_string())?)?;
            
            let key = match key_pair.as_rule() {
                Rule::identifier => grammar::Expression::Identifier(parse_identifier(key_pair)),
                Rule::string => {
                    let content = key_pair.as_str().to_string();
                    let span = to_span(key_pair.as_span());
                    grammar::Expression::PartRaw(grammar::PartRaw::new_span(content, span))
                }
                _ => return Err(format!("Unexpected key type: {:?}", key_pair.as_rule())),
            };
            
            Ok(grammar::Expression::BindingKeyValue(
                grammar::BindingKeyValue::new_span(key, value)
            ))
        }
        _ => Err(format!("Unsupported binding type: {:?}", pair.as_rule())),
    }
}

fn parse_operator(pair: Pair<Rule>) -> Result<grammar::Operator, String> {
    match pair.as_str() {
        "+" => Ok(grammar::Operator::Addition(grammar::Addition)),
        "-" => Ok(grammar::Operator::Subtraction(grammar::Subtraction)),
        "*" => Ok(grammar::Operator::Multiplication(grammar::Multiplication)),
        "/" => Ok(grammar::Operator::Division(grammar::Division)),
        "==" => Ok(grammar::Operator::EqualTo(grammar::EqualTo)),
        "!=" => Ok(grammar::Operator::NotEqualTo(grammar::NotEqualTo)),
        ">" => Ok(grammar::Operator::GreaterThan(grammar::GreaterThan)),
        ">=" => Ok(grammar::Operator::GreaterThanOrEqualTo(grammar::GreaterThanOrEqualTo)),
        "<" => Ok(grammar::Operator::LessThan(grammar::LessThan)),
        "<=" => Ok(grammar::Operator::LessThanOrEqualTo(grammar::LessThanOrEqualTo)),
        "&&" => Ok(grammar::Operator::LogicalAnd(grammar::LogicalAnd)),
        "||" => Ok(grammar::Operator::LogicalOr(grammar::LogicalOr)),
        "!" => Ok(grammar::Operator::Not(grammar::Not)),
        "++" => Ok(grammar::Operator::Concatenation(grammar::Concatenation)),
        "->" => Ok(grammar::Operator::Implication(grammar::Implication)),
        ":=" => Ok(grammar::Operator::Update(grammar::Update)),
        "-" => Ok(grammar::Operator::Negate(grammar::Negate)), // Unary minus
        _ => Err(format!("Unknown operator: {}", pair.as_str())),
    }
}

pub fn parse_nix(input: &str) -> Result<grammar::Expression, String> {
    let mut pairs = NixParser::parse(Rule::file, input)
        .map_err(|e| format!("Parse error: {}", e))?;

    let pair = pairs
        .next()
        .ok_or_else(|| "Expected expression at root".to_string())?;

    if pair.as_rule() != Rule::file {
        return Err(format!("Expected Rule::file, got {:?}", pair.as_rule()));
    }

    // Extract the inner expr from Rule::file
    let inner_pair = pair.into_inner().next()
        .ok_or_else(|| "Expected expr inside file".to_string())?;

    parse_expr(inner_pair)
}



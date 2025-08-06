// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of GNix.
// GNix - The Graphical Nix Project
// -----------------------------------------------------------------------------------------|
// GNix is free software: you can redistribute it and/or modify                             |
// it under the terms of the GNU General Public License as published by                     |
// the Free Software Foundation, either version 3 of the License, or any later version.     |
//                                                                                          |
// GNix is distributed in the hope that it will be useful,                                  |
// but WITHOUT ANY WARRANTY; without even the implied warranty of                           |
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                            |
// GNU General Public License for more details.                                             |
//                                                                                          |
// You should have received a copy of the GNU General Public License                        |
// along with GNix.  If not, see <https://www.gnu.org/licenses/>.                           |
// -----------------------------------------------------------------------------------------|

use parser::grammar::RenderError;
use parser::parser::parse_nix;

mod parser;

fn main() ->Result<(), RenderError> {
    use parser::grammar::*;
    
    // Test Integer
    let int_expr = Expression::Integer(Integer::new("42".to_string()));
    println!("Integer render: {}", int_expr.render()?);

    // Test Float
    let float_expr = Expression::Float(Float::new("3.14".to_string()));
    println!("Float render: {}", float_expr.render()?);

    // Test Identifier
    let id_expr = Expression::Identifier(Identifier::new("myVar".to_string()));
    println!("Identifier render: {}", id_expr.render()?);

    // Test Operators directly
    let add_op = Operator::Addition(Addition);
    println!("Addition operator render: {}", add_op.render());

    let eq_op = Operator::EqualTo(EqualTo);
    println!("EqualTo operator render: {}", eq_op.render());

    let not_op = Operator::Not(Not);
    println!("Not operator render: {}", not_op.render());

    let struc_func_head = FunctionHeadDestructuredArgument::new(String::from("name"), Some(Expression::Integer(Integer::new(2.to_string()))));
    println!("Structured Function Head render: {}", struc_func_head.render()?);
    Ok(())
}

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

use parser::ast::core::*;
use parser::ast::expr::*;

use crate::parser::ast::operators::Addition;

mod parser;

fn main() {
    println!("GNix NixLens Parser Module");

    // Example usage of the Expression enum
    let operation = BinaryOperation::new(
        Expression::Integer(Integer::new(42.to_string())),
        operators::Operator::Addition(Addition),
        Expression::Integer(Integer::new(58.to_string())),
    );
    let expr = match operation {
        Ok(op) => Expression::BinaryOperation(op),
        Err(e) => {
            eprintln!("Error creating binary operation: {}", e.message);
            return;
        }
    };
    println!("{}", expr.render().unwrap());
}

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
mod lsp;

use lsp::{Connection, DidOpenTextDocument, HoverRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = Connection::new().await?;
    connection.initialize().await?;

    let mut cursor = connection.cursor("file:///example.nix".into());
    
    // Send text document open notification
    cursor.notify(DidOpenTextDocument {
        text: "let x = 1; in x".into(),
        language_id: "nix".into(),
    }).await?;

    // Get hover information at position (0, 5)
    let hover = cursor.execute(HoverRequest {
        position: (0, 5)
    }).await?;

    println!("Hover info: {:?}", hover);
    
    Ok(())
}
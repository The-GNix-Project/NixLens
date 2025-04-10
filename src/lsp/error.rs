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
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LspError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("LSP protocol error: {0}")]
    Protocol(String),

    #[error("Server error: {0}")]
    Server(String),
    
    #[error("Request timeout")]
    Timeout,
}
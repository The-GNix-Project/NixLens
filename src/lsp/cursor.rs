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
use serde_json::Value;
use super::{LspError, LspTransport, requests::LspRequest, LspNotification};

pub struct Cursor<'a> {
    transport: &'a mut LspTransport,
    uri: String,
    version: u64,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(transport: &'a mut LspTransport, uri: String) -> Self {
        Self {
            transport,
            uri,
            version: 0,
        }
    }

    pub async fn execute<R: LspRequest>(&mut self, request: R) -> Result<R::Response, LspError> {
        let params = request.build_params(self.uri.clone(), self.version);
        let response = self.transport.send_request(R::METHOD, params).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn notify<N: LspNotification>(&mut self, notification: N) -> Result<(), LspError> {
        let params = notification.build_params(self.uri.clone(), self.version);
        self.transport.send_notification(N::METHOD, params).await
    }
}

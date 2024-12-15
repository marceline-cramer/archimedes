// Copyright (c) 2024 Marceline Cramer.
// SPDX-License-Identifier: AGPL-3.0-or-later
//
// This file is part of Archimedes.
//
// Archimedes is free software: you can redistribute it and/or modify it under
// the terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// Archimedes is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Archimedes. If not, see <https://www.gnu.org/licenses/>.

use archimedes::parser::Module;
use tower_lsp::{LspService, Server};

pub mod lsp;

#[tokio::main]
async fn main() {
    let args: Vec<_> = std::env::args().collect();

    if let Some("parse") = args.get(1).map(String::as_str) {
        let src = std::fs::read_to_string(&args[2]).unwrap();
        let module = Module::new(&src);
        module.items();
        return;
    }

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(lsp::LspBackend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}

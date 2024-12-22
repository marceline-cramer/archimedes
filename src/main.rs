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

use std::path::PathBuf;

use archimedes::{
    frontend::{
        dataflow::{frontend_worker, FrontendResultKind, FrontendUpdate},
        parser::Module,
        span::{MapSpan, Point},
    },
    utils::run_dataflow,
};
use clap::{Parser, Subcommand};
use tower_lsp::{LspService, Server};
use url::Url;

pub mod lsp;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Executes a Fulcrum file and displays its decision selection.
    Run { path: PathBuf },

    /// Parses a Fulcrum file and dumps the AST debug-print to stderr.
    Parse { path: PathBuf },

    /// Run the Fulcrum language server over stdio.
    Lsp,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run { path } => command_run(path),
        Command::Parse { path } => {
            let src = std::fs::read_to_string(path).unwrap();
            let module = Module::new(&src);
            eprintln!("{:#?}", module.items().map_span(&mut |_| ()));
        }
        Command::Lsp => {
            let stdin = tokio::io::stdin();
            let stdout = tokio::io::stdout();
            let (service, socket) = LspService::new(lsp::LspBackend::new);
            Server::new(stdin, stdout, socket).serve(service).await;
        }
    }
}

pub fn command_run(path: PathBuf) {
    let url = Url::from_file_path(path.canonicalize().expect("failed to canonicalize path"))
        .expect("failed to create URI to file path");

    let src = std::fs::read_to_string(&path).unwrap();
    let module = Module::new(&src);
    let (update_tx, update_rx) = flume::unbounded();
    let result_rx = run_dataflow(update_rx, frontend_worker);

    let updates = module
        .items()
        .into_iter()
        .map(|item| item.map_span(&mut |span| (url.clone(), span)))
        .map(|item| FrontendUpdate::Item(url.clone(), item, true))
        .collect();

    update_tx
        .send(updates)
        .expect("failed to send item updates to dataflow");

    let results = result_rx
        .recv()
        .expect("failed to receive dataflow results");

    let filename = path
        .file_name()
        .expect("failed to get file name")
        .to_string_lossy()
        .to_string();

    let src = ariadne::Source::from(src);
    for (result_url, result) in results {
        match result {
            FrontendResultKind::Diagnostic(d) => {
                d.map_span(&mut |(url, span)| {
                    let map_point =
                        |point: Point| src.line(point.row).unwrap().offset() + point.col;

                    let span = map_point(span.start)..map_point(span.end);

                    let id = if url == result_url {
                        filename.clone()
                    } else {
                        result_url.to_string()
                    };

                    (id, span)
                })
                .to_ariadne()
                .print((filename.clone(), src.clone()))
                .expect("failed to print report");
            }
            _ => {}
        }
    }
}

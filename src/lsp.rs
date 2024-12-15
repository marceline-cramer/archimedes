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

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use archimedes::{
    frontend::{
        dataflow::{frontend_worker, FrontendResultKind, FrontendUpdate},
        parser::Module,
        span::Span,
        types::ModuleItem,
    },
    utils::run_dataflow,
};
use flume::Sender;
use tokio::sync::Mutex;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{OneOf, *},
    Client, LanguageServer,
};

type FileMap = Arc<Mutex<HashMap<Url, File>>>;

pub struct LspBackend {
    pub client: Client,
    pub files: FileMap,
    pub update_tx: Sender<Vec<FrontendUpdate>>,
}

impl LspBackend {
    pub fn new(client: Client) -> Self {
        let files = FileMap::default();
        let (update_tx, update_rx) = flume::unbounded();

        let result_rx = run_dataflow(update_rx, frontend_worker);
        eprintln!("started dataflow");

        tokio::spawn({
            let files = files.clone();
            async move {
                let mut file_results: HashMap<Url, Vec<_>> = HashMap::new();
                while let Ok(results) = result_rx.recv_async().await {
                    for (url, result) in results {
                        file_results.entry(url).or_default().push(result);
                    }

                    let mut files = files.lock().await;
                    for (url, results) in file_results.clone() {
                        if let Some(file) = files.get_mut(&url) {
                            file.on_results(results).await;
                        }
                    }

                    // ensure that empty results are still flushed to corresponding files
                    file_results.retain(|_url, results| !results.is_empty());
                    file_results.values_mut().for_each(Vec::clear);
                }
            }
        });

        Self {
            client,
            files,
            update_tx,
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LspBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                inlay_hint_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        // TODO: incremental update
                        change: Some(TextDocumentSyncKind::FULL),
                        save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                            include_text: Some(true),
                        })),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let file = File::new(self.client.clone(), self.update_tx.clone(), params).await;
        self.files.lock().await.insert(uri, file);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.files.lock().await.remove(&params.text_document.uri);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut files = self.files.lock().await;

        let Some(file) = files.get_mut(&params.text_document.uri) else {
            return;
        };

        file.on_change(TextDocumentItem {
            language_id: "fulcrum".to_string(),
            text: params.content_changes[0].text.clone(),
            uri: params.text_document.uri,
            version: params.text_document.version,
        })
        .await;

        self.client
            .log_message(MessageType::INFO, "File changed")
            .await;
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        let mut files = self.files.lock().await;

        let Some(file) = files.get_mut(&params.text_document.uri) else {
            return Ok(None);
        };

        eprintln!("inlay hints: {:?}", file.inlay_hints);

        Ok(Some(file.inlay_hints.clone()))
    }
}

pub struct File {
    client: Client,
    module: Module,
    url: Url,
    old_items: HashSet<ModuleItem<Span, String, String>>,
    update_tx: Sender<Vec<FrontendUpdate>>,
    inlay_hints: Vec<InlayHint>,
}

impl File {
    pub async fn new(
        client: Client,
        update_tx: Sender<Vec<FrontendUpdate>>,
        params: DidOpenTextDocumentParams,
    ) -> Self {
        let mut file = File {
            client,
            module: Module::new(&params.text_document.text),
            url: params.text_document.uri.clone(),
            old_items: HashSet::new(),
            inlay_hints: Vec::new(),
            update_tx,
        };

        file.on_change(TextDocumentItem {
            language_id: "fulcrum".to_string(),
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await;

        file
    }

    async fn on_change(&mut self, params: TextDocumentItem) {
        self.module.update(&params.text);

        let items = self.module.items();
        let new_items: HashSet<_> = items.iter().cloned().collect();

        let removed = self
            .old_items
            .difference(&new_items)
            .map(|removed| (removed, false));

        let added = new_items
            .difference(&self.old_items)
            .map(|added| (added, true));

        let mut updates = Vec::new();
        for (el, added) in removed.chain(added) {
            updates.push(FrontendUpdate::Item(self.url.clone(), el.clone(), added));
        }

        if !updates.is_empty() {
            let _ = self.update_tx.send(updates);
        }

        self.old_items = new_items;
    }

    async fn on_results(&mut self, results: Vec<FrontendResultKind>) {
        let mut diagnostics = Vec::new();
        self.inlay_hints.clear();

        eprintln!("results {:?}", results);

        for result in results {
            match result {
                FrontendResultKind::Diagnostic(diagnostic) => diagnostics.push(diagnostic.to_lsp()),
                FrontendResultKind::Hover(_) => {}
                FrontendResultKind::InlayHint(hint) => self.inlay_hints.push(hint.to_lsp()),
            }
        }

        self.client
            .publish_diagnostics(self.url.clone(), diagnostics, None)
            .await;

        let _ = self.client.inlay_hint_refresh().await;
    }
}

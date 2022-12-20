use crate::{global_state::GlobalState, ide::Lines, Result};
use crossbeam_channel::select;
use lsp_server::{Connection, Message, Notification, Request};
use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use star_syntax::{parse_file, SyntaxElement, SyntaxNode, WalkEvent};
use std::sync::Arc;

#[derive(Debug)]
pub enum Task {
    Diagnostics(Vec<(Url, Vec<Diagnostic>)>),
}

#[derive(Debug)]
enum Event {
    Lsp(lsp_server::Message),
    Task(Task),
}

pub fn main_loop(connection: Connection) -> Result<()> {
    GlobalState::new(connection).run()
}

impl GlobalState {
    fn set_document_content(&mut self, uri: Url, text: String) {
        let mut content = self.content.write().unwrap();
        let lines = Lines::from_str(&text);
        content.insert(uri.clone(), Arc::new((text, lines)));
        self.changes.insert(uri);
    }

    fn did_change_text_document(&mut self, mut params: lsp_types::DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.pop() {
            self.set_document_content(params.text_document.uri, change.text);
        }
    }

    fn did_open_text_document(&mut self, params: lsp_types::DidOpenTextDocumentParams) {
        let mut content = self.content.write().unwrap();
        let lines = Lines::from_str(&params.text_document.text);
        content.insert(
            params.text_document.uri.clone(),
            Arc::new((params.text_document.text, lines)),
        );
        self.changes.insert(params.text_document.uri.clone());
        self.subscriptions.add(params.text_document.uri);
    }

    fn did_close_text_document(&mut self, params: lsp_types::DidCloseTextDocumentParams) {
        self.subscriptions.remove(&params.text_document.uri);
    }

    fn recv(&self) -> Option<Event> {
        select! {
            recv(self.connection.receiver) -> msg => {
                msg.ok().map(Event::Lsp)
            }
            recv(self.task_pool.receiver) -> task => {
                Some(Event::Task(task.unwrap()))
            }
        }
    }

    fn run(mut self) -> Result<()> {
        // let params: InitializeParams = serde_json::from_value(raw_params).unwrap();
        eprintln!("starting example main loop");

        while let Some(event) = self.recv() {
            if let Event::Lsp(Message::Request(ref req)) = event {
                if self.connection.handle_shutdown(req)? {
                    return Ok(());
                }
            }
            self.handle_event(event)?;
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Lsp(msg) => match msg {
                Message::Request(req) => {
                    eprintln!("got request: {:?}", req);
                }
                Message::Response(resp) => {
                    eprintln!("got response: {:?}", resp);
                }
                Message::Notification(not) => {
                    if let Some(params) =
                        cast_notification::<lsp_types::notification::DidChangeTextDocument>(&not)
                    {
                        self.did_change_text_document(params);
                    } else if let Some(params) =
                        cast_notification::<lsp_types::notification::DidOpenTextDocument>(&not)
                    {
                        self.did_open_text_document(params);
                    } else if let Some(params) =
                        cast_notification::<lsp_types::notification::DidCloseTextDocument>(&not)
                    {
                        self.did_close_text_document(params);
                    }
                }
            },
            Event::Task(task) => {
                self.handle_task(task);
            }
        }

        eprintln!("main loop turn");

        // Check changed files.
        let changes = self.take_changes();

        eprintln!("processing changes: {}", changes.len());

        for change in changes {
            // TODO:
            // For each file change:
            //   - update file content map
            // Spawn task:
            //   - For each subscribed file
            //     - Recalculate diagnostics
            //     - update_diagnostics()
        }

        let subscriptions_changed = self.subscriptions.take_changed();

        // Process diagnostic changes.
        let diagnostic_changes = self.take_diagnostic_changes();
        for url in diagnostic_changes {
            let diagnostic = self.latest_diagnostics.get(&url).cloned().unwrap();
            self.send_notification::<lsp_types::notification::PublishDiagnostics>(
                lsp_types::PublishDiagnosticsParams::new(url, diagnostic, None),
            );
        }

        Ok(())
    }

    fn handle_task(&mut self, task: Task) {
        match task {
            Task::Diagnostics(diagnostics) => {
                for (url, file_diagnostics) in diagnostics {
                    self.process_incoming_diagnostics(url, file_diagnostics);
                }
            }
        }
    }

    fn update_diagnostics(&self) {
        self.task_pool.spawn(|| Task::Diagnostics(Vec::new()))
        // Spawn task on thread pool.
    }
}

fn cast_request<R>(req: &Request) -> Option<R::Params>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    if req.method == R::METHOD {
        let params = serde_json::from_value(req.params.clone()).unwrap();
        Some(params)
    } else {
        None
    }
}

fn cast_notification<R>(not: &Notification) -> Option<R::Params>
where
    R: lsp_types::notification::Notification,
    R::Params: serde::de::DeserializeOwned,
{
    if not.method == R::METHOD {
        let params = serde_json::from_value(not.params.clone()).unwrap();
        Some(params)
    } else {
        None
    }
}

fn render(syntax: SyntaxNode) -> String {
    let mut buf = String::new();
    let mut indent = 0;
    let mut start = 0;
    let mut pos = 0;

    for event in syntax.preorder_with_tokens() {
        match event {
            WalkEvent::Enter(node) => {
                let text = match &node {
                    SyntaxElement::Node(it) => it.text().to_string(),
                    SyntaxElement::Token(it) => {
                        start = pos;
                        pos += it.text().len();
                        it.text().to_string()
                    }
                };
                buf.push_str(&format!(
                    "{:indent$}{:?}@{}..{} {:?}\n",
                    " ",
                    node.kind(),
                    start,
                    pos,
                    text,
                    indent = indent
                ));
                indent += 2;
            }
            WalkEvent::Leave(_) => indent -= 2,
        }
    }

    buf
}

// fn print(indent: usize, element: SyntaxElement) {
//     let kind: SyntaxKind = element.kind().into();
//     eprint!("{:indent$}", "", indent = indent);
//     match element {
//         SyntaxElement::Node(node) => {
//             eprintln!("- {:?}", kind);
//             for child in node.children_with_tokens() {
//                 print(indent + 2, child);
//             }
//         }
//         SyntaxElement::Token(token) => eprintln!("- {:?} {:?}", token.text(), kind),
//     }
// }

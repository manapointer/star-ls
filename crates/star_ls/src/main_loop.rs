use crate::{global_state::GlobalState, ide::Lines, Result};
use crossbeam_channel::select;
use lsp_server::{Connection, Message, Notification, Request};
use lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};
use std::sync::Arc;

#[derive(Debug)]
pub enum Task {
    Diagnostics,
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
        self.changes.insert(params.text_document.uri);
    }

    fn recv(&self) -> Option<Event> {
        select! {
            recv(self.connection.receiver) -> msg => {
                msg.ok().map(Event::Lsp)
            }
            recv(self.thread_pool_receiver) -> task => {
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
                    }
                }
            },
            Event::Task(task) => {}
        }

        eprintln!("main loop turn");

        // Check changed files.
        let changes = self.take_changes();
        eprintln!("processing changes: {}", changes.len());

        for change in changes {
            // Calculate and update diagnostics.
            let content = self.content.read().unwrap();
            let (ref text, ref lines) = **(content.get(&change).unwrap());

            // let mut diagnostics = Vec::new();

            let mut pos = 0;

            // let tokens =
            //     Lexer::from_str(text).map(|LexerReturn(Token { kind, len }, diagnostic)| {
            //         if let Some(diagnostic) = diagnostic {
            //             diagnostics.push(Diagnostic {
            //                 severity: Some(DiagnosticSeverity::ERROR),
            //                 range: Range {
            //                     start: lines.line_num_and_col(pos),
            //                     end: lines.line_num_and_col(pos + len),
            //                 },
            //                 message: diagnostic,
            //                 ..Default::default()
            //             });
            //         }
            //         pos += len;
            //         (kind, len)
            //     });

            // let parser = Parser::new(tokens, text);
            // let parse = parser.parse();

            // let diagnostics = diagnostics
            //     .into_iter()
            //     .chain(parse.errors.into_iter().map(|(message, pos)| {
            //         let pos = lines.line_num_and_col(pos);
            //         Diagnostic {
            //             severity: Some(DiagnosticSeverity::ERROR),
            //             range: Range {
            //                 start: pos,
            //                 end: pos,
            //             },
            //             message,
            //             ..Default::default()
            //         }
            //     }))
            //     .collect::<Vec<_>>();

            drop(content);

            // self.send_notification::<lsp_types::notification::PublishDiagnostics>(
            //     lsp_types::PublishDiagnosticsParams::new(change, diagnostics, None),
            // );
        }

        // Process diagnostic changes.
        // let diagnostic_changes = self.take_diagnostic_changes();
        // for url in diagnostic_changes {
        //     self.send_notification::<lsp_types::notification::PublishDiagnostics>(
        //         lsp_types::PublishDiagnosticsParams::new(url, vec![], None),
        //     );
        // }

        Ok(())
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

use star_syntax::{parse_file, Parse};
use std::collections::HashMap;

#[salsa::jar(db = Db)]
pub struct Jar(lines, parse, File);

pub trait Db: salsa::DbWithJar<Jar> {}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
    files: HashMap<String, File>,
}

impl Database {
    pub fn set_file_text(&mut self, path: String, text: String) {
        match self.files.get_mut(&path) {
            Some(file) => {
                file.set_text(self).to(text);
            }
            None => {
                self.files.insert(path, File::new(self, text));
            }
        }
    }
}

impl Db for Database {}

impl salsa::Database for Database {}

#[salsa::input]
pub struct File {
    #[return_ref]
    text: String,
}

#[salsa::tracked]
pub fn parse(db: &dyn Db, file: File) -> Parse {
    parse_file(file.text(db))
}

#[salsa::tracked]
pub fn lines(db: &dyn Db, file: File) {}

// let content = self.content.read().unwrap();
// let (ref text, ref lines) = **(content.get(&change).unwrap());

// let parse = parse_file(text);

// eprintln!("{}", render(parse.syntax()));

// let diagnostics = parse
//     .errors()
//     .iter()
//     .map(|(message, pos)| {
//         let pos = lines.line_num_and_col(*pos);
//         Diagnostic {
//             severity: Some(DiagnosticSeverity::ERROR),
//             range: Range {
//                 start: pos,
//                 end: pos,
//             },
//             message: message.clone(),
//             ..Default::default()
//         }
//     })
//     .collect();

// drop(content);

// self.send_notification::<lsp_types::notification::PublishDiagnostics>(
//     lsp_types::PublishDiagnosticsParams::new(change, diagnostics, None),
// );

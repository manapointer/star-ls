use lsp_types::{ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind};

mod errors;
mod global_state;
mod main_loop;
mod subscriptions;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub use main_loop::main_loop;

#[salsa::input]
pub(crate) struct Document {
    text: String,
}

#[salsa::jar(db = Db)]
pub(crate) struct Jar(Document);

trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

pub fn server_capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        ..Default::default()
    }
}

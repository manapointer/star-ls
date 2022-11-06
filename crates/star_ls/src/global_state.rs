use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Connection, Message, Notification};
use lsp_types::Url;
use std::{
    collections::{HashMap, HashSet},
    mem,
    sync::{Arc, RwLock},
};

use crate::{db::Database, ide::Lines, main_loop::Task};

pub(crate) struct GlobalState {
    /// Changes to document contents.
    pub(crate) changes: HashSet<Url>,
    pub(crate) connection: Connection,
    pub(crate) content: RwLock<HashMap<Url, Arc<(String, Lines)>>>,
    pub(crate) db: Database,

    /// Changes to calculated diagnostics.
    pub(crate) diagnostic_changes: HashSet<Url>,
    pub(crate) diagnostics_for_url: HashMap<Url, Vec<lsp_types::Diagnostic>>,
    pub(crate) thread_pool: rayon::ThreadPool,
    pub(crate) thread_pool_sender: Sender<Task>,
    pub(crate) thread_pool_receiver: Receiver<Task>,
}

impl GlobalState {
    pub(crate) fn new(connection: Connection) -> Self {
        let (thread_pool_sender, thread_pool_receiver) = crossbeam_channel::unbounded();
        Self {
            changes: Default::default(),
            diagnostic_changes: Default::default(),
            diagnostics_for_url: Default::default(),
            connection,
            content: RwLock::default(),
            db: Database::default(),
            thread_pool: rayon::ThreadPoolBuilder::new().build().unwrap(),
            thread_pool_sender,
            thread_pool_receiver,
        }
    }

    pub(crate) fn update_diagnostics(&mut self, url: Url, diagnostics: Vec<lsp_types::Diagnostic>) {
        // TODO: Check if diagnostics match, if they do, skip setting.

        self.diagnostic_changes.insert(url.clone());
        self.diagnostics_for_url.insert(url, diagnostics);
    }

    pub(crate) fn take_diagnostic_changes(&mut self) -> HashSet<Url> {
        mem::take(&mut self.diagnostic_changes)
    }

    pub(crate) fn take_changes(&mut self) -> HashSet<Url> {
        mem::take(&mut self.changes)
    }

    pub(crate) fn send_notification<R>(&self, params: R::Params)
    where
        R: lsp_types::notification::Notification,
        R::Params: serde::de::DeserializeOwned,
    {
        let not = lsp_server::Notification::new(R::METHOD.to_string(), params);
        self.send(not.into())
    }

    pub(crate) fn send(&self, msg: Message) {
        self.connection.sender.send(msg).unwrap();
    }
}

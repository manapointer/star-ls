use crossbeam_channel::{Receiver, Sender};
use lsp_server::{Connection, Message, Notification};
use lsp_types::Url;
use star_db::Database;
use std::{
    collections::{HashMap, HashSet},
    mem,
};

use crate::{main_loop::Task, subscriptions::Subscriptions};

pub(crate) struct GlobalState {
    /// Changes to document contents.
    pub(crate) changes: Vec<(Url, String)>,
    pub(crate) connection: Connection,
    pub(crate) db: Database,

    /// Changes to calculated diagnostics.
    pub(crate) diagnostics_to_sync: HashSet<Url>,
    pub(crate) latest_diagnostics: HashMap<Url, Vec<lsp_types::Diagnostic>>,
    pub(crate) task_pool: TaskPool,

    pub(crate) subscriptions: Subscriptions,
}

impl GlobalState {
    pub(crate) fn new(connection: Connection) -> Self {
        Self {
            changes: Default::default(),
            diagnostics_to_sync: Default::default(),
            latest_diagnostics: Default::default(),
            connection,
            db: Database::default(),
            task_pool: TaskPool::new(),
            subscriptions: Default::default(),
        }
    }

    pub(crate) fn process_incoming_diagnostics(
        &mut self,
        url: Url,
        diagnostics: Vec<lsp_types::Diagnostic>,
    ) {
        // Check if diagnostics match, if they do, skip setting.
        let empty = Vec::new();
        let latest_diagnostic = self.latest_diagnostics.get(&url).unwrap_or(&empty);

        if latest_diagnostic.len() == diagnostics.len() {
            for (latest, incoming) in latest_diagnostic.iter().zip(diagnostics.iter()) {
                if !latest.eq(incoming) {
                    self.diagnostics_to_sync.insert(url.clone());
                    self.latest_diagnostics.insert(url, diagnostics);
                    return;
                }
            }
        }
    }

    pub(crate) fn take_diagnostic_changes(&mut self) -> HashSet<Url> {
        mem::take(&mut self.diagnostics_to_sync)
    }

    pub(crate) fn take_changes(&mut self) -> Vec<(Url, String)> {
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

pub struct TaskPool {
    pool: rayon::ThreadPool,
    sender: Sender<Task>,
    pub(crate) receiver: Receiver<Task>,
}

impl TaskPool {
    pub(crate) fn new() -> TaskPool {
        let (sender, receiver) = crossbeam_channel::unbounded();
        TaskPool {
            pool: rayon::ThreadPoolBuilder::new().build().unwrap(),
            sender: sender,
            receiver: receiver,
        }
    }

    pub(crate) fn spawn<F>(&self, f: F)
    where
        F: FnOnce() -> Task + Send + 'static,
    {
        let sender = self.sender.clone();
        self.pool.spawn(move || {
            sender.send(f()).unwrap();
        });
    }

    pub(crate) fn spawn_with_sender<F>(&self, f: F)
    where
        F: FnOnce(Sender<Task>) + Send + 'static,
    {
        let sender = self.sender.clone();
        self.pool.spawn(move || f(sender))
    }
}

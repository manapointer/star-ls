use salsa::{Database, Durability, ParallelDatabase};
use star_syntax::{lines::Lines, parse_file, Parse};
use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
    sync::{Arc, Mutex},
};

#[salsa::jar(db = Db)]
pub struct Jar(lines, parse, File);

pub trait Db: salsa::DbWithJar<Jar> {}

#[derive(Default)]
#[salsa::db(Jar)]
pub struct RootDatabase {
    storage: salsa::Storage<Self>,
}

impl Db for RootDatabase {}

impl salsa::Database for RootDatabase {}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}

#[salsa::input]
pub struct File {
    #[return_ref]
    text: String,
}

#[derive(Default)]
pub struct SourceDatabase {
    pub db: RootDatabase,
    pub files: Arc<Mutex<HashMap<String, File>>>,
}

impl SourceDatabase {
    pub fn set_file_text(&mut self, path: String, text: String) {
        match self.files.lock().unwrap().entry(path) {
            Entry::Occupied(entry) => {
                entry.get().set_text(&mut self.db).to(text);
            }
            Entry::Vacant(entry) => {
                entry.insert(File::new(&self.db, text));
            }
        }
    }

    pub fn cancel(&mut self) {
        self.db.synthetic_write(Durability::LOW);
    }

    pub fn snapshot(&self) -> SourceDatabaseSnapshot {
        SourceDatabaseSnapshot {
            db: self.db.snapshot(),
            files: Arc::clone(&self.files),
        }
    }
}

pub struct SourceDatabaseSnapshot {
    pub db: salsa::Snapshot<RootDatabase>,
    pub files: Arc<Mutex<HashMap<String, File>>>,
}

#[salsa::tracked]
pub fn parse(db: &dyn Db, file: File) -> Parse {
    parse_file(file.text(db))
}

#[salsa::tracked]
pub fn lines(db: &dyn Db, file: File) -> Lines {
    std::thread::sleep(std::time::Duration::from_secs(2));

    Lines::new(file.text(db))
}

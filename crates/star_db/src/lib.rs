use std::collections::HashMap;

#[salsa::jar(db = Db)]
pub struct Jar(parse, File);

pub trait Db: salsa::DbWithJar<Jar> {}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
    files: HashMap<String, File>,
}

impl Db for Database {}

impl salsa::Database for Database {}

#[salsa::input]
pub struct File {
    #[return_ref]
    text: String,
}

#[salsa::tracked]
pub fn parse(db: &dyn Db) {}

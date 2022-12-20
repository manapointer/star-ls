use lsp_types::Url;
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct Subscriptions {
    urls: HashSet<Url>,
    changed: bool,
}

impl Subscriptions {
    pub(crate) fn add(&mut self, url: Url) {
        self.changed = true;
        self.urls.insert(url);
    }

    pub(crate) fn remove(&mut self, url: &Url) {
        self.urls.remove(&url);
    }

    pub(crate) fn take_changed(&mut self) -> bool {
        std::mem::replace(&mut self.changed, false)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &Url> {
        self.urls.iter()
    }
}

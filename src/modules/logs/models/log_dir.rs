use std::borrow::Cow;
use std::path::{Path, PathBuf};

mod iter;
mod try_iter;

pub use iter::Iter;
use crate::logs::models::log_dir::try_iter::TryIter;

pub struct LogDir<'a> {
    path: Cow<'a, Path>,
}

impl<'a> LogDir<'a> {
    pub fn new(path: &'a Path) -> Self {
        LogDir { path: Cow::Borrowed(path) }
    }

    pub fn iter(&'a self) -> Iter<'a> {
        Iter { inner: self }
    }

    pub fn try_iter(&'a self) -> TryIter<'a> {
        TryIter { inner: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::logs::models::log_dir::LogDir;

    #[test]
    fn api_design() {
        let log_dir = LogDir::new("./path".as_ref());

        log_dir.iter();
    }
}
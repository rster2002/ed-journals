use crate::logs::models::log_dir::LogDir;
use crate::logs::models::log_file::LogFile;

pub struct Iter<'a> {
    pub(crate) inner: &'a LogDir<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = LogFile<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
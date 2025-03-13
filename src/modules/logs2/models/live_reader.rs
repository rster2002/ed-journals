use std::fs::File;
use std::io::{BufReader};
use crate::modules::logs2::models::log_iter::LogIter;

pub struct LiveFileReader {
    inner: LogIter<BufReader<File>>,
}
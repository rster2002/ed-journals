use std::fs::File;
use std::io::{BufReader};
use crate::modules::logs2::models::log_reader::LogReader;

pub struct LiveFileReader {
    inner: LogReader<BufReader<File>>,
}
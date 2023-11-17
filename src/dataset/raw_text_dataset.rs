use std::path::Path;
use crate::dataset::{LogDataset, LogRecord};

pub struct RawTextDataset {

}

impl LogDataset for RawTextDataset {
    fn from_file(file_path: &Path) -> Self {
        todo!()
    }

    fn get_records(&self) -> &Vec<LogRecord> {
        todo!()
    }
}
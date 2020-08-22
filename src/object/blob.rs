use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct Blob {
    data: Vec<u8>,
    path: PathBuf,
}

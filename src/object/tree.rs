use super::blob::Blob;

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct Tree {
    blobs: Vec<Blob>,
    trees: Vec<Tree>,
    path: PathBuf,
}

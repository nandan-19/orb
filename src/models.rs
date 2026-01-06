use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct VectorClock {
    versions: HashMap<String, u32>,
}

impl VectorClock {
    pub fn new() -> Self {
        VectorClock {
            versions: HashMap::new(),
        }
    }

    pub fn tick(&mut self, node_id: &str) {
        *self.versions.entry(node_id.to_string()).or_insert(0) += 1;
    }
}
#[derive(Debug, Clone)]
pub struct FileState {
    pub path: String,
    pub version: VectorClock,
    pub hash: String,
    pub last_modified: u64,
}

impl FileState {
    pub fn new(path: String, hash: String, last_modified: u64, version: VectorClock) -> Self {
        FileState {
            path,
            hash,
            version: VectorClock::new(),
            last_modified,
        }
    }
}

pub struct Manifest {
    pub files: HashMap<String, FileState>,
}

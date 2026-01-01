use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub fn get_node_id() -> String {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
    path.push("orb");
    fs::create_dir_all(&path).ok();

    path.push("node_id");

    if let Ok(id) = fs::read_to_string(&path) {
        return id.trim().to_string();
    }

    let new_id = Uuid::new_v4().to_string();
    let _ = fs::write(&path, &new_id);
    new_id
}

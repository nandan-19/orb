use ed25519_dalek::SigningKey;
use rand_core::OsRng;
use std::fs;
use std::path::PathBuf;

pub fn get_node_id() -> String {
    let mut path = dirs::config_dir().unwrap_or(PathBuf::from("."));
    path.push("orb");
    fs::create_dir_all(&path).ok();

    path.push("identity.sk");

    if let Ok(bytes) = fs::read(&path)
        && bytes.len() == 32
    {
        let arr: [u8; 32] = bytes.try_into().unwrap();
        let signing_key = SigningKey::from_bytes(&arr);
        let vk = signing_key.verifying_key();
        return hex::encode(vk.to_bytes());
    }

    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);

    let _ = fs::write(&path, signing_key.to_bytes());

    let vk = signing_key.verifying_key();
    hex::encode(vk.to_bytes())
}

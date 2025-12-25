use crate::models::VectorClock;

#[derive(Debug)]
pub enum SyncAction {
    UploadToRemote,
    DownloadFromRemote,
    DoNothing,
    Conflict,
}

impl VectorClock {}

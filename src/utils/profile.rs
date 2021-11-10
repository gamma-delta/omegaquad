use std::sync::{Mutex, MutexGuard};

use macroquad::prelude::warn;
use once_cell::sync::Lazy;
use quad_wasmnastics::storage::{self, Location};
use serde::{Deserialize, Serialize};

const SERIALIZATION_VERSION: &str = "0";

/// Storage that persists between opening and closing the game.
///
/// Data is loaded from disk/localstorage in the `get` method, and is saved back upon
/// `drop`ping it.
///
/// There is a protective mutex making sure you don't have two accesses at once.
pub struct PersistentStorage {
    data: PersistentData,
    _lock: MutexGuard<'static, ()>,
}

static PERSISTENT_LOCKER: Lazy<&'static Mutex<()>> =
    Lazy::new(|| Box::leak(Box::new(Mutex::new(()))));

impl PersistentStorage {
    pub fn get() -> Self {
        let lock = PERSISTENT_LOCKER.try_lock().map_err(|e| format!("another place is interacting with persistent storage, make sure to drop it: {:?}", e)).unwrap();
        let data = PersistentData::load();
        Self { data, _lock: lock }
    }
}

impl std::ops::Deref for PersistentStorage {
    type Target = PersistentData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::ops::DerefMut for PersistentStorage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Drop for PersistentStorage {
    fn drop(&mut self) {
        let res: anyhow::Result<()> = try {
            let data = bincode::serialize(&self.data)?;
            storage::save_to(
                &data,
                &Location {
                    version: String::from(SERIALIZATION_VERSION),
                    ..Default::default()
                },
            )?
        };
        if let Err(oh_no) = res {
            warn!("Couldn't save profile!\n{:?}", oh_no);
        }
    }
}

/// The data held in persistent storage.
#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    pub open_count: u64,
}

impl PersistentData {
    fn new() -> Self {
        Self { open_count: 0 }
    }

    fn load() -> PersistentData {
        let maybe_profile: anyhow::Result<PersistentData> = try {
            // note we save the raw bincode! it's already gzipped!
            // if we gzipped it here it would jut be gzipped twice
            let data = storage::load_from(&Location {
                version: String::from(SERIALIZATION_VERSION),
                ..Default::default()
            })?;
            bincode::deserialize(&data)?
        };
        match maybe_profile {
            Ok(it) => it,
            Err(oh_no) => {
                warn!("Couldn't load profile! Loading default...\n{:?}", oh_no);
                PersistentData::new()
            }
        }
    }
}

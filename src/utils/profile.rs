use macroquad::prelude::warn;
use quad_wasmnastics::storage;
use serde::{Deserialize, Serialize};

/// Profile information. The `get` function loads it from storage; on drop it saves it back.
#[derive(Serialize, Deserialize)]
pub struct Profile {}

impl Default for Profile {
    fn default() -> Self {
        Profile {}
    }
}

impl Profile {
    pub fn get() -> Profile {
        let maybe_profile: anyhow::Result<Profile> = try {
            // note we save the raw bincode! it's already gzipped!
            // if we gzipped it here it would jut be gzipped twice
            let data = storage::load()?;
            bincode::deserialize(&data)?
        };
        match maybe_profile {
            Ok(it) => it,
            Err(oh_no) => {
                warn!("Couldn't load profile! Loading default...\n{:?}", oh_no);
                Profile::default()
            }
        }
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let res: anyhow::Result<()> = try {
            let data = bincode::serialize(self)?;
            storage::save(&data)?
        };
        if let Err(oh_no) = res {
            warn!("Couldn't save profile!\n{:?}", oh_no);
        }
    }
}

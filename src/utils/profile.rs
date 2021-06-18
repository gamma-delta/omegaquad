use macroquad::prelude::warn;
use quad_wasmnastics::storage::{self, Location};
use serde::{Deserialize, Serialize};

const SERIALIZATION_VERSION: &str = "0";

/// Profile information. The `get` function loads it from storage; on drop it saves it back.
#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub open_count: u64,
}

impl Default for Profile {
    fn default() -> Self {
        Profile { open_count: 0 }
    }
}

impl Profile {
    pub fn get() -> Profile {
        let maybe_profile: anyhow::Result<Profile> = try {
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
                Profile::default()
            }
        }
    }
}

impl Drop for Profile {
    fn drop(&mut self) {
        let res: anyhow::Result<()> = try {
            let data = bincode::serialize(self)?;
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

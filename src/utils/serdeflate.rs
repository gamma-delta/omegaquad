use anyhow::Context;
use quad_wasmnastics::storage::flate;
use serde::{de::DeserializeOwned, Serialize};

/// Serialize something to bincode and gzip it.
pub fn binzip<T: Serialize>(obj: &T) -> anyhow::Result<Vec<u8>> {
    let data = bincode::serialize(obj).context("When serializing to bincode")?;
    let zipped = flate::zip(data).context("When deflating")?;
    Ok(zipped)
}

/// Serialize something to bincode, gzip it, and base64 encode it.
pub fn binzip64<T: Serialize>(obj: &T) -> anyhow::Result<String> {
    let data = bincode::serialize(obj).context("When serializing to bincode")?;
    let zipped = flate::zip64(data).context("When deflating")?;
    Ok(zipped)
}

/// Deserialize something from gzipped bincode.
pub fn unbinzip<T: DeserializeOwned>(zipped: &[u8]) -> anyhow::Result<T> {
    let data = flate::unzip(zipped).context("When inflating")?;
    let obj = bincode::deserialize(&data).context("When deserializing from bincode")?;
    Ok(obj)
}

/// Deserialize something from base64-encodedm gzipped bincode.
pub fn unbinzip64<T: DeserializeOwned>(zipped: &str) -> anyhow::Result<T> {
    let data = flate::unzip64(zipped).context("When inflating")?;
    let obj = bincode::deserialize(&data).context("When deserializing from bincode")?;
    Ok(obj)
}

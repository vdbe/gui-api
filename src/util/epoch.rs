use std::time::SystemTime;

use serde::Serializer;

/// Custom [`Serializer`] for [`SystemTime`]
pub fn system_time<S: Serializer>(
    system_time: &SystemTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_u64(
        system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_| serde::ser::Error::custom("Invalid timestamp"))?
            .as_secs(),
    )
}

/// Custom [`Serializer`] for [`Option`]<[`SystemTime`]>
pub fn option_system_time<S: Serializer>(
    option_system_time: &Option<SystemTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    Ok(match option_system_time {
        Some(system_time) => serializer.serialize_u64(
            system_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|_| serde::ser::Error::custom("Invalid timestamp"))?
                .as_secs(),
        )?,
        None => serializer.serialize_none()?,
    })
}

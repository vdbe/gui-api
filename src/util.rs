use validator::Validate;

use crate::error::Result;

pub(crate) mod encryption;
pub(crate) mod epoch;
pub(crate) mod jwt;

pub(crate) fn validate_payload<T: Validate>(payload: &T) -> Result<()> {
    Ok(payload.validate()?)
}

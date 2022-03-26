use rand::RngCore;

use crate::error::Result;

const SALT_LEN: usize = 32;

fn argon2_config<'a>() -> argon2::Config<'a> {
    argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 4,
        mem_cost: 16 * 1024,
        time_cost: 32,
        ..Default::default()
    }
}

pub(crate) async fn hash_password(password: String) -> Result<String> {
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let mut salt = [0u8; SALT_LEN];
        rand::rngs::OsRng.fill_bytes(&mut salt);

        let argon2_config = argon2_config();

        let result = argon2::hash_encoded(password.as_bytes(), &salt, &argon2_config);
        let _ = send.send(result);
    });

    Ok(recv.await??)
}

pub async fn verify_password(password: String, hash: String) -> Result<bool> {
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let result = argon2::verify_encoded(&hash, password.as_bytes());
        let _ = send.send(result);
    });

    Ok(recv.await??)
}

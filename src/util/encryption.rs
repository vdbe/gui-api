use argon2::{self, Config};

use crate::error::Result;

pub(crate) async fn hash_password(password: String) -> Result<String> {
    let (send, recv) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let salt: String = (0..32).map(|_| rand::random::<char>()).collect();
        let result = argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &Config::default());
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

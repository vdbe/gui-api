#[derive(Serialize)]
pub(crate) struct Health {
    pub(crate) db: DbHealth,
}

#[derive(Serialize)]
pub(crate) enum DbHealth {
    Available,

    Unavailable,
}

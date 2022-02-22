#[derive(Debug, Deserialize)]
pub(crate) struct CreateInput {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) progress: i32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CreateStateInput {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) progress: i32,
}

#[derive(Debug)]
pub(crate) struct SearchStateInput<'a> {
    pub(crate) name: Option<&'a String>,
    pub(crate) description: Option<&'a String>,
}

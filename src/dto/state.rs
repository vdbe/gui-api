use crate::dto::IdentifierInput;

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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(deny_unknown_fields)]
pub(crate) enum PostStateIdentifier {
    Name(String),
    Progress(i32),
}

impl From<PostStateIdentifier> for IdentifierInput {
    fn from(identifier: PostStateIdentifier) -> Self {
        match identifier {
            PostStateIdentifier::Name(name) => Self::Text(name),
            PostStateIdentifier::Progress(progress) => Self::Integer(progress),
        }
    }
}

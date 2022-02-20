use uuid::Uuid;

use crate::schema::states;

#[derive(Debug, Queryable, Serialize)]
pub(crate) struct State {
    #[serde(skip)]
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) progress: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "states"]
pub(crate) struct CreateStateData {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) progress: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[table_name = "states"]
pub(crate) struct UpdateStateData {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) progress: Option<i32>,
}

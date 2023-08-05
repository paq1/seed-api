use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTaskCommand {
    pub titre: String,
    pub description: String,
    pub tags: Vec<String>
}

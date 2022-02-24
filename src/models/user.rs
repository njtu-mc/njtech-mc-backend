use chrono::NaiveDateTime;
use crate::app::oauth::MCProfileResp;
use crate::schema::users;

#[derive(Debug, Queryable, Identifiable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub mc_id: String,
    pub mc_name: String,
    pub gender: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub njtech_open_id: Option<String>,
    pub school: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub mc_name: String,
    pub mc_id: String,
    pub name: Option<String>,
    pub gender: i32,
    pub email: Option<String>,
    pub njtech_open_id: Option<String>,
    pub school: Option<String>,
}

impl std::convert::From<MCProfileResp> for NewUser {
    fn from(u: MCProfileResp) -> Self {
        NewUser {
            mc_name: u.name,
            mc_id: u.id,
            name: None,
            gender: 0,
            email: None,
            njtech_open_id: None,
            school: None
        }
    }
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub mc_name: Option<String>,
    pub mc_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub njtech_open_id: Option<String>,
}

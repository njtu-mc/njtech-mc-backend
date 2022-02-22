use chrono::NaiveDateTime;
use crate::schema::users;

#[derive(Debug, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub mc_name: String,
    pub mc_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub referrer_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub mc_name: String,
    pub mc_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub referrer_id: Option<i32>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub mc_name: Option<String>,
    pub mc_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub referrer_id: Option<i32>,
}

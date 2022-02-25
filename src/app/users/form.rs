#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateGender {
    #[validate(
    range(
    min = 0,
    max = 2,
    message = "fails validation - gender must be 0-2"
    )
    )]
    pub gender: i32,
    pub id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUser {
    pub realname: String,
    pub email: String,
    pub open_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnlineUser {
    pub realname: String,
    pub email: String,
    #[serde(rename(deserialize = "openId"))]
    pub open_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnlinePost {
    pub code: String,
    pub secret: String,
    pub u: OnlineUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnlineUpdateUser {
    pub id: i32,
    pub realname: String,
    pub email: String,
    pub open_id: String,
}

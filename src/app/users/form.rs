use regex::Regex;

lazy_static! {
    static ref RE_OPEN_ID: Regex = Regex::new(r"^[0-9]*$").unwrap();
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct OnlineUser {
    #[serde(rename(deserialize = "realname"))]
    pub real_name: String,
    pub email: String,
    #[serde(rename(deserialize = "openId"))]
    pub open_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OnlineUpdateUserAuthorize {
    pub code: String,
    pub secret: String,
    pub u: OnlineUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserAuthorize {
    pub id: i32,
    pub real_name: String,
    pub email: String,
    pub open_id: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct EmailUpdateUserAuthorize {
    #[validate(
    length(
    min = 1,
    max = 50,
    message = "fails validation - must be 1-50 characters long"
    ),
    )]
    pub real_name: String,
    #[validate(
    length(
    min = 1,
    max = 50,
    message = "fails validation - must be 1-50 characters long"
    ),
    regex(
    path = "RE_OPEN_ID",
    message = "fails validation - is not only number characters"
    )
    )]
    pub open_id: String,
    #[validate(
    length(
    min = 6,
    max = 6,
    message = "fails validation - must be 1-50 characters long"
    ),
    regex(
    path = "RE_OPEN_ID",
    message = "fails validation - is not only number characters"
    )
    )]
    pub code: String,
}

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

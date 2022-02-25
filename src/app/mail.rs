use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::web::{Data, Json};
use crate::app::{AppState, get_login_user_id};
use crate::error::Error;
use crate::send_authorize_code_mail;
use validator::Validate;
use crate::util::check_mail_addr;
use regex::Regex;

lazy_static! {
    static ref RE_OPEN_ID: Regex = Regex::new(r"^[0-9]*$").unwrap();
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PostMail {
    #[validate(email)]
    pub mail: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateMail {
    #[validate(email)]
    pub mail: String,
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
    pub id: Option<i32>,
}

pub async fn post_mail(form: Json<PostMail>, id: Identity) -> Result<HttpResponse, Error> {
    let form = form.into_inner();
    form.validate()?;

    let id = get_login_user_id(id)?;
    send_authorize_code_mail(id, &form.mail)?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn put_mail(
    form: Json<UpdateMail>,
    id: Identity,
    app_state: Data<AppState>
) -> Result<HttpResponse, Error> {
    let mut form = form.into_inner();
    form.validate()?;

    let id = get_login_user_id(id)?;
    form.id = Some(id);

    check_mail_addr(id, &form.mail, &form.code)?;

    let db = app_state.db.clone();
    let user = db.send(form).await??;

    Ok(HttpResponse::Ok().json(user))
}

use actix_identity::Identity;
use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::app::get_login_user_id;
use crate::error::Error;
use crate::send_authorize_code_mail;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct PostMail {
    #[validate(email)]
    pub mail: String,
}

pub async fn post_mail(form: Json<PostMail>, id: Identity) -> Result<HttpResponse, Error> {
    let form = form.into_inner();
    form.validate()?;

    let id = get_login_user_id(id)?;
    send_authorize_code_mail(id, &form.mail)?;
    Ok(HttpResponse::Ok().finish())
}
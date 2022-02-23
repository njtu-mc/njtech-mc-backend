mod form;

pub use self::form::*;
use std::fmt::Debug;
use actix_identity::Identity;
use actix_web::{HttpResponse};
use actix_web::web::{Query};
use crate::app::AppState;
use crate::error::Error;
use validator::{Validate};

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub code: String,
}

async fn login(code: &str, setting: &form::OauthSetting) -> Result<MCProfileResp, Error> {
    let client = awc::Client::default();
    let response = client.post("https://login.live.com/oauth20_token.srf")
        .send_form(&AuthorizationToken::new(code, &setting))
        .await?.json::<AuthorizationTokenResp>().await?;

    let response = client.post("https://user.auth.xboxlive.com/user/authenticate")
        .send_json(&XBLAuthorizationToken::new(&response.access_token))
        .await?.json::<XblAuthorizationTokenResp>().await?;

    let response = client.post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .send_json(&XSTSAuthorizationToken::new(&response.token))
        .await?.json::<XSTSAuthorizationTokenResp>().await?;

    let response = client.post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .send_json(&MCAuthorizationToken::new(&response.uhs, &response.token))
        .await?.json::<AuthorizationTokenResp>().await?;

    let response = match client.get("https://api.minecraftservices.com/minecraft/profile")
        .insert_header(("Authorization", format!("Bearer {}", response.access_token)))
        .send().await?.json::<MCProfileResp>().await {
        Ok(o) => o,
        Err(_) => {
            return Err(Error::BadRequest(json!("The server has not found anything matching minecraft profile")));
        }
    };

    Ok(response)
}

pub async fn auth(
    params: Query<AuthQuery>,
    app_state: actix_web::web::Data<AppState>,
    id: Identity
) -> Result<HttpResponse, Error> {
    let mc_profile = login(&params.code, &app_state.oauth_setting).await?;

    let db = app_state.db.clone();
    let user_id = match mc_profile.validate() {
        Ok(_) => {
            db.send(mc_profile).await?
        }
        Err(_) => {
            return Err(Error::InternalServerError);
        }
    }?;

    id.remember(user_id.to_string());

    Ok(
        HttpResponse::Found()
            .append_header(("Location", "https://njtumc.org"))
            .finish()
    )
}

pub(crate) mod form;

use form::*;
use std::fmt::Debug;
use actix_web::{HttpResponse};
use actix_web::web::{Query};
use crate::app::AppState;
use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub code: String,
}

async fn login(code: &str, setting: &form::OauthSetting) -> Result<MCProfileResp, Error> {
    let client = awc::Client::default();
    let response = client.post("https://login.live.com/oauth20_token.srf") // <- Create request builder
        .send_form(&AuthorizationToken::new(code, &setting))
        .await?.json::<AuthorizationTokenResp>().await?;

    let response = client.post("https://user.auth.xboxlive.com/user/authenticate") // <- Create request builder
        .send_json(&XBLAuthorizationToken::new(&response.access_token))
        .await?.json::<XblAuthorizationTokenResp>().await?;

    let response = client.post("https://xsts.auth.xboxlive.com/xsts/authorize") // <- Create request builder
        .send_json(&XSTSAuthorizationToken::new(&response.token))
        .await?.json::<XSTSAuthorizationTokenResp>().await?;

    let response = client.post("https://api.minecraftservices.com/authentication/login_with_xbox") // <- Create request builder
        .send_json(&MCAuthorizationToken::new(&response.uhs, &response.token))
        .await?.json::<AuthorizationTokenResp>().await?;

    let response = client.get("https://api.minecraftservices.com/minecraft/profile") // <- Create request builder
        .insert_header(("Authorization", format!("Bearer {}",response.access_token )))
        .send().await?.json::<MCProfileResp>().await?;

    Ok(response)
}

pub async fn auth(
    params: Query<AuthQuery>,
    app_state: actix_web::web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(login(&params.code, &app_state.oauth_setting).await?))
}

use serde::{Deserialize, Deserializer};

#[derive(Clone)]
pub struct OauthSetting {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub online_secret: String
}

#[derive(Debug, Deserialize)]
pub struct AuthorizationTokenResp {
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct XblAuthorizationTokenResp {
    #[serde(rename(deserialize = "Token"))]
    pub token: String,
}

#[derive(Debug)]
pub struct XSTSAuthorizationTokenResp {
    pub token: String,
    pub uhs: String,
}

impl<'de> Deserialize<'de> for XSTSAuthorizationTokenResp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Outer {
            #[serde(rename(deserialize = "DisplayClaims"))]
            display_claims: DisplayClaims,
            #[serde(rename(deserialize = "Token"))]
            token: String,
        }

        #[derive(Deserialize)]
        struct DisplayClaims {
            xui: Vec<Xui>,
        }

        #[derive(Deserialize)]
        struct Xui {
            uhs: String,
        }

        let helper = Outer::deserialize(deserializer)?;
        Ok(XSTSAuthorizationTokenResp {
            token: helper.token,
            uhs: helper.display_claims.xui[0].uhs.clone(),
        })
    }
}

#[derive(Serialize)]
pub struct AuthorizationToken<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
    grant_type: &'a str,
    redirect_uri: &'a str,
}

impl AuthorizationToken<'a> {
    pub fn new(code: &'a str, setting: &&'a OauthSetting) -> Self {
        AuthorizationToken {
            client_id: &setting.client_id,
            client_secret: &setting.client_secret,
            code,
            redirect_uri: &setting.redirect_url,
            grant_type: "authorization_code",
        }
    }
}

#[derive(Serialize, Debug)]
struct XBLProperties<'a> {
    #[serde(rename(serialize = "AuthMethod"))]
    auth_method: &'a str,
    #[serde(rename(serialize = "SiteName"))]
    site_name: &'a str,
    #[serde(rename(serialize = "RpsTicket"))]
    rps_ticket: String,
}

#[derive(Serialize, Debug)]
pub struct XBLAuthorizationToken<'a> {
    #[serde(rename(serialize = "Properties"))]
    properties: XBLProperties<'a>,
    #[serde(rename(serialize = "RelyingParty"))]
    relying_party: &'a str,
    #[serde(rename(serialize = "TokenType"))]
    token_type: &'a str,
}

impl XBLAuthorizationToken<'a> {
    pub fn new(token: &'a str) -> Self {
        XBLAuthorizationToken {
            properties: XBLProperties {
                auth_method: "RPS",
                site_name: "user.auth.xboxlive.com",
                rps_ticket: format!("d={}", token),
            },
            relying_party: "http://auth.xboxlive.com",
            token_type: "JWT",
        }
    }
}

#[derive(Serialize, Debug)]
struct XSTSProperties<'a> {
    #[serde(rename(serialize = "SandboxId"))]
    sand_box_id: &'a str,
    #[serde(rename(serialize = "UserTokens"))]
    user_tokens: Vec<&'a str>,
}

#[derive(Serialize, Debug)]
pub struct XSTSAuthorizationToken<'a> {
    #[serde(rename(serialize = "Properties"))]
    properties: XSTSProperties<'a>,
    #[serde(rename(serialize = "RelyingParty"))]
    relying_party: &'a str,
    #[serde(rename(serialize = "TokenType"))]
    token_type: &'a str,
}

impl XSTSAuthorizationToken<'a> {
    pub fn new(token: &'a str) -> Self {
        XSTSAuthorizationToken {
            properties: XSTSProperties {
                sand_box_id: "RETAIL",
                user_tokens: vec![token],
            },
            relying_party: "rp://api.minecraftservices.com/",
            token_type: "JWT",
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MCAuthorizationToken {
    #[serde(rename(serialize = "identityToken"))]
    identity_token: String,
}

impl MCAuthorizationToken {
    pub fn new(uhs: &str, token: &str) -> Self {
        MCAuthorizationToken {
            identity_token: format!("XBL3.0 x={}; {}", uhs, token, )
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MCProfileResp {
    #[validate(
        length(
            min = 1,
            max = 100,
            message = "fails validation - must be 1-100 characters long"
        ),
    )]
    pub id: String,
    #[validate(
        length(
            min = 1,
            max = 50,
            message = "fails validation - must be 1-50 characters long"
        ),
    )]
    pub name: String,
}


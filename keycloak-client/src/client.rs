use crate::error::{Error, Result};
use crate::response::ResponseError;
use derive_builder::Builder;
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, EmptyExtraTokenFields, ResourceOwnerPassword,
    ResourceOwnerUsername, StandardTokenResponse, TokenUrl,
    basic::BasicTokenType,
};
use oauth2::{ClientSecret, TokenResponse};
use reqwest::header::LOCATION;
use reqwest::{Method, Url};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::str::FromStr;

// Because of the stupidly overdesigned oauth2 library, we use this generator macro
// to not have to write idiotic amounts of Generic types.
macro_rules! basic_client {
    ($auth:expr) => {{
        let builder = BasicClient::new($auth.client_id.clone())
            .set_auth_uri($auth.auth_url.clone())
            .set_token_uri($auth.token_url.clone());
        let builder = if let Some(secret) = &$auth.client_secret {
            builder.set_client_secret(secret.clone())
        } else {
            builder
        };
        builder
    }};
}

/// A wrapper around the OAuth2 token response that does safe the expiration time as an absolute
/// timestime.
#[derive(Debug, Clone)]
pub struct OAuth2Token {
    pub token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
}

impl OAuth2Token {
    fn create(
        token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Self {
        let expires = token.expires_in().map(|e| chrono::Utc::now() + e);
        OAuth2Token { token, expires }
    }

    pub fn access_token(&self) -> &oauth2::AccessToken {
        self.token.access_token()
    }

    pub fn refresh_token(&self) -> Option<&oauth2::RefreshToken> {
        self.token.refresh_token()
    }
}

/// This struct represents an unauthenticated Keycloak Client. This struct can be turned into an
/// authenticated client by either providing an [`OAuth2Token`] or providing a username and
/// password.
#[derive(Debug, Builder, Clone)]
pub struct ApiAuth {
    #[builder(setter(into))]
    url: String,
    #[builder(setter(into), default = "self.default_realm()")]
    realm: String,
    #[builder(setter(into), default = "self.default_auth_url()")]
    auth_url: AuthUrl,
    #[builder(setter(into), default = "self.default_token_url()")]
    token_url: TokenUrl,
    #[builder(setter(into), default = "self.default_logout_url()")]
    logout_url: String,
    #[builder(setter(into), default = "self.default_client_id()")]
    client_id: ClientId,
    #[builder(setter(into, strip_option), default)]
    client_secret: Option<ClientSecret>,
    #[builder(setter(into), default = "self.default_http_client()")]
    http_client: reqwest::Client,
}

impl ApiAuthBuilder {
    fn default_realm(&self) -> String {
        "master".to_string()
    }

    fn default_auth_url(&self) -> AuthUrl {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        AuthUrl::new(format!(
            "{url}/realms/{realm}/protocol/openid-connect/auth"
        ))
        .unwrap()
    }

    fn default_logout_url(&self) -> String {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        format!("{url}/realms/{realm}/protocol/openid-connect/logout",)
    }

    fn default_token_url(&self) -> TokenUrl {
        let url = self.url.as_ref().unwrap();
        let realm = self.realm.clone().unwrap_or_else(|| self.default_realm());
        TokenUrl::new(format!(
            "{url}/realms/{realm}/protocol/openid-connect/token"
        ))
        .unwrap()
    }

    fn default_client_id(&self) -> ClientId {
        ClientId::new("admin-cli".to_string())
    }

    fn default_http_client(&self) -> reqwest::Client {
        reqwest::ClientBuilder::new()
            .user_agent("rustcloak")
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap()
    }
}

impl ApiAuth {
    /// Creates a new KeycloakAuth struct with the given URL. If you need more customization, you
    /// can use the [`KeycloakAuthBuilder`] struct.
    pub fn new(url: &str) -> Self {
        ApiAuthBuilder::default()
            .url(url.to_string())
            .build()
            .unwrap()
    }

    /// Tries to login with given credentials. On success a new [`KeycloakAuthBuilder`] is
    /// returned.
    pub async fn login_with_credentials(
        self,
        user_name: &str,
        password: &str,
    ) -> Result<ApiClient> {
        debug!("Logging in with user {}", user_name);

        let user = ResourceOwnerUsername::new(user_name.to_string());
        let password = ResourceOwnerPassword::new(password.to_string());

        let oauth_client = basic_client!(&self);
        let token = oauth_client
            .exchange_password(&user, &password)
            .request_async(&self.http_client)
            .await?;
        debug!("Successfully logged in with user {}", user_name);

        Ok(ApiClient::new(self, OAuth2Token::create(token)))
    }

    /// Creates a new [`KeycloakClient`] using a given [`OAuth2Token`]. There is no checking done
    /// to verify that this token is valid.
    pub fn into_client(self, token: OAuth2Token) -> ApiClient {
        ApiClient::new(self, token)
    }
}

/// This struct provides the functionality to interact with a Keycloak server. For the creation of
/// this client, see [`KeycloakAuth`].
#[derive(Debug, Clone)]
pub struct ApiClient {
    auth: ApiAuth,
    token: OAuth2Token,
}

impl ApiClient {
    fn new(auth: ApiAuth, token: OAuth2Token) -> Self {
        ApiClient { auth, token }
    }

    fn request_url(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> reqwest::RequestBuilder {
        self.auth
            .http_client
            .request(method, url)
            .bearer_auth(self.token.access_token().secret())
    }

    pub async fn handle_response(
        &self,
        res: reqwest::Response,
    ) -> Result<reqwest::Response> {
        if res.status().is_success() {
            return Ok(res);
        }

        let status = res.status();
        let result = res.bytes().await?;
        let error_msg = if let Ok(error) =
            serde_json::from_slice::<ResponseError>(&result)
        {
            error.to_string()
        } else {
            String::from_utf8_lossy(&result).to_string()
        };
        Err(Error::ResponseError(status, error_msg))
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let res = self.request_builder(Method::DELETE, path).send().await?;
        self.handle_response(res).await?;
        Ok(())
    }

    pub async fn get<O: DeserializeOwned>(&self, path: &str) -> Result<O> {
        let res = self.request_builder(Method::GET, path).send().await?;
        Ok(self.handle_response(res).await?.json().await?)
    }

    pub async fn put<I: Serialize>(
        &self,
        path: &str,
        payload: I,
    ) -> Result<()> {
        let res = self
            .request_builder(Method::PUT, path)
            .json(&payload)
            .send()
            .await?;

        self.handle_response(res).await?;
        Ok(())
    }

    pub async fn post_location<I: Serialize>(
        &self,
        path: &str,
        payload: I,
    ) -> Result<String> {
        let res = self
            .request_builder(Method::POST, path)
            .json(&payload)
            .send()
            .await?;

        let location = res
            .headers()
            .get(LOCATION)
            .cloned()
            .ok_or(Error::NoLocationHeader)?;

        let base_url = Url::from_str(&self.auth.url)?;
        self.handle_response(res).await?;
        let resource_url = Url::parse(location.to_str()?)?;
        Ok(resource_url
            .path()
            .strip_prefix(base_url.path().trim_end_matches('/'))
            .ok_or_else(|| Error::MalformedUrl(resource_url.to_string()))?
            .to_string())
    }

    fn request_builder(
        &self,
        method: reqwest::Method,
        path: &str,
    ) -> reqwest::RequestBuilder {
        self.request_url(
            method,
            &format!(
                "{}/{}",
                self.auth.url.trim_end_matches('/'),
                path.trim_start_matches('/')
            ),
        )
    }

    /// Uses the refresh token to get a new access token. The old token is invalidated.
    pub async fn refresh(&mut self) -> Result<&OAuth2Token> {
        let oauth_client = basic_client!(&self.auth);
        let token = oauth_client
            .exchange_refresh_token(self.token.refresh_token().unwrap())
            .request_async(&self.auth.http_client)
            .await?;

        self.token = OAuth2Token::create(token);
        Ok(&self.token)
    }

    /// Returns the current token.
    pub fn token(&self) -> &OAuth2Token {
        &self.token
    }

    /// Returns the realm of the current client.
    pub fn realm(&self) -> &str {
        &self.auth.realm
    }

    /// Terminates the current session.
    pub async fn logout(self) -> Result<ApiAuth> {
        let logout_url = self.auth.logout_url.clone();
        self.request_url(reqwest::Method::POST, &logout_url)
            .send()
            .await?
            .error_for_status()?;
        Ok(self.auth)
    }
}

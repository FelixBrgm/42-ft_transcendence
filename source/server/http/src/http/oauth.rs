use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn setup_oauth_client() -> BasicClient {
    let client_id = ClientId::new(dotenvy::var("CLIENT_ID").expect("CLIENT_ID not set."));
    let client_secret =
        ClientSecret::new(dotenvy::var("CLIENT_SECRET").expect("CLIENT_SECRET not set."));
    let auth_url = AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string())
        .expect("Invalid token endpoint URL");
    let redirect_uri =
        RedirectUrl::new(dotenvy::var("REDIRECT_URI").expect("REDIRECT_URI not set."))
            .expect("Invalid redirect URL");

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_uri)
}

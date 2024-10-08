use crate::commands::helper::helper_keyring::{fetch_keyring_entry, save_keyring_entry};
use crate::database::access_token_repository::save_access_token_google;
use crate::database::keychain_entry_repository::{
    save_keychain_entry_google, KEYCHAIN_KEY_GMAIL_REFRESH_TOKEN,
};
use crate::structs::access_token::AccessToken;
use crate::structs::auth::{AuthState, CallbackQuery};
use crate::structs::keychain_entry::KeychainEntry;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use dotenv::{dotenv, var};
use oauth2::basic::{BasicClient, BasicTokenResponse};
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl,
    RefreshToken, TokenResponse, TokenUrl,
};
use std::net::{SocketAddr, TcpListener};
use std::sync::Arc;
use tauri::Manager;

#[tauri::command]
pub async fn authenticate_google(handle: tauri::AppHandle) {
    let auth = handle.state::<AuthState>();
    let scope_value = "https://mail.google.com/ https://www.googleapis.com/auth/userinfo.email openid https://www.googleapis.com/auth/gmail.compose".to_string();
    let (auth_url, _) = auth
        .client
        .authorize_url(|| auth.csrf_token.clone())
        .add_extra_param("scope", scope_value)
        .add_extra_param("access_type", "offline".to_string())
        .add_extra_param("prompt", "consent".to_string())
        .add_extra_param("include_granted_scopes", "true".to_string())
        .set_pkce_challenge(auth.pkce.0.clone())
        .url();

    tauri::async_runtime::spawn(async move { run_server(handle).await });
    open::that(auth_url.to_string()).unwrap();
}

async fn authorize(
    handle: Extension<tauri::AppHandle>,
    query: Query<CallbackQuery>,
) -> impl IntoResponse {
    let auth = handle.state::<AuthState>();

    if query.state.secret() != auth.csrf_token.secret() {
        println!("Suspected Man in the Middle attack!");
        return "authorized".to_string(); // never let them know your next move
    }

    let token = auth
        .client
        .exchange_code(query.code.clone())
        .set_pkce_verifier(PkceCodeVerifier::new(auth.pkce.1.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let email = match fetch_user_email(&token).await {
        s => s,
    };

    let refresh_token = match token.refresh_token() {
        Some(token) => token.secret(),
        None => {
            panic!("Error getting refresh token from token response");
        }
    };

    // store the token in the keyring
    save_keyring_entry(KEYCHAIN_KEY_GMAIL_REFRESH_TOKEN, &email, &refresh_token);

    save_keychain_entry_google(&KeychainEntry {
        key: KEYCHAIN_KEY_GMAIL_REFRESH_TOKEN.to_string(),
        id: email.clone(),
    });

    save_access_token_google(&AccessToken {
        token: token.access_token().secret().clone(),
        keychain_user: email,
    });

    "authorized".to_string()
}

async fn fetch_user_email(token: &BasicTokenResponse) -> String {
    let user_info = reqwest::Client::new()
        .get("https://www.googleapis.com/userinfo/v2/me")
        .header(
            "Authorization",
            format!("Bearer {}", token.access_token().secret()),
        );
    let user_info_response = match user_info.send().await {
        Ok(info) => info,
        Err(error) => {
            panic!("Error getting user info: {:?}", error);
        }
    };

    let user_info_json: serde_json::Value = match user_info_response.json().await {
        Ok(json) => json,
        Err(error) => {
            panic!("Error parsing user info JSON: {:?}", error);
        }
    };

    match user_info_json["email"].as_str() {
        None => {
            panic!("Error getting email from user info JSON");
        }
        Some(email) => email.to_string(),
    }
}

async fn run_server(handle: tauri::AppHandle) -> Result<(), axum::Error> {
    let app = Router::new()
        .route("/callback", get(authorize))
        .layer(Extension(handle.clone()));

    let listener = tokio::net::TcpListener::bind(&handle.state::<AuthState>().socket_addr.clone())
        .await
        .unwrap();
    let _ = axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn create_client(redirect_url: RedirectUrl) -> BasicClient {
    dotenv().ok();
    let client_id = ClientId::new(
        var("OAUTH2_CLIENT_ID")
            .expect("Missing AUTH0_CLIENT_ID!")
            .to_string(),
    );
    let client_secret = ClientSecret::new(
        var("OAUTH2_CLIENT_SECRET")
            .expect("Missing AUTH0_CLIENT_SECRET!")
            .to_string(),
    );
    let auth_url = AuthUrl::new(
        var("OAUTH2_AUTH_URL")
            .expect("Missing AUTH0_AUTH_URL!")
            .to_string(),
    );
    let token_url = TokenUrl::new(
        var("OAUTH2_TOKEN_URL")
            .expect("Missing AUTH0_TOKEN_URL!")
            .to_string(),
    );

    BasicClient::new(
        client_id,
        Option::from(client_secret),
        auth_url.unwrap(),
        token_url.ok(),
    )
        .set_redirect_uri(redirect_url)
}

fn get_available_addr() -> SocketAddr {
    let listener = TcpListener::bind("localhost:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    addr
}

pub fn create_auth_state() -> AuthState {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let socket_addr = get_available_addr();
    let redirect_url = format!("http://localhost:{}/callback", socket_addr.port()).to_string();

    let state = AuthState {
        csrf_token: CsrfToken::new_random(),
        pkce: Arc::new((
            pkce_code_challenge,
            PkceCodeVerifier::secret(&pkce_code_verifier).to_string(),
        )),
        client: Arc::new(create_client(RedirectUrl::new(redirect_url).unwrap())),
        socket_addr,
    };

    state
}

pub async fn renew_token(handle: &tauri::AppHandle, user: &str) -> AccessToken {
    let refresh_token = fetch_keyring_entry(KEYCHAIN_KEY_GMAIL_REFRESH_TOKEN, user);
    let auth_state = handle.state::<AuthState>();
    let token = auth_state
        .client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await
        .unwrap();

    save_access_token_google(&AccessToken {
        token: token.access_token().secret().clone(),
        keychain_user: user.to_string(),
    })
}

use axum::http::{HeaderMap, HeaderValue};
use cookie::{Cookie, CookieBuilder, SameSite};
use http::header::SET_COOKIE;
 
pub fn create_jwt_cookie(token: String) -> Cookie<'static> {
    CookieBuilder::new("Token", token)
        .http_only(true)
        .secure(false) //<-- Set this in production with HTTPS
        .same_site(SameSite::Lax) //<-- Change this in production
        .path("/")
        .finish()
}

pub fn build_cookie_header(cookie: Cookie<'static>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    return headers
}
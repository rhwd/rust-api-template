use axum_extra::extract::cookie::{Cookie, Key, SignedCookieJar};

pub async fn set_signed_cookie(key: String, value: String) -> SignedCookieJar{
  let safe_key = Key::generate();
  let jar = SignedCookieJar::new(safe_key);
  jar.add(Cookie::new(key, value))
}
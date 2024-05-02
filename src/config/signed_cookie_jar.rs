use axum_extra::extract::cookie::{ Key, SignedCookieJar};

pub fn signed_jar() -> SignedCookieJar {
  let safe_key = Key::generate();
  let jar = SignedCookieJar::new(safe_key);
  jar.clone()
}
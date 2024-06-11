use crate::*;

pub fn acquire_cookie() -> String {
    let cookie_options = cookies::CookieOptions::default()
        .expires_after(core::time::Duration::from_secs(52 * 7 * 24 * 60 * 60));
    match cookies::get("test") {
        Some(Ok(cookie)) => {
            return cookie;
        }
        Some(Err(_)) => {}
        None => {}
    }
    cookies::set("test", "", &cookie_options);
    "".to_string()
}

pub fn render_cookie(cookie: &str) -> Html {
    html! {
        <div>
            <p>{cookie}</p>
        </div>
    }
}

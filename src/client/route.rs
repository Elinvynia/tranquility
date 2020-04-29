use std::fmt;

#[derive(Debug)]
pub enum Route {
    UserAbout(String),
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base = "https://oauth.reddit.com";
        let route = match self {
            Route::UserAbout(ua) => format!("/user/{}/about", ua),
        };
        write!(f, "{}{}", base, route)
    }
}

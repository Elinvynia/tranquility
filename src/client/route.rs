use std::fmt;

pub enum Route {
    UserAbout(String),
    SubredditAbout(String),
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base = "https://oauth.reddit.com";
        let route = match self {
            Route::UserAbout(ua) => format!("/user/{}/about", ua),
            Route::SubredditAbout(sa) => format!("/r/{}/about", sa),
        };
        write!(f, "{}{}", base, route)
    }
}

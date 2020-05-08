use std::fmt;

#[derive(Debug)]
pub enum Route {
    UserAbout(String),
    SubredditAbout(String),
    SubredditHot(String),
    SubredditNew(String),
    SubredditRising(String),
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let base = "https://oauth.reddit.com";
        let route = match self {
            Route::UserAbout(ua) => format!("/user/{}/about", ua),
            Route::SubredditAbout(sa) => format!("/r/{}/about", sa),
            Route::SubredditHot(sh) => format!("/r/{}/hot", sh),
            Route::SubredditNew(sn) => format!("/r/{}/new", sn),
            Route::SubredditRising(sr) => format!("/r/{}/rising", sr),
        };
        write!(f, "{}{}", base, route)
    }
}

# tranquility

Tranquility is a Rust library for the Reddit API.

Tranquilitye supports bot login via the use of `Client::new`. You must then pick an authentication method. Currently, only `BasicAuth::new` is available, meaning it can be used just for the accounts that you own as it requires the account's password.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
tranquility = "0.0.2"
```

use tranquility::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client = Client::new(
        "tranquility e01-basic, please change me",
        BasicAuth::new(
            &std::env::var("REDDIT_KEY").unwrap(),
            &std::env::var("REDDIT_SECRET").unwrap(),
            &std::env::var("REDDIT_USERNAME").unwrap(),
            &std::env::var("REDDIT_PASSWORD").unwrap(),
        ).await
    ).await.unwrap();

    let comment = client.comment("fr11tmm").await.unwrap();
    let replies = comment.replies(&client).await.unwrap();
    for reply in replies {
        println!("{}", reply.author)
    }
}

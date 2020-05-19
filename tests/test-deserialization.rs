use std::convert::{TryFrom, TryInto};
use std::{fs::File, io::prelude::*, path::Path};
use tranquility::model::prelude::*;

macro_rules! deser_from_file {
    ($filename:expr, $struct:ty) => {{
        let contents = get_file!($filename);
        let deserialized: $struct =
            serde_json::from_str(&contents).expect("Failed to deserialize.");
        deserialized
    }};
}

macro_rules! get_file {
    ($filename:expr) => {{
        let name = format!("./tests/json/{}.json", $filename);
        let path = Path::new(&name);
        let mut file = File::open(&path).expect("Failed to open file");
        let mut contents = "".to_string();
        file.read_to_string(&mut contents)
            .expect("Failed to read file.");
        contents
    }};
}

#[test]
fn test_user_about() {
    let thing: Thing = deser_from_file!("user-about", Thing);
    assert!(User::try_from(thing).is_ok())
}

#[test]
fn test_subreddit_about() {
    let thing: Thing = deser_from_file!("subreddit-about", Thing);
    assert!(Subreddit::try_from(thing).is_ok())
}

#[test]
fn test_info_comment() {
    let thing: Thing = deser_from_file!("info-comment", Thing);
    let listing: Listing = Thing::try_into(thing).unwrap();
    let children: Vec<Comment> = Listing::try_into(listing).expect("Expected children");
    assert!(children.first().is_some())
}

#[test]
fn test_info_link() {
    let thing: Thing = deser_from_file!("info-link", Thing);
    let listing: Listing = Thing::try_into(thing).unwrap();
    let children: Vec<Link> = Listing::try_into(listing).expect("Expected children");
    assert!(children.first().is_some())
}

#[test]
fn test_article_comments() {
    let mut things: Vec<Thing> = deser_from_file!("article-comments", Vec<Thing>);
    let listing: Listing = Thing::try_into(things.remove(1)).unwrap();
    let mut comments: Vec<Comment> = Listing::try_into(listing).unwrap();
    let comment: Comment = comments.remove(0);
    let replies: Thing = *comment.replies.unwrap();
    let replies2: Listing = Thing::try_into(replies).unwrap();
    let replies3: Vec<Comment> = Listing::try_into(replies2).unwrap();
    assert_eq!(replies3.len(), 1)
}

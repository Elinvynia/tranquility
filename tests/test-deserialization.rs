use std::convert::{TryFrom, TryInto};
use std::{fs::File, io::prelude::*, path::Path};
use tranquility::model::prelude::*;

macro_rules! deser_from_file {
    ($filename:expr, $struct:ident) => {{
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
    let children: Vec<Comment> = Thing::try_into(thing).expect("Expected children");
    assert!(children.first().is_some())
}

#[test]
fn test_info_link() {
    let thing: Thing = deser_from_file!("info-link", Thing);
    let children: Vec<Link> = Thing::try_into(thing).expect("Expected children");
    assert!(children.first().is_some())
}

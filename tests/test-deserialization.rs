use std::convert::TryInto;
use std::{fs::File, io::prelude::*, path::Path};
use tranquility::model::prelude::*;
use tranquility::model::thing::Thing;

macro_rules! test_deser {
    ($filename:expr, $thing:ident, $struct:ident) => {
        let contents = get_file!($filename);
        let thing: $thing = serde_json::from_str(&contents).expect("Failed to deserialize.");
        let _deserialized: $struct = $thing::try_into(thing).expect("Failed to convert.");
    };

    ($filename:expr, $struct:ident) => {
        let contents = get_file!($filename);
        let _deserialized: $struct =
            serde_json::from_str(&contents).expect("Failed to deserialize.");
    };
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
fn test_user() {
    test_deser!("user", Thing, User);
}

#[test]
fn test_subreddit() {
    test_deser!("subreddit", Thing, Subreddit);
}

#[test]
fn test_comment() {
    test_deser!("comment", Thing, Comment);
}

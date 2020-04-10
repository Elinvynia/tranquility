use std::{fs::File, io::prelude::*, path::Path};
use tranquility::model::prelude::*;

macro_rules! test_deser {
    ($filename:expr, $struct:ident) => {
        let name = format!("./tests/json/{}.json", $filename);
        println!("{:?}", &name);
        let path = Path::new(&name);
        let mut file = File::open(&path).expect("Failed to open file");
        let mut contents = "".to_string();
        file.read_to_string(&mut contents)
            .expect("Failed to read file.");

        let _deserialized: $struct =
            serde_json::from_str(&contents).expect("Failed to deserialize.");
    };
}

#[test]
fn test_user() {
    test_deser!("user", User);
}

#[test]
fn test_subreddit() {
    test_deser!("subreddit", Subreddit);
}

use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    pub name: String,
    pub path: String,
    pub link_path: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tree {
    pub root: Vec<Entry>,
    pub _links: Vec<Link>,
}

pub fn parse_tree() -> Tree {
    let data = fs::read_to_string("tree.json").unwrap();
    let t: Tree = serde_json::from_str(&data).unwrap();

    t
}

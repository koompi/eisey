use serde::{Deserialize, Serialize};
use serde_yaml::to_writer;
use std::{collections::HashMap, fs::File, io::Result, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct DB {
    pub issues: HashMap<String, PathBuf>,
}

impl DB {
    pub fn new() -> Self {
        let mut db = DB {
            issues: HashMap::new(),
        };
        db.issues
            .insert(String::from("hello"), PathBuf::from("maintenance/hello.sh"));
        db
    }
}
fn main() -> Result<()> {
    let db: DB = DB::new();
    let file = File::create("operation.yml")?;

    to_writer(file, &db).unwrap();
    Ok(())
}

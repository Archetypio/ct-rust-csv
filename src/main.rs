extern crate csv;

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use serde_json::{json, Value};
use std::collections::BTreeMap;

type Record = BTreeMap<String, String>;

#[derive(Default, Debug, Serialize)]
struct Criterion {
    text: String,
    hinting: String,
}

#[derive(Default, Debug, Serialize)]
struct Database {
    children: BTreeMap<String, Database>,
    data: Record,
    inclusion_criteria: Vec<Criterion>,
    exclusion_criteria: Vec<Criterion>,
}

impl Database {
    fn insert_path(&mut self, path: &[&str]) -> &mut Self {
        // node is a mutable reference to the current database
        let mut node = self;
        // iterate through the path
        for &subkey in path.iter() {
            // insert the new database object if necessary and
            // set node to (a mutable reference to) the child node
            node = node
                .children
                .entry(subkey.to_string())
                .or_insert_with(Database::default);
        }
        node
    }
}

fn populate_record(record: &mut Record) -> Result<String, Box<dyn Error>> {
    let mut db = Database {
        children: BTreeMap::new(),
        data: Record::new(),
        inclusion_criteria: Vec::new(),
        exclusion_criteria: Vec::new(),
    };

    let nct_trial = record.get("NCT Number").unwrap().as_str();
    let subkeys = vec![nct_trial];
    let node = db.insert_path(&subkeys);
    node.data = record.clone();

    let criterion: Criterion = Criterion {
        text: String::from("Text"),
        ..Criterion::default()
    };
    node.inclusion_criteria.push(criterion);

    let criterion2: Criterion = Criterion {
        text: String::from("Text2"),
        ..Criterion::default()
    };
    node.inclusion_criteria.push(criterion2);

    let db_json = serde_json::to_string(&db).unwrap();
    Ok(db_json.clone())
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in rdr.deserialize() {
        let mut record: Record = result?;
        let completed_record: String = populate_record(&mut record).unwrap();

        println!("{},", completed_record);
    }
    Ok(())
}

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use std::collections::HashMap;

type Record = HashMap<String, String>;

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        //println!("{:?}", record);

        let j = serde_json::to_string(&record).unwrap();
        println!("{}", j);
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
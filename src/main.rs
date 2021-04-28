extern crate csv;

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use serde_json::json;
use std::collections::BTreeMap;

type Record = BTreeMap<String, String>;

static ACRONYM: &'static str = "Acronym";
static AGE: &'static str = "Age";
static COMPLETION_DATE: &'static str = "Completion Date";
static CONDITIONS: &'static str = "Conditions";
static ENROLLMENT: &'static str = "Enrollment";
static FIRST_POSTED: &'static str = "First Posted";
static FUNDED_BYS: &'static str = "Funded Bys";
static GENDER: &'static str = "Gender";
static INTERVENTIONS: &'static str = "Interventions";
static LAST_UPDATE_POSTED: &'static str = "Last Update Posted";
static LOCATIONS: &'static str = "Locations";
static NCT_NUMBER: &'static str = "NCT Number";
static OTHER_IDS: &'static str = "Other IDs";
static OUTCOME_MEASURES: &'static str = "Outcome Measures";
static PHASES: &'static str = "Phases";
static PRIMARY_COMPLETION_DATE: &'static str = "Primary Completion Date";
static RANK: &'static str = "Rank";
static RESULTS_FIRST_POSTED: &'static str = "Results First Posted";
static SPONSOR_COLLABORATORS: &'static str = "Sponsor/Collaborators";
static START_DATE: &'static str = "Start Date";
static STATUS: &'static str = "Status";
static STUDY_DESIGNS: &'static str = "Study Designs";
static STUDY_DOCUMENTS: &'static str = "Study Documents";
static STUDY_RESULTS: &'static str = "Study Results";
static STUDY_TYPE: &'static str = "Study Type";
static TITLE: &'static str = "Title";
static URL: &'static str = "URL";

#[derive(Serialize, Default)]
struct Criterion {
    text: String,
    hinting: String,
}

#[derive(Serialize, Default)]
struct Trial {
    acronym: String,
    age: String,
    completion_date: String,
    conditions: String,
    enrollment: String,
    exclusion_criteria: Vec<Criterion>,
    first_posted: String,
    funded_bys: String,
    gender: String,
    inclusion_criteria: Vec<Criterion>,
    interventions: String,
    last_update_posted: String,
    locations: String,
    nct_nunber: String,
    other_ids: String,
    outcome_measures: String,
    phases: String,
    primary_completion_date: String,
    rank: String,
    results_first_posted: String,
    sponsor_collaborators: String,
    start_date: String,
    status: String,
    study_designs: String,
    study_documents: String,
    study_results: String,
    study_type: String,
    title: String,
    url: String,
}

fn populate_record(record: &mut Record) -> Result<Record, Box<dyn Error>> {
    let c1: Criterion = Criterion {
        text: String::from("criterion string"),
        ..Criterion::default()
    };
    let c2: Criterion = Criterion {
        text: String::from("criterion string"),
        hinting: String::from("hinting string"),
    };

    let trial: Trial = Trial {
        acronym: String::from(record[ACRONYM].clone()),
        age: String::from(record[AGE].clone()),
        completion_date: String::from(record[COMPLETION_DATE].clone()),
        conditions: String::from(record[CONDITIONS].clone()),
        enrollment: String::from(record[ENROLLMENT].clone()),
        exclusion_criteria: vec![c1, c2],
        first_posted: String::from(record[FIRST_POSTED].clone()),
        funded_bys: String::from(record[FUNDED_BYS].clone()),
        gender: String::from(record[GENDER].clone()),
        interventions: String::from(record[INTERVENTIONS].clone()),
        last_update_posted: String::from(record[LAST_UPDATE_POSTED].clone()),
        locations: String::from(record[LOCATIONS].clone()),
        nct_nunber: String::from(record[NCT_NUMBER].clone()),
        other_ids: String::from(record[OTHER_IDS].clone()),
        outcome_measures: String::from(record[OUTCOME_MEASURES].clone()),
        phases: String::from(record[PHASES].clone()),
        primary_completion_date: String::from(record[PRIMARY_COMPLETION_DATE].clone()),
        rank: String::from(record[RANK].clone()),
        results_first_posted: String::from(record[RESULTS_FIRST_POSTED].clone()),
        sponsor_collaborators: String::from(record[SPONSOR_COLLABORATORS].clone()),
        start_date: String::from(record[START_DATE].clone()),
        status: String::from(record[STATUS].clone()),
        study_designs: String::from(record[STUDY_DESIGNS].clone()),
        study_documents: String::from(record[STUDY_DOCUMENTS].clone()),
        study_results: String::from(record[STUDY_RESULTS].clone()),
        study_type: String::from(record[STUDY_TYPE].clone()),
        title: String::from(record[TITLE].clone()),
        url: String::from(record[URL].clone()),
        ..Trial::default()
    };

    let tjson = json!(trial);
    println!("{}\n", tjson);

    record
        .entry(String::from("Inclusion Criteria"))
        .or_insert(String::from("Criterion"));
    Ok(record.clone())
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    for result in rdr.deserialize() {
        let mut record: Record = result?;
        let completed_record: Record = populate_record(&mut record).unwrap();

        let j = serde_json::to_string_pretty(&completed_record).unwrap();
        //println!("{},", j);
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

// Project 2: Contact manager
//
// User stories:
// * L1: I want to view my saved contacts.
// * L2: I want to add new contacts.
// * L2: I want to search for contacts.
// * L3: I want to edit and remove existing contacts.
//
// Tips:
// * Make a backup of the existing `p2_data.csv` file.
// * CSV files contain records and fields:
//   Each line is a "record" which contain "fields" of information.
//   The fields are separated by commas. If a field is not provided,
//   then there is no data for that particular field. There will
//   always be a comma for the field even if no data is present.
// * The `id` and `name` fields are required, the `email` field is optional.
// * Check the documentation on the `std::fs::File` struct for reading
//   and writing files.
// * Use the `split` function from the standard library to extract
//   specific fields.
// * Try the `structopt` crate if you want to make a non-interactive
//   command line application.
// * Create your program starting at level 1. Once finished, advance
//   to the next level.
// * Make your program robust: there are 7 errors & multiple blank lines
//   present in the data.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug)]
struct Record {
    id: i64,
    name: String,
    email: Option<String>,
}

#[derive(Debug)]
struct Records{
    inner: HashMap<i64,Record>
}

impl Records {
    fn new() -> Self {
        Self { inner: HashMap::new() }
    }
 
    fn edit(&mut self, id: i64, name: &str, email: Option<String>) {
        self.inner.insert(
            id,
            Record {
                id,
                name: name.to_string(),
                email,
            },
        );
    }

    
    fn next_id(&self) -> i64 {
       
        let mut ids: Vec<_> = self.inner.keys().collect();
        ids.sort();
       
        match ids.pop() {
            Some(id) => id + 1,
            None => 1,
        }
    }

   
    fn add(&mut self, record: Record) {
        self.inner.insert(record.id, record);
    }

   
    fn into_vec(mut self) -> Vec<Record> {
      
        let mut records: Vec<_> = self.inner.drain().map(|kv| kv.1).collect();
       
        records.sort_by_key(|rec| rec.id);
        records
    }

   
    fn search(&self, name: &str) -> Vec<&Record> {
       
        self.inner
            .values()
            .filter(|rec| rec.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }

    fn remove(&mut self, id: i64) -> Option<Record> {
        self.inner.remove(&id)
    }
}

#[derive(Error,Debug)]
enum ParseError {
    #[error("missing field")]
    MissingField(String),
    #[error("empty record")]
    EmptyRecord,
    #[error("invalid id")]
    InvalidId(#[from] std::num::ParseIntError),
}

fn parse_record(record: &str) -> Result<Record, ParseError> {
   
    let fields: Vec<&str> = record.split(',').collect();   
    let id = match fields.get(0) {
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyRecord),
    };

   
    let name = match fields.get(1).filter(|name| **name != "") {
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField("name".to_owned())),
    };

   
    let email = fields
        .get(2)
        .map(|email| email.to_string())
        .filter(|email| email != "");

    Ok(Record { id, name, email })
}
fn parse_records(records: String, verbose: bool) -> Records {
    let mut recs = Records::new();
    
    for (num, record) in records.split('\n').enumerate() {
        if record != "" {
            match parse_record(record) {
                Ok(rec) => recs.add(rec),
                Err(e) => {
                    if verbose {
                        println!(
                            "error on line number {}: {}\n  > \"{}\"\n",
                            num + 1,
                            e,
                            record
                        );
                    }
                }
            }
        }
    }
    recs
}
fn load_records(file_name: PathBuf, verbose:bool) -> std::io::Result<Records> {
    let mut file = File::open(file_name)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_records(buffer, verbose))
}

fn save_records(file_name: PathBuf, records: Records) -> std::io::Result<()> {
    
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

   
    file.write(b"id,name,email\n")?;

   
    for record in records.into_vec().into_iter() {
        
        let email = match record.email {
            Some(email) => email,
            None => "".to_string(),
        };
       
        let line = format!("{},{},{}\n", record.id, record.name, email);
       
        file.write(line.as_bytes())?;
    }
   
    file.flush()?;
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(about = "project 2: contact manager")]
struct Opt {
    #[structopt(short, parse(from_os_str), default_value = "p2_data.csv")]
    data_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool,
}

#[derive(StructOpt, Debug)]
enum Command {
    Add {
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    Edit {
        id: i64,
        name: String,
        #[structopt(short)]
        email: Option<String>,
    },
    List {},
    Remove {
        id: i64,
    },
    Search {
        query: String,
    },
}


fn run(opt: Opt) -> Result<(), std::io::Error> {
    match opt.cmd {
        Command::Add { name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            let next_id = recs.next_id();
            recs.add(Record {
                id: next_id,
                name,
                email,
            });
            save_records(opt.data_file, recs)?;
        }
        Command::Edit { id, name, email } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            recs.edit(id, &name, email);
            save_records(opt.data_file, recs)?;
        }
        Command::List { .. } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            for record in recs.into_vec() {
                println!("{:?}", record);
            }
        }
        Command::Remove { id } => {
            let mut recs = load_records(opt.data_file.clone(), opt.verbose)?;
            if recs.remove(id).is_some() {
                save_records(opt.data_file, recs)?;
                println!("record deleted");
            } else {
                println!("record not found");
            }
        }
        Command::Search { query } => {
            let recs = load_records(opt.data_file, opt.verbose)?;
            let results = recs.search(&query);
            if results.is_empty() {
                println!("no records found");
            } else {
                for rec in results {
                    println!("{:?}", rec);
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        println!("an error occurred: {}", e);
    }
}


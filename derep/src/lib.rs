use bio::io::fasta;
use bio::io::fasta::Record;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// Path to input fasta file.
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,
}

fn format_header(rec: &Record) -> String {
    match rec.desc() {
        Some(desc) => format!(">{} {}", rec.id(), desc),
        None => rec.id().to_string(),
    }
}

/// We start with the simplest possible way to do this.  Just hash the record and store it.
pub fn run(opts: Opts) {
    let mut records = HashMap::new();
    let reader = fasta::Reader::from_file(opts.infile).expect("couldn't read input file");

    let mut total_duplicates = 0;

    for item in reader.records() {
        let record = item.unwrap();

        // Doing it this way will keep the headers further down in the file.
        match records.insert(
            str::from_utf8(record.seq()).unwrap().to_ascii_uppercase(),
            format_header(&record),
        ) {
            None => (),
            Some(_) => total_duplicates += 1,
        }
    }

    eprintln!("Total duplicates: {}", total_duplicates);

    for (seq, header) in records.iter() {
        println!("{}\n{}", header, seq);
    }
}

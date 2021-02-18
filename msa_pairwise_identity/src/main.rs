use bio::io::fasta;
use bio::io::fasta::Record;
use indicatif::{ProgressBar, ProgressStyle};
use std::process;
use structopt::StructOpt;

/// All combinations of sequence identity based on a multiple sequence alignment (MSA).
///
/// Prints combinations, not permutations.  Doesn't print self-hits.
///
/// E.g., for seqs A, B, C, I will print,
///
/// A B XX.X
/// A C XX.X
/// B C XX.X
#[derive(StructOpt, Debug)]
struct Opts {
    /// Path to MSA fasta file.
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,
}

/// If no positions are in common, then it's 0.0 by definition.  Otherwise, it
/// is identity = (number of identities) / (number of shared columns) * 100.
/// See https://drive5.com/usearch/manual7/id_threshold.html.
fn get_percent_identity(rec1: &Record, rec2: &Record) -> f64 {
    let mut matches = 0u32;
    let mut total = 0u32;

    let seq1 = rec1.seq();
    let seq2 = rec2.seq();

    let shared_positions = seq1
        .iter()
        .zip(seq2)
        .filter(|(&pos1, &pos2)| pos1 != b'-' && pos2 != b'-');

    for (&pos1, &pos2) in shared_positions {
        total += 1;

        if pos1 == pos2 {
            matches += 1;
        }
    }

    if total == 0 {
        0.0
    } else {
        f64::from(matches) / f64::from(total) * 100.0
    }
}

fn format_header(rec: &Record) -> String {
    match rec.desc() {
        Some(desc) => format!(">{} {}", rec.id(), desc),
        None => rec.id().to_string(),
    }
}

fn check_seq_len(rec: &Record, expected_len: usize) {
    if expected_len != rec.seq().len() {
        eprintln!(
            "ERROR -- expected length of {}.  But got {} for seq {}",
            expected_len,
            rec.seq().len(),
            format_header(&rec)
        );
        process::exit(1);
    }
}

fn make_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);

    pb.set_draw_delta(10);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {bar:40}")
            .progress_chars("#-"),
    );

    pb.set_message("Working");

    pb
}

fn main() {
    let opts = Opts::from_args();

    eprintln!("DEBUG -- {:?}", opts);

    let reader = fasta::Reader::from_file(opts.infile).expect("couldn't read input file");

    let mut records = Vec::new();

    let mut seq_length = 0;

    for item in reader.records() {
        // Obtain record or fail with error
        let record = item.unwrap_or_else(|err| {
            eprintln!("ERROR -- problem getting record from fasta file: {}", err);
            process::exit(1);
        });

        if seq_length == 0 {
            seq_length = record.seq().len();
        }

        check_seq_len(&record, seq_length);

        records.push(record);
    }

    let pb = make_progress_bar(records.len() as u64 - 1);

    for i in 0..(records.len() - 1) {
        pb.inc(1);

        let rec1 = &records[i];

        for rec2 in &records[i + 1..] {
            let percent_identity = get_percent_identity(rec1, rec2);

            println!(
                "{}\t{}\t{}",
                format_header(rec1),
                format_header(rec2),
                percent_identity
            );
        }
    }

    pb.finish();
}

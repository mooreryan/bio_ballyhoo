mod blosum;

use std::process;

use bio::io::fasta;
use bio::io::fasta::Record;
use blosum::{blosum45, blosum50, blosum62, blosum80, blosum90, BlosumMatrix, MinScore};
use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Method {
    /// Output percent identities.
    Identity,

    /// Output percent similarities
    ///
    /// --blosum must be one of: blosum45, blosum50, blosum62, blosum80, blosum90
    ///
    /// --min-score must be one of: 0, 1
    Similarity {
        /// Which scoring matrix to use?
        #[structopt(short, long, default_value = "blosum62")]
        blosum: BlosumMatrix,

        /// Min score to count as 'similarity'?
        #[structopt(short, long, default_value = "1")]
        min_score: MinScore,
    },
}

/// All combinations of sequence identity or similarity based on a multiple sequence alignment (MSA).
///
/// Prints combinations, not permutations.  Doesn't print self-hits.
///
/// E.g., for seqs A, B, C, I will print,
///
/// A\tB\tXX.X\nA\tC\tXX.X\nB\tC\tXX.X\n
#[derive(StructOpt, Debug)]
pub struct Opts {
    /// Path to MSA fasta file.
    #[structopt(short, long, parse(from_os_str))]
    infile: std::path::PathBuf,

    #[structopt(subcommand)]
    method: Method,
}

fn is_identical(this: u8, other: u8) -> bool {
    this == other
}

/// If either of them are not comparable, you just get false back.
fn is_similar(this: u8, other: u8, matrix: BlosumMatrix, min_score: MinScore) -> bool {
    let score = match matrix {
        BlosumMatrix::Blosum45 => blosum45::score(this, other),
        BlosumMatrix::Blosum50 => blosum50::score(this, other),
        BlosumMatrix::Blosum62 => blosum62::score(this, other),
        BlosumMatrix::Blosum80 => blosum80::score(this, other),
        BlosumMatrix::Blosum90 => blosum90::score(this, other),
    };

    match score {
        Some(score) => score >= min_score.to_i8(),
        None => false,
    }
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

        if is_identical(pos1, pos2) {
            matches += 1;
        }
    }

    if total == 0 {
        0.0
    } else {
        f64::from(matches) / f64::from(total) * 100.0
    }
}

fn get_percent_similarity(
    rec1: &Record,
    rec2: &Record,
    matrix: BlosumMatrix,
    min_score: MinScore,
) -> f64 {
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

        if is_similar(pos1, pos2, matrix, min_score) {
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

/// I will panic with "good" error message if anything goes wrong.
pub fn run(opts: Opts) {
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
            let percent_score = match opts.method {
                Method::Identity => get_percent_identity(rec1, rec2),
                Method::Similarity { blosum, min_score } => {
                    get_percent_similarity(rec1, rec2, blosum, min_score)
                }
            };

            println!(
                "{}\t{}\t{}",
                format_header(rec1),
                format_header(rec2),
                percent_score
            );
        }
    }

    pb.finish();
}

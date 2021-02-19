use bio::io::fasta;
use bio::io::fasta::Record;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process;
use structopt::StructOpt;

/// Give your fasta file Simple Headers.
#[derive(StructOpt, Debug)]
struct Opts {
    /// Path to fasta file.
    #[structopt(parse(from_os_str))]
    infile: std::path::PathBuf,

    /// Annotation to put at start of seq name.
    annotation: Option<String>,
}

fn format_header(rec: &Record) -> Cow<str> {
    match rec.desc() {
        Some(desc) => format!("{} {}", rec.id(), desc).into(),
        None => rec.id().into(),
    }
}

fn check_file_arg(arg: &Path) {
    if !arg.exists() {
        eprintln!("FATAL -- infile does not exist: {:?}", arg);
        process::exit(1);
    }

    if !arg.is_file() {
        eprintln!("FATAL -- infile is not a file: {:?}", arg);
        process::exit(1);
    }
}

fn get_new_header(index: usize, annotation: &Option<String>) -> String {
    match annotation {
        Some(annotation) => format!("{}___seq_{}", annotation, index + 1),
        None => format!("seq_{}", index + 1),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    eprintln!("DEBUG -- {:?}", opts);

    check_file_arg(&opts.infile);

    let infile_stem = opts.infile.file_stem().expect("no file stem");
    let infile_extension = opts.infile.extension().expect("no file extension");
    let infile_dir = opts.infile.parent().expect("no dir");

    let simple_headers_fname = infile_dir.join(format!(
        "{}.simple_headers.{}",
        infile_stem.to_str().expect("invalid UTF-8 for infile stem"),
        infile_extension
            .to_str()
            .expect("invalid UTF-8 for infile stem")
    ));

    // new header => old header
    let map_fname = infile_dir.join(format!(
        "{}.simple_headers.name_map.tsv",
        infile_stem.to_str().expect("invalid UTF-8 for infile stem"),
    ));

    let map_writer = File::create(map_fname)?;
    let mut map_writer = BufWriter::new(map_writer);

    let simple_headers_writer = File::create(simple_headers_fname)?;
    let mut simple_headers_writer = BufWriter::new(simple_headers_writer);

    let reader = fasta::Reader::from_file(opts.infile)?;

    for (i, rec) in reader.records().enumerate() {
        let rec = rec?;

        let new_header = get_new_header(i, &opts.annotation);
        let seq = std::str::from_utf8(rec.seq())?;

        writeln!(&mut simple_headers_writer, ">{}\n{}", new_header, seq)?;
        writeln!(&mut map_writer, "{}\t{}", new_header, format_header(&rec))?;
    }

    Ok(())
}

use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::io::BufWriter;
use std::io::Write;
use std::str::FromStr;
use std::{fmt, io};
use structopt::StructOpt;

// TODO generate fastq as well.

fn generate_sequence<R: Rng>(seq_type: SequenceType, length: u64, mut rng: &mut R) -> Vec<u8> {
    let choices = match seq_type {
        SequenceType::Dna => vec![b'A', b'C', b'T', b'G'],
        SequenceType::Rna => vec![b'A', b'C', b'U', b'G'],
        SequenceType::Protein => vec![
            b'G', b'A', b'L', b'M', b'F', b'W', b'K', b'Q', b'E', b'S', b'P', b'V', b'I', b'C',
            b'Y', b'H', b'R', b'N', b'D', b'T',
        ],
    };

    (0..length)
        .map(|_| choices.choose(&mut rng).unwrap())
        .copied()
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum SequenceType {
    Dna,
    Rna,
    Protein,
}

#[derive(Debug)]
struct ParseSequenceTypeErr;

impl fmt::Display for ParseSequenceTypeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SequenceType parse error!")
    }
}

impl FromStr for SequenceType {
    type Err = ParseSequenceTypeErr;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.to_ascii_lowercase().as_str() {
            "dna" => Ok(Self::Dna),
            "rna" => Ok(Self::Rna),
            "protein" => Ok(Self::Protein),
            _ => Err(ParseSequenceTypeErr),
        }
    }
}

/// Generate random reads.
#[derive(StructOpt, Debug)]
pub struct Opts {
    /// Number of sequences to generate
    #[structopt(short, long)]
    num_seqs: u64,

    /// Length of generated sequences.
    #[structopt(short, long)]
    length: u64,

    /// Seed for random number generator.
    #[structopt(short, long)]
    seed: u64,

    /// Type of sequence to generate (dna, rna, protein).
    #[structopt(short = "t", long)]
    seq_type: SequenceType,
}

pub fn run(opts: Opts) {
    let mut rng = ChaCha20Rng::seed_from_u64(opts.seed);

    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut writer = BufWriter::new(lock);

    for i in 1..=opts.num_seqs {
        let seq = generate_sequence(opts.seq_type, opts.length, &mut rng);

        writeln!(
            writer,
            ">seq_{}\n{}",
            i,
            std::str::from_utf8(seq.as_slice()).unwrap()
        )
        .unwrap();
    }
}

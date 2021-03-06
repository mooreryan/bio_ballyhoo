use msa_pairwise::{run, Opts};
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();

    eprintln!("DEBUG -- {:?}", opts);

    run(opts);
}

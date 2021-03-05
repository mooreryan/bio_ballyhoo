use structopt::StructOpt;

use msa_pairwise::{run, Opts};

fn main() {
    let opts = Opts::from_args();

    eprintln!("DEBUG -- {:?}", opts);

    run(opts);
}

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct DEBruijnArgs {
    /// please provide the path to the fastq file
    pub debruijn_arg: String,
    /// please provide the kmer for the graph construction
    pub kmer_arg: usize,
}

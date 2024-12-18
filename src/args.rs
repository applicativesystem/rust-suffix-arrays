use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct SuffixArraysArgs {
    /// please provide the path to the fastq file
    pub suffixarray_arg: String
}

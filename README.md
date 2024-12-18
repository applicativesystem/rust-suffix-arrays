# rust-suffix-arrays

**Today i finished one month of RUST and memorized all RUST docs and all these crates i have coded in one month of learning and applying at the same time. I coded all these algorithms in C++ previously and now coded in RUST and finished learning RUST from system programming to Web development in one month**.

 - This rust emphasis on the following part for the implementation.
 - rust suffix array building from two points: 1. LCP and 2. Kasai path generation.  
 - please see the other trait for the Kasai and Ukonnen implemented using this crate to build the graph.
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.
 
 ```
 cargo build 
 
 ```
 ```
  ➜ gauravsablok  rust-suffix-arrays-tries git:(main) ✗ ./target/debug/rust-suffix-arrays -h
Usage: rust-suffix-arrays <SUFFIXARRAY_ARG>

Arguments:
  <SUFFIXARRAY_ARG>  please provide the path to the fastq file

Options:
  -h, --help     Print help
  -V, --version  Print version 

```
- to run the binary 
```
./target/debug/rust-suffix-arrays ./sample-files/sample.fasta
```

 Gaurav Sablok 

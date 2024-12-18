mod args;
use args::SuffixArraysArgs;
use clap::Parser;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-18

rust-suffix-arrays: implementing the suffix arrays building as BtreeMaps,
so that when you have to get the edge of the suffix array, you dont have to traverse
throug the entire array. This will maintain the order of the heap for
later implementation of the LCP block or tree construction using the Kasai algorithm.

* */

fn main() {
    let args = SuffixArraysArgs::parse();
    let suffix_output = suffix_array(&args.suffixarray_arg).unwrap();
    sorted_suffix_array();
    println!("The suffix arrays has been written: {}", suffix_output);
}

fn suffix_array(path: &str) -> Result<String, Box<dyn Error>> {
    let file_appear = File::open(path).expect("file not present");
    let fileread_appear = BufReader::new(file_appear);
    let mut filecontent: Vec<String> = Vec::new();
    let mut filecontent_seq: Vec<String> = Vec::new();
    for i in fileread_appear.lines() {
        let line = i.expect("line not present");
        if line.starts_with("@") {
            filecontent.push(line)
        } else if line.starts_with("A")
            || line.starts_with("T")
            || line.starts_with("G")
            || line.starts_with("C")
        {
            filecontent_seq.push(line)
        }
    }

    // forming a binary heap for the suffix arrays so that the order of the suffixes
    // can be kept intact and on the heap so that i can look for the top prefix using
    // traversal or implementation for a inverse suffix.

    #[derive(Debug, Clone)]

    struct SuffixMap {
        indices: Vec<usize>,
        array: Vec<String>,
    }
    let mut final_suffix_array: BTreeMap<String, SuffixMap> = BTreeMap::new();
    let mut final_suffix_sorted_btreemap: BTreeMap<String, Vec<(usize, String)>> = BTreeMap::new();
    for i in 0..filecontent_seq.len() {
        let seqhold: Vec<_> = filecontent_seq[i]
            .chars()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        let mut heap_indices: Vec<usize> = Vec::new();
        let mut heap_string: Vec<String> = Vec::new();
        for i in 0..seqhold.len() {
            heap_indices.push(i);
            heap_string.push(seqhold[i..seqhold.len()].join("").to_string());
        }
        let mut suffix_hold: Vec<(usize, String)> = Vec::new();
        for (indices, str) in heap_indices.iter().zip(heap_string.iter()) {
            suffix_hold.push((*indices, str.to_string()));
        }
        suffix_hold.sort_by(|a, b| a.1.cmp(&b.1));

        final_suffix_sorted_btreemap.insert(filecontent_seq[i].clone(), suffix_hold);
        final_suffix_array.insert(
            seqhold.join(""),
            SuffixMap {
                indices: heap_indices,
                array: heap_string,
            },
        );
    }

    let mut suffix_sorted_btreemap =
        File::create("suffix-sorted-trees.txt").expect("file not present");

    for (i,j) in final_suffix_sorted_btreemap.iter() {
        writeln!(suffix_sorted_btreemap, "{}\t{:?}\n", i,j ).expect("line not present");
    }

    Ok("Binary Heaped Suffix arrays have been written".to_string())
}

fn sorted_suffix_array() {
    /* This ives you a tuple based approach so that you can access the
     * indices and the corresponding suffices by their value coordinates.
     * */
    let argsparse = SuffixArraysArgs::parse();
    let fileopen = File::open(&argsparse.suffixarray_arg).unwrap();
    let fileread = BufReader::new(fileopen);
    let mut sequence: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("line not present");
        if !line.starts_with("@") {
            sequence.push(line);
        }
    }

    let mut final_array: Vec<(usize, String)> = Vec::new();
    let mut suffix_indices: Vec<usize> = Vec::new();
    let mut suffix_strings: Vec<String> = Vec::new();
    for i in 0..sequence.len() {
        let seq_hold = sequence[i]
            .chars()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        for i in 0..seq_hold.len() {
            suffix_indices.push(i);
            suffix_strings.push(seq_hold[i..seq_hold.len()].join("").to_string());
        }

        for (indices, string) in suffix_indices.iter().zip(suffix_strings.iter()) {
            final_array.push((*indices, string.to_string()));
        }
    }

    final_array.sort_by(|a, b| a.1.cmp(&b.1));

    let mut final_tuple_write = File::create("tuple_suffix_array.txt").expect("file not present");
    for i in final_array.iter() {
        writeln!(final_tuple_write, "{}\t{}\n", i.0, i.1).expect("line not present");
    }
}

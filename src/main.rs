mod args;
use args::DEBruijnArgs;
use clap::Parser;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-17

rust-debruijn-algorithm: implementation of debruijns algorithm and
graph traversal in RUST. This implements three data structure, one of the
data structure is the BTreeMap which allows for for the node to the edge representation.
This alow to see that if the key is present in the BTreemap then visit the ode and retrieve
the edges or else dont for the tree traversal.


* */

fn main() {
    let args = DEBruijnArgs::parse();
    let debruijn_longread_output = debruijn_longread(&args.debruijn_arg, args.kmer_arg).unwrap();
    println!("The bwt has been written: {}", debruijn_longread_output);
}

fn debruijn_longread(path: &str, overlap: usize) -> Result<String, Box<dyn Error>> {
    /* the debruijn graph is constructed and hold the nodes and egdes into the Btreemap and
     * TheBRtreeMap is further inserted into the Vector which allows for he faster revtriveal
     * of the data.
     *
     * */

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

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct NodeEdgeHold {
        node: String,
        edge: String,
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct MapStore {
        node_edge_relation: Vec<NodeEdgeHold>,
        uniq_node: Vec<String>,
    }

    let mut debruijn: BTreeMap<String, MapStore> = BTreeMap::new();
    for i in 0..filecontent_seq.len() {
        let contentseq: Vec<String> = filecontent_seq[i]
            .chars()
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        let mut edge_map: Vec<String> = Vec::new();
        let mut node_map: Vec<String> = Vec::new();
        for i in 0..=contentseq.len() - overlap {
            node_map.push(contentseq[i..i + overlap - 1].join(""));
            edge_map.push(contentseq[i + 1..i + overlap].join(""));
        }
        let unique_node: HashSet<_> = node_map
            .iter()
            .collect::<HashSet<_>>();
        let mut final_node_unique:Vec<String> = Vec::new();
        for i in unique_node.iter(){
            final_node_unique.push(i.to_string());
        }
        let mut node_edge_tuple: Vec<NodeEdgeHold> = Vec::new();
        for (nodeiter, edgeiter) in node_map.iter().zip(edge_map.iter()) {
            node_edge_tuple.push(NodeEdgeHold {
                node: nodeiter.to_string(),
                edge: edgeiter.to_string(),
            })
        }
        debruijn.insert(
            filecontent_seq[i].clone(),
            MapStore {
                node_edge_relation: node_edge_tuple,
                uniq_node: final_node_unique,
            },
        );
    }

    let mut file_debruijn_write = File::create("debruijn-graph.txt").expect("file not pesent");
    for i in debruijn.iter() {
        writeln!(file_debruijn_write, "{:?}\t{:?}\n", i.0, i.1).expect("file not present");
    }

    Ok("The debruijn graph construction has finished".to_string())
}

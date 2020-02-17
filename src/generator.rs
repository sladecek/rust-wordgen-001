use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use rand::{Rng};
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

use crate::builder::{Builder, START_WORD, END_WORD};

#[derive(Copy, Clone,Serialize,Deserialize)]
pub struct CumCount {
    c: char,
    // Number of transitions to character c and all other characters with
    // code smaller than c.
    cf: i32
}

// Generates random words.
#[derive(Serialize,Deserialize)]
pub struct Generator {
    pub depth: usize, 
    transitions: HashMap<String, Vec<CumCount>>    
}

impl Generator {

    pub fn new_from_builder(b: Builder) -> Generator {
        let mut result = Generator { depth: b.depth,  transitions: HashMap::new() };
        // for all starting characters
        for c1 in b.transitions.keys() {
            let m2 = &b.transitions[c1];
            // create new vector of cumulative counts
            let mut v: Vec<CumCount> = Vec::new();
            let mut cs: i32 = 0;
            let mut k2: Vec<char> = Vec::new();
            for c22 in  m2.keys() {
                k2.push(*c22);
            }
            k2.sort();
            // for all following characters in alphabet order
            for c2 in k2 {
                cs += m2[&c2];
                v.push( CumCount{c: c2, cf: cs});
            }
            result.transitions.insert(c1.to_string(), v);
        }
        result        
    }
    
    pub fn new_from_file(file_name: &str) -> Generator {
        let file = File::open(file_name).expect("Cannot read file");
        let br = BufReader::new(file);
        let decompressed = ZlibDecoder::new(br);
        serde_json::from_reader(decompressed).unwrap()
    }

    pub fn save_to_file(&mut self, file_name: &str) {
        let buffer = File::create(file_name).expect("Cannot create file");
        let compressed = ZlibEncoder::new(buffer, Compression::default());
        serde_json::to_writer(compressed, self).unwrap();
    }
   
    pub fn generate_random_word(&mut self, depth: usize, rng: &mut StdRng) -> String {
        // allocate empty string 
        let mut result = String::from("");
        let mut current = vec![START_WORD; depth];
        loop {
            // loop until stop character
            let cur_str: String = (&current).into_iter().collect();
            let c = self.random_transition(&self.transitions[&cur_str], rng);
            if c == END_WORD  {
                break;
            }
            result.push(c);
            current.remove(0);
            current.push(c);
        }
        result
    }

    fn random_transition(&self, v: &Vec<CumCount>, rng: &mut StdRng) -> char {
        let total_count = v[v.len()-1].cf;
        let random = rng.gen_range(0, total_count);
        let pos = v.binary_search_by_key(&random, |cv| cv.cf);
        let ix = match pos {
            Ok(i) => i+1,
            Err(i) => i
        };
        v[ix].c            
    }    
}




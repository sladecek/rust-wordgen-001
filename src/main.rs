use std::io::BufRead;
use std::collections::HashMap;
use rand::Rng;


// Builds information about transitions.
pub struct Builder {
    // Transition counts from c1 to c2.
    transitions: HashMap<char, HashMap<char, i32>>
}

impl Builder {
    pub fn new() -> Builder {
        Builder { transitions: HashMap::new() }
    }
    
    pub fn add_char_pair(&mut self, c1: char, c2: char, count: i32) {
        *self
            .transitions.entry(c1)
            .or_insert(HashMap::new())
            .entry(c2)
            .or_insert(0)
            += count;       
    }
}

struct CumCount {
    c: char,
    // Number of transitions to character c and all other characters with
    // code smaller than c.
    cf: i32
}

// Generates random words.
struct Generator {
    transitions: HashMap<char, Vec<CumCount>>    
}

impl Generator {

    pub fn new_from_builder(b: Builder) -> Generator {
        let mut result = Generator { transitions: HashMap::new() };
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
            result.transitions.insert(*c1, v);
        }
        result        
    }

    pub fn generate_random_word(&mut self) -> String {
        // allocate empty string 
        let mut result = String::from("");
        let mut current = START_WORD;
//      let mut c = 10i32;
        loop {
            // loop until stop character
            current = self.random_transition(&self.transitions[&current]);
            if current == END_WORD /*|| c < 1*/ {
                break;
            }
            result.push(current);
  //        c -= 1;
        }
        result
    }

    fn random_transition(&self, v: &Vec<CumCount>) -> char {
        let mut rng = rand::thread_rng();
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

const START_WORD: char = '^';
const END_WORD: char = '$';

fn main() {
    // Open file.
    let f = std::fs::File::open("wisilityinput/cs.dict").expect("error opening input file");
    let buf_reader = std::io::BufReader::new(f);
    
    let mut bld = Builder::new();
    
    // Read all lines.
    for line_result in buf_reader.lines() {
        let line = line_result.expect("error reading input file");
        let vec: Vec<&str> = line.split("\t").collect();
        if vec.len() > 0 {
            let word = vec[0];
            let mut count = 1;
            if vec.len() > 1 {
                count = vec[1].parse().unwrap();
            }
                
            let sz = word.len();
            if  sz > 0 {
                let s = format!("{}{}", word, END_WORD);
                let mut prev =  START_WORD;
                for c in s.chars() {
                    bld.add_char_pair(prev, c, count);
//                    println!("{} '{}'->'{}'", count, prev, c);
                    prev = c;
                }
            }
        }
    }
    
    // Create generator from the builder.
    let mut gen = Generator::new_from_builder(bld);
    
    // Generate 27 words.
    for _ in 0..277 {
        let w = gen.generate_random_word();
        println!("word: {}", w);
    }
    
}

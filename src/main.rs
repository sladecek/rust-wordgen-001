use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use rand::Rng;
use clap::{App, Arg};
use serde::{Serialize, Deserialize};
use serde_json::json;

// Builds information about transitions.
pub struct Builder {
    depth: usize,
    // Transition counts from c1 to c2.
    transitions: HashMap<String, HashMap<char, i32>>
}

impl Builder {
    pub fn new(depth: usize) -> Builder {
        Builder { depth, transitions: HashMap::new() }
    }
    
    pub fn add_char_pair(&mut self, c1: &String, c2: char, count: i32) {
        *(self
            .transitions
            .entry(c1.to_string())
            .or_insert(HashMap::new())
            .entry(c2)
          .or_insert(0)
          ) += count;       
    }

    pub fn add_pairs_from_string(&mut self, word: &String, count: i32) {
        let wchars: Vec<char>  = word.chars().collect();
        if  wchars.len() == 0 {
            return
        }
            
        let mut w: Vec<char> = vec![START_WORD; self.depth];
        w.extend(word.chars());
        w.extend(vec![END_WORD; self.depth]);

        for i in 0..wchars.len()+self.depth {
            let prev = w.get(i..i+self.depth).expect("apa").into_iter().collect();
            self.add_char_pair(&prev, w[i+self.depth], count);
        }
    }

    pub fn add_wordlist_file(&mut self, file_name: &str) {
        
        let f = std::fs::File::open(file_name).expect("error opening input file");
        let buf_reader = std::io::BufReader::new(f);
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
                self.add_pairs_from_string(&word.to_string(), count);
            }
        }        
    }

    pub fn add_text_file(&mut self, file_name: &str) {
        /* */
    }
}    


#[derive(Copy, Clone,Serialize,Deserialize)]
struct CumCount {
    c: char,
    // Number of transitions to character c and all other characters with
    // code smaller than c.
    cf: i32
}

// Generates random words.
#[derive(Serialize,Deserialize)]
struct Generator {
    depth: usize, 
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
        serde_json::from_reader(file).unwrap()
    }

    pub fn save_to_file(&mut self, file_name: &str) {
        let mut buffer = File::create(file_name).expect("Cannot create file");
        serde_json::to_writer(buffer, self).unwrap();
    }
   
    pub fn generate_random_word(&mut self, depth: usize) -> String {
        // allocate empty string 
        let mut result = String::from("");
        let mut current = vec![START_WORD; depth];
        loop {
            // loop until stop character
            let cur_str: String = (&current).into_iter().collect();
            let c = self.random_transition(&self.transitions[&cur_str]);
            if c == END_WORD  {
                break;
            }
            result.push(c);
            current.remove(0);
            current.push(c);
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


// Defines actions taken by the program.
struct Parameters {
    generate: bool,
    learn: bool,
    word_count: u32,
    depth: usize,
    dict_file: String,
    input_wordlists: Vec<String>,
    input_textfiles: Vec<String>    
}

impl Parameters {
    fn new() -> Parameters {
        Parameters {
            generate: false,
            learn: false,
            word_count: 1,
            depth: 2,
            dict_file: String::new(),
            input_wordlists: Vec::new(),
            input_textfiles: Vec::new()
        }
    }
}

fn parse_arguments() -> Parameters {

   let matches = App::new("rust-wordgen")
        .version("1.0")
        .author("Ladislav Sladecek <ladislav.sladecek@gmail.com>")
        .about("Generates random words based on n-gramm partial probability.")
        .arg(Arg::with_name("generate")            
             .long("generate")
             .short("g")
             .help("Generate random words")
             .takes_value(false)
             .conflicts_with("learn"))
        .arg(Arg::with_name("learn")            
             .long("learn")
             .short("l")
             .help("Create new dictionary from text files and wordlists.")
             .takes_value(false)
             .requires("depth")
             .requires("dict")
             )
        .arg(Arg::with_name("count")            
             .value_name("CNT")
             .long("count")
             .short("c")
             .help("Number of generated words")
             .conflicts_with("learn")
             .takes_value(true))
        .arg(Arg::with_name("depth")            
             .value_name("DEPTH")
             .long("depth")
             .short("d")
             .help("Set n-gramm depth")
             .takes_value(true)
             .conflicts_with("generate")
             )
        .arg(Arg::with_name("dict")            
             .value_name("DICT")
             .long("dict")
             .short("t")
             .help("Set dictionary name")
             .default_value("default.dict")             
             .takes_value(true))
        .arg(Arg::with_name("wl")            
             .value_name("WL")
             .long("input-word-list")
             .short("i")
             .multiple(true)
             .help("Input wordlist file")
             .conflicts_with("generate")
             .takes_value(true))
        .arg(Arg::with_name("if")            
             .value_name("IF")
             .long("input-file")
             .short("f")
             .multiple(true)
             .help("Input text file")
             .conflicts_with("generate")
             .takes_value(true))
        .get_matches();
    
    let mut parameters = Parameters::new();
    let c = matches.value_of("count");
    if c.is_some() {
        parameters.word_count = c.unwrap().parse().expect("Word count must be an integer");
    }

    parameters.generate = matches.is_present("generate");
    parameters.learn = matches.is_present("learn");

    let d = matches.value_of("depth");
    if d.is_some() {
        parameters.depth = d.expect("udef").parse().expect("Undefined depth");
    }

    parameters.dict_file = matches.value_of("dict").unwrap().to_string();

    let wl = matches.values_of("wl");
    if wl.is_some() {
        parameters.input_wordlists = wl.expect("wl").map(String::from).collect();
    }

    let inf = matches.values_of("if");
    if inf.is_some() {
        parameters.input_textfiles = inf.expect("if").map(String::from).collect();
    }
    parameters
}


fn main() {
    
    let parameters = parse_arguments();
    if parameters.generate {
        let mut gen = Generator::new_from_file(parameters.dict_file.as_str());
        for _ in 0..parameters.word_count {
            let w = gen.generate_random_word(gen.depth);
            println!("word: {}", w);
        }        
    } else if parameters.learn {
        let mut bld = Builder::new(parameters.depth);
        for i in parameters.input_wordlists {
            bld.add_wordlist_file(i.as_str());
        }
        for i in parameters.input_textfiles {
            bld.add_text_file(i.as_str())
        }
        let mut gen = Generator::new_from_builder(bld);
        gen.save_to_file(parameters.dict_file.as_str());
    } else {
        panic!("Nothing to do.");
    }
}

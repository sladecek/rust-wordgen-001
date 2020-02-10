use std::io::BufRead;
use std::collections::HashMap;
use rand::Rng;
use clap::{App, Arg};

// Builds information about transitions.
pub struct Builder {
    // Transition counts from c1 to c2.
    transitions: HashMap<String, HashMap<char, i32>>
}

impl Builder {
    pub fn new() -> Builder {
        Builder { transitions: HashMap::new() }
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

    pub fn add_pairs_from_string(&mut self, word: &String, count: i32, length: usize) {
        let wchars: Vec<char>  = word.chars().collect();
        if  wchars.len() == 0 {
            return
        }
            
        let mut w: Vec<char> = vec![START_WORD; length];
        w.extend(word.chars());
        w.extend(vec![END_WORD; length]);

        for i in 0..wchars.len()+length {
            let prev = w.get(i..i+length).unwrap().into_iter().collect();
            self.add_char_pair(&prev, w[i+length], count);
        }
    }
    
}

#[derive(Copy, Clone)]
struct CumCount {
    c: char,
    // Number of transitions to character c and all other characters with
    // code smaller than c.
    cf: i32
}

// Generates random words.
struct Generator {
    transitions: HashMap<String, Vec<CumCount>>    
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
            result.transitions.insert(c1.to_string(), v);
        }
        result        
    }

    pub fn generate_random_word(&mut self, length: usize) -> String {
        // allocate empty string 
        let mut result = String::from("");
        let mut current = vec![START_WORD; length];
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
    generated_word_count: u32,
    depth: u8,
    dict_file: Option<String>,
    input_wordlists: Vec<String>,
    input_textfiles: Vec<String>    
}

impl Parameters {
    fn new() -> Parameters {
        Parameters {
            generated_word_count: 0,
            depth: 2,
            dict_file: None,
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
             .value_name("CNT")
             .long("generate")
             .short("g")
             .help("Generates CNT random words")
             .takes_value(true))
        .arg(Arg::with_name("depth")            
             .value_name("DEPTH")
             .long("depth")
             .short("d")
             .help("Set n-gramm depth")
             .takes_value(true))
        .arg(Arg::with_name("dict")            
             .value_name("DICT")
             .long("dict")
             .short("c")
             .help("Set dictionary name")
             .takes_value(true))
        .arg(Arg::with_name("wl")            
             .value_name("WL")
             .long("input-word-list")
             .short("i")
             .multiple(true)
             .help("Input wordlist file")
             .takes_value(true))
        .arg(Arg::with_name("if")            
             .value_name("IF")
             .long("input-file")
             .short("f")
             .multiple(true)
             .help("Input text file")
             .takes_value(true))
        .get_matches();
    
    let mut parameters = Parameters::new();
    let g = matches.value_of("generate");
    if g.is_some() {
        parameters.generated_word_count = g.unwrap().parse().unwrap();
    }
    let d = matches.value_of("depth");
    if d.is_some() {
        parameters.depth = d.unwrap().parse().unwrap();
    }

    parameters.dict_file = matches.value_of("dict").map(String::from);

    let wl = matches.values_of("wl");
    if wl.is_some() {
        parameters.input_wordlists = wl.unwrap().map(String::from).collect();
    }

    let inf = matches.values_of("if");
    if inf.is_some() {
        parameters.input_textfiles = inf.unwrap().map(String::from).collect();
    }
    parameters
}


fn main() {
    
    let parameters = parse_arguments();
        
    // Open file.
    let f = std::fs::File::open("wisilityinput/cs.dict").expect("error opening input file");
    let buf_reader = std::io::BufReader::new(f);
    
    let mut bld = Builder::new();
    let length : usize = parameters.depth as usize;
    
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
            bld.add_pairs_from_string(&word.to_string(), count, length);
        }
    }
    
    // Create generator from the builder.
    let mut gen = Generator::new_from_builder(bld);
    
    for _ in 0..parameters.generated_word_count {
        let w = gen.generate_random_word(length);
        println!("word: {}", w);
    }
    
}

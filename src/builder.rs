use std::io::BufRead;
use std::collections::HashMap;

// Builds information about transitions.
pub struct Builder {
    pub depth: usize,
    // Transition counts from c1 to c2.
    pub transitions: HashMap<String, HashMap<char, i32>>
}

// Special symbols to denote the word boundary.
pub const START_WORD: char = '^';
pub const END_WORD: char = '$';

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
        let f = std::fs::File::open(file_name).expect("error opening input file");
        let buf_reader = std::io::BufReader::new(f);
        for line_result in buf_reader.lines() {
            let line = line_result.expect("error reading input file");
            self.add_text_line(&line)
        }
    }

    fn add_text_line(&mut self, line: &String) {
        let mut last_word: String = String::new();
        for c in line.chars() {
            if c.is_alphabetic() {
                last_word.push(c)
            } else {
                if !last_word.is_empty() {
                    self.add_pairs_from_string(&last_word, 1);
                    last_word = String::new();
                }
            }
        }
        if !last_word.is_empty() {
            self.add_pairs_from_string(&last_word, 1);
        }
    }    
}    


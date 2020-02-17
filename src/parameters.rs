use clap::{App, Arg};

// Defines actions taken by the program.
pub struct Parameters {
    pub generate: bool,
    pub learn: bool,
    pub word_count: u32,
    pub depth: usize,
    pub dict_file: String,
    pub input_wordlists: Vec<String>,
    pub input_textfiles: Vec<String>,
    pub use_seed: bool,
    pub seed: u64
}


impl Parameters {
    pub fn new() -> Parameters {
        Parameters {
            generate: false,
            learn: false,
            word_count: 1,
            depth: 2,
            dict_file: String::new(),
            input_wordlists: Vec::new(),
            input_textfiles: Vec::new(),
            use_seed: false,
            seed: 0

        }
    }
}

pub fn parse_arguments() -> Parameters {

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
        .arg(Arg::with_name("seed")            
             .value_name("SEED")
             .long("seed")
             .short("s")
             .help("Random seed")
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
    let s = matches.value_of("seed");
    if s.is_some() {
        parameters.use_seed = true;
        parameters.seed = s.unwrap().parse().expect("Seed must be an unsigned integer");
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



use rand::SeedableRng;
use rand::rngs::StdRng;

use wordgen::generator::{Generator};
use wordgen::builder::{Builder};
use wordgen::parameters::{parse_arguments};

fn main() {
    
    let parameters = parse_arguments();
    if parameters.generate {
        let mut gen = Generator::new_from_file(parameters.dict_file.as_str());
        let mut rng = if parameters.use_seed {
            StdRng::seed_from_u64(parameters.seed)
        } else {
            StdRng::from_entropy()
        };

        for _ in 0..parameters.word_count {
            let w = gen.generate_random_word(gen.depth, &mut rng);
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

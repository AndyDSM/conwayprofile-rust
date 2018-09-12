extern crate conwayprofile;
extern crate rand;

use conwayprofile::from_string;
use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut factor = 10u16;
    let mut string = String::new();
    if args.len() < 2 {
        let mut rng = rand::thread_rng();
        for _i in 0..32 {
            string.push(rng.gen_range(b'a', b'z') as char);
        }
        println!("No string defined, generating random string \"{}\".", string);
        println!("No scale factor defined, defaulting to 10.");
    } else if args.len() < 3 {
        string = args[1].clone();
        println!("Using string \"{}\".", string);
        println!("No scale factor defined, defaulting to 10.");
    } else {
        string = args[1].clone();
        println!("Using string \"{}\".", string);
        let factor_res = args[2].parse();
        match factor_res {
            Ok(x) => {
                factor = x;
                println!("Using scale factor \"{}\".", factor);
            },
            Err(_x) => {
                println!("Invalid scale factor format, defaulting to 10.");
            }
        }
    }
    from_string(&string, factor);
    println!("Gif's ready, champ.");
}

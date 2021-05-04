use std::env;
mod PerceptronRustNotebook;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: perceptron_source a b c\n       Where a,b,c are all float values\n");
    } else {
        let a: f64  = args[1].parse().unwrap();
        let b: f64  = args[2].parse().unwrap();
        let c: f64  = args[3].parse().unwrap();
        PerceptronRustNotebook::run(a,b,c);
    }
}




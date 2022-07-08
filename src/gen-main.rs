use std::env;
use std::cmp::Ordering;
use rand::prelude::SliceRandom;

mod polyomino;
mod generate;
mod encode;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let count = match args.get(1).and_then(|s| s.parse::<i32>().ok()) {
        Some(x) => x,
        _ => 4
    };
    
    let is_free = match args.get(2).map(|s| s.as_str()) {
        Some("free") => true,
        _ => false
    };

    let mut polyominos = generate::get_monomino();
    for _ in 1..count {
        polyominos = if is_free {
            generate::extend_polys(&polyominos, &polyomino::align_polyomino_free)
        } else {
            generate::extend_polys(&polyominos, &polyomino::align_polyomino)
        }
    }
    
    for poly in polyominos.iter() {
        // println!("{}\n", polyomino::format_poly(&poly));

        let encoded = encode::encode(poly);
        //println!("Testing\n{}\n{:}", polyomino::format_poly(&test_poly), encoded);
        let decoded = encode::decode(&encoded).unwrap();
        //println!("Result\n{}\n{:?}", polyomino::format_poly(&decoded), polyomino::cmp_poly(&test_poly, &decoded));
        assert_eq!(polyomino::cmp_poly(&poly, &decoded), Ordering::Equal)
    }
    
    eprintln!("Found {} polyominos", &polyominos.len());

    //let test_poly = polyominos.choose(&mut rand::thread_rng()).unwrap();
}

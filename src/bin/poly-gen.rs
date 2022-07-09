use std::env;
use std::process::exit;
use std::fs::File;
use std::io::*;
use std::path::*;
use polyominos::polyomino::*;
use polyominos::encode::encode;
use polyominos::generate::*;

fn write_header(h: &mut File) -> Result<()>{
    h.write_all(b"PIF\0\0\0\0\0\0\0\0\0\0\0\0\0")
}

fn write_polys(h: &mut File, n: i16, polys: &Vec<Polyomino>) -> Result<()> {
    h.write_all(b"\xff\0")?;
    h.write_all(&n.to_le_bytes())?;
    for poly in polys {
        let encoded = encode(&poly);
        h.write_all(encoded.as_raw_slice())?;
    }
    h.write_all(b"\0")
}

fn panic_if_fail(res: Result<()>, file: &Display) {
    match res {
        Ok(_) => (),
        Err(why) => panic!("Failed to write to {}: {}", file, why)

    };
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(x) => x,
        None => {
            println!("Usage: {} <filename> [max_order: 4] [one-sided|free]", 
                     args.get(0).unwrap());
            exit(1)
        }
    };
    
    let path = Path::new(filename);
    let display = path.display();
    let mut file_h = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", &display, why),
        Ok(file) => file,
    };


    let count = match args.get(2).and_then(|s| s.parse::<i16>().ok()) {
        Some(x) => x,
        _ => 4
    };
    
    let is_free = match args.get(3).map(|s| s.as_str()) {
        Some("free") => true,
        _ => false
    };
    
    panic_if_fail(write_header(&mut file_h), &display);

    let mut polyominos = get_monomino();
    panic_if_fail(write_polys(&mut file_h, 1, &polyominos), &display);


    for i in 2..count+1 {
        polyominos = if is_free {
            extend_polys(&polyominos, &align_polyomino_free)
        } else {
            extend_polys(&polyominos, &align_polyomino)
        };
        panic_if_fail(write_polys(&mut file_h, i, &polyominos), &display);
    }
}

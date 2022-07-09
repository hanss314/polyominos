use std::env;
use std::process::exit;
use std::fs::File;
use std::io::*;
use std::path::*;
use bitvec::prelude::*;

use polyominos::encode::*;
use polyominos::polyomino::*;

fn panic_if_fail<T>(res: Result<T>, file: &Display) -> T {
    match res {
        Ok(x) => x,
        Err(why) => panic!("Failed to read {}: {}", file, why)

    }
}

fn validate_header(h: &mut File) -> Result<bool> {
    let mut buf = [0u8; 3];
    h.read_exact(&mut buf)?;
    h.seek(SeekFrom::Start(16))?;
    Ok(buf == *b"PIF")
}

fn advance_to_order (h: &mut File, n: i16) -> Result<()> {
    let mut order_buf = [0u8; 2];
    loop {
        h.seek(SeekFrom::Current(2))?;
        h.read_exact(&mut order_buf)?;
        let order = i16::from_le_bytes(order_buf);
        if order == n {
            return Ok(())
        }

        let step = bytes_required(order);
        let mut peek_buf = [0u8; 1];
        h.read_exact(&mut peek_buf)?;

        while peek_buf[0] != 0 {
            h.seek(SeekFrom::Current((step-1).into()))?;
            h.read_exact(&mut peek_buf)?;
        }
    }
}

fn get_polyomino(h: &mut File, order: i16, ind: i64) -> Result<Option<Polyomino>> {
    let step = bytes_required(order);
    h.seek(SeekFrom::Current(ind*(step as i64)))?;
    
    let mut buf = vec![0u8; step as usize];
    h.read_exact(buf.as_mut_slice())?;

    let bitvec = BitVec::from_iter(buf.into_iter());

    Ok(decode(&bitvec))
}



fn print_usage_and_exit(name: &str) -> ! {
    println!("Usage: {} <filename> <order> <index>", name);
    exit(1)
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(x) => x,
        None => print_usage_and_exit(args.get(0).unwrap())
    };
    
    let path = Path::new(filename);
    let display = path.display();
    let mut file_h = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", &display, why),
        Ok(file) => file,
    };


    let order = match args.get(2).and_then(|s| s.parse::<i16>().ok()) {
        Some(x) => x,
        None => print_usage_and_exit(args.get(0).unwrap())
    };
    
    let index = match args.get(3).and_then(|s| s.parse::<i64>().ok()) {
        Some(x) => x,
        None => print_usage_and_exit(args.get(0).unwrap())
    };

    if !panic_if_fail(validate_header(&mut file_h), &display) {
        panic!("Not a PIF file")
    }

    panic_if_fail(advance_to_order(&mut file_h, order), &display);

    let polyomino = match panic_if_fail(get_polyomino(&mut file_h, order, index), &display) {
        None => panic!("Corrupt data"),
        Some(p) => p
    };

    println!("{}", format_poly(&polyomino));
}

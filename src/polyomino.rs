use std::cmp::Ordering;
use std::cmp::min_by;

pub type Point = (i16,i16);
pub type Polyomino = Vec<Point>;

pub fn align_polyomino(mut polyomino: Polyomino) -> Polyomino {
    polyomino.sort();
    polyomino.dedup();

    let mut copies = [polyomino.to_vec(), polyomino.to_vec(), polyomino.to_vec(), polyomino];
    let mut i=0;
    for p in copies.iter_mut() {
        rotatei(p, i);
        i+=1;
    }

    copies.into_iter().min_by(cmp_poly).unwrap()
}

pub fn align_polyomino_free(polyomino: Polyomino) -> Polyomino {
    let flipped = polyomino.iter().map(|(x,y)|(-x,*y)).collect();

    let aligned_polyomino = align_polyomino(polyomino);
    let aligned_flipped = align_polyomino(flipped);
    
    min_by(aligned_polyomino, aligned_flipped, cmp_poly)
}

pub fn cmp_poly(a: &Polyomino, b: &Polyomino) -> Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    for (x, y) in a.iter().zip(b.iter()) {
        if x != y {
            return x.cmp(y);
        }
    }
    return Ordering::Equal;
}

pub fn format_poly(poly: &Polyomino) -> String {
    let mut string = String::with_capacity(poly.len() * 10);
    let mut col = 0;
    let mut row = 0;
    for (x, y) in poly.iter() {
        for _ in row..*x {
            string.push('\n');
            col = 0;
        }
        row = *x;

        for _ in col..*y {
            string.push(' ');
        }
        string.push('*');
        col = y+1;
    }
    string
}

pub fn rotatei(polyomino: &mut Polyomino, times: i8) {
    let trans = match times.rem_euclid(4) {
        1 => |(x, y): (i16, i16)| (-y,  x),
        2 => |(x, y): (i16, i16)| (-x, -y),
        3 => |(x, y): (i16, i16)| ( y, -x),
        _ => |(x, y): (i16, i16)| ( x,  y),
    };
    let mut min_x = 100;
    let mut min_y = 100;
    for point in polyomino.iter_mut() {
        *point = trans(*point);
        if point.0 < min_x { min_x = point.0; }
        if point.1 < min_y { min_y = point.1; }
    }
    
    for point in polyomino.iter_mut() {
        *point = (point.0 - min_x, point.1 - min_y);
    }
    polyomino.sort();
}

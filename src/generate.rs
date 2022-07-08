use crate::polyomino::*;

pub fn get_monomino() -> Vec<Polyomino> {
    vec![vec![(0,0)]]
}

// Assumes poly is aligned
fn extensions(poly: &Polyomino, aligner: &dyn Fn(Polyomino) -> Polyomino) -> Vec<Polyomino> {
    let mut ext_points = vec![];
    for point in poly.iter() {
        for offset in [(0,1), (0,-1), (1, 0), (-1, 0)] {
            let new_point = (point.0+offset.0, point.1+offset.1);
            if poly.as_slice().binary_search(&new_point).is_err() {
                ext_points.push(new_point);
            }
        }
    }
    ext_points.sort();
    ext_points.dedup();

    ext_points.into_iter().map(|p| {
        let mut new_poly = poly.to_vec();
        new_poly.push(p);
        aligner(new_poly)
    }).collect()
}

pub fn extend_polys(polys: &Vec<Polyomino>, aligner: &dyn Fn(Polyomino) -> Polyomino) -> Vec<Polyomino>{
    let mut new_polys = Vec::new();
    for poly in polys.iter() {
        new_polys.append(&mut extensions(poly, aligner));
    }
    new_polys.sort_by(cmp_poly);
    new_polys.dedup_by(|a, b| cmp_poly(a,b).is_eq());
    new_polys
}

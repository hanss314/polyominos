use std::collections::VecDeque;
use std::vec::Vec;
use std::collections::HashSet;
use bitvec::prelude::*;

use crate::polyomino::*;

const DIRECTIONS: [(i16, i16); 4] = [
    (1,0), (0,1), (-1,0), (0,-1)];


fn get_directions(dir: i8, pos: Point) -> Vec<(i8, Point)> {
    let mut dirs = Vec::new();
    if dir == -1 {
        for i in 0i8..2 {
            let pt = (pos.0+DIRECTIONS[i as usize].0, pos.1+DIRECTIONS[i as usize].1);
            dirs.push((i, pt));
        }
    } else {
        for i in 0i8..4 {
            if (i+2)%4 == dir {
                continue;
            }
            let pt = (pos.0+DIRECTIONS[i as usize].0, pos.1+DIRECTIONS[i as usize].1);
            dirs.push((i as i8, pt));
        }
    }
    dirs
}

pub fn encode(poly: &Polyomino) -> BitVec<u8> {
    if poly.is_empty() {
        return BitVec::new();
    } 
    if poly.len() == 1 {
        return BitVec::from_iter(vec![0b0100 as u8].into_iter());
    }
    
    let mut bfs_seen: HashSet<Point> = HashSet::new();
    let mut bfs_queue: VecDeque<(i8, Point)> = VecDeque::new();
    bfs_queue.push_back((-1, *poly.get(0).unwrap()));
    bfs_seen.insert(*poly.get(0).unwrap());
    
    let mut encoding = BitVec::new();

    while let Some((from, pos)) = bfs_queue.pop_front() {
        for (to, neigh) in get_directions(from, pos).into_iter() {
            if !bfs_seen.contains(&neigh) && poly.binary_search(&neigh).is_ok() {
                bfs_seen.insert(neigh);
                bfs_queue.push_back((to, neigh));
                encoding.push(true);
            } else {
                encoding.push(false);
            }
        }
    }
    for _ in 0..3 {
        encoding.pop();
    }
    encoding
}

fn chunkify(bits: &BitVec<u8>) -> Option<Vec<i8>> {
    let mut ret = Vec::new();
    if bits.is_empty() {
        return Some(ret);
    }

    let mut is_first = true;
    let mut offset = 0;
    let mut curr_val = 0;

    for bit in bits.iter().by_vals() {
        if bit {
            curr_val |= 1 << offset;
        }
        offset += 1;
        if (is_first && offset == 2) || offset >= 3 {
            ret.push(curr_val);
            curr_val = 0;
            offset = 0;
            is_first = false;
        }
    }
    if curr_val != 0 {
        None
    } else {
        Some(ret)
    }
}

pub fn bytes_required(n: i16) -> i16 {
    if n <= 0 {
        1
    } else if n == 1 {
        1
    } else {
        let x = 3*n-4;
        (x+7)/8
    }
}

pub fn decode(encoded: &BitVec<u8>) -> Option<Polyomino> {
    if encoded.starts_with(bits![0,0,1]) {
        return Some(vec![(0,0)]);
    }

    let mut poly = Vec::new();
    let mut bfs_queue: VecDeque<(i8, Point)> = VecDeque::new();

    bfs_queue.push_back((-1, (0,0)));

    for bfs_to in chunkify(encoded)? {
        if bfs_queue.is_empty() && bfs_to == 0 {
            break;
        }

        let (from, pos) = bfs_queue.pop_front()?;
        for (i, next) in get_directions(from, pos).into_iter().enumerate() {
            if bfs_to & (1 << i) != 0 {
                bfs_queue.push_back(next);
            }
        }
        poly.push(pos);
    }

    while let Some((_, pos)) = bfs_queue.pop_front() {
        poly.push(pos);
    }
    rotatei(&mut poly, 0);

    Some(poly)
}

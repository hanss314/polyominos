use bitvec::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

use crate::encode::*;
use crate::polyomino::*;

#[wasm_bindgen]
pub fn validate_header(h_se: &Uint8Array) -> bool {
    let h = h_se.to_vec();
    h.starts_with(b"PIF")
}

#[wasm_bindgen]
pub fn get_orders(h_se: &Uint8Array) -> JsValue {
    let h = h_se.to_vec();
    let mut hash_map: HashMap<i16, (usize, i32)> = HashMap::new();
    let mut ind = 16;
    
    ind += 2;
    while let Some(order) = h.get(ind..ind+2).map(|x| i16::from_le_bytes(x.try_into().unwrap_or_default())) {
        ind += 2;
        let start_ind = ind;
        
        let step = bytes_required(order) as usize;
        let mut count = 0;
        while let Some(first) = h.get(ind) {
            if *first == 0 {
                ind += 1;
                break;
            }
            ind += step;
            count += 1;
        }
        hash_map.insert(order, (start_ind, count));

        ind += 2;
    }

    JsValue::from_serde(&hash_map).unwrap()
}

#[wasm_bindgen]
pub fn get_polyomino(h_se: &Uint8Array, order: i16, ind: i32, 
                 order_map_se: &JsValue) -> JsValue {

    let h: Vec<u8> = h_se.to_vec();
    let order_map: HashMap<i16, (usize, i32)> = order_map_se.into_serde().unwrap();

    let none = JsValue::from_serde(&(None as Option<Polyomino>)).unwrap();

    let step = bytes_required(order) as usize;
    if let Some((start_index, total)) = order_map.get(&order) {
        if ind >= *total {
            return none;
        }

        let index = start_index + step * (ind as usize);
        let encoded = h.get(index..index+step);
        let decoded = encoded.and_then(|e| decode(&BitVec::from_iter(e)));
        JsValue::from_serde(&decoded).unwrap()
    } else {
        none
    }
}

#[wasm_bindgen]
pub fn get_polyominos(h_se: &Uint8Array, order: i16, 
                 order_map_se: &JsValue) -> JsValue {
    
    let h: Vec<u8> = h_se.to_vec();
    let order_map: HashMap<i16, (usize, i32)> = order_map_se.into_serde().unwrap();

    let step = bytes_required(order) as usize;
    let mut vec: Vec<Polyomino> = Vec::new();

    if let Some((start_index, total)) = order_map.get(&order) {
        for i in 0..*total {
            let index = start_index + step * (i as usize);
            let encoded = h.get(index..index+step);
            if let Some(p) = encoded.and_then(|e| decode(&BitVec::from_iter(e))) {
                vec.push(p)
            }
        }
    }
    JsValue::from_serde(&vec).unwrap()
}



extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

#[wasm_bindgen]
pub struct VariableMesh {
    pub px: Vec<f64>,
    pub py: Vec<f64>,
    pub pdx: Vec<f64>,
    pub pdy: Vec<f64>,
    pub val: Vec<f64>,
}

#[wasm_bindgen]
impl VariableMesh {
    #[wasm_bindgen(constructor)]
    pub fn new(
        px: Vec<f64>,
        py: Vec<f64>,
        pdx: Vec<f64>,
        pdy: Vec<f64>,
        val: Vec<f64>,
        //values: HashMap<String, Vec<f64>>,
    ) -> VariableMesh {
        VariableMesh {
            px,
            py,
            pdx,
            pdy,
            val,
        }
    }
}
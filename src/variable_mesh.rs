extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

#[wasm_bindgen]
pub struct VariableMesh {
    px: Vec<f64>,
    py: Vec<f64>,
    pdx: Vec<f64>,
    pdy: Vec<f64>,
    val: Vec<f64>,
}

pub struct VariablePixel{
    pub px: f64,
    pub py: f64,
    pub pdx: f64,
    pub pdy: f64,
    pub val: f64,
}

pub struct VariablePixelIterator<'a>  {
    mesh: &'a VariableMesh,
    index: usize,
    values: &'a Vec<f64>,
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

impl VariableMesh {
    pub fn iter<'a>(&'a self) -> VariablePixelIterator<'a> {
        VariablePixelIterator {
            mesh: self,
            index: 0,
            values: &self.val
        }
    }
}

impl<'a> Iterator for VariablePixelIterator<'a> {
    type Item = VariablePixel;

    fn next(&mut self) -> Option<VariablePixel> {

        if self.index >= self.mesh.px.len() {
            None
        } else {
            self.index += 1;
            Some(VariablePixel {
                px: self.mesh.px[self.index - 1],
                py: self.mesh.py[self.index - 1],
                pdx: self.mesh.pdx[self.index - 1],
                pdy: self.mesh.pdy[self.index - 1],
                val: self.values[self.index - 1],
            })
        }
    }
}
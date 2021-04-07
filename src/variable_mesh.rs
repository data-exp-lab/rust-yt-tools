extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct VariableMesh {
    px: Vec<f64>,
    py: Vec<f64>,
    pz: Vec<f64>,
    pdx: Vec<f64>,
    pdy: Vec<f64>,
    pdz: Vec<f64>,
    field_values: HashMap<String, Vec<f64>>,
}

#[derive(Clone, Debug, Copy)]
pub struct VariablePixel {
    pub px: f64,
    pub py: f64,
    pub pz: f64,
    pub pdx: f64,
    pub pdy: f64,
    pub pdz: f64,
    pub val: f64,
}

pub struct VariablePixelIterator<'a> {
    mesh: &'a VariableMesh,
    index: usize,
    values: &'a Vec<f64>,
}

#[wasm_bindgen]
impl VariableMesh {
    #[wasm_bindgen(constructor)]
    pub fn new(px: Vec<f64>, py: Vec<f64>, pdx: Vec<f64>, pdy: Vec<f64>, pz: Option<Vec<f64>>, pdz: Option<Vec<f64>>) -> VariableMesh {
        let size = px.len();
        if !((size == py.len()) && (size == pdx.len()) && (size == pdy.len())) {
            // This should eventually be a Result
            panic!(
                "Size mismatch for Vector components: {:?}, {:?}, {:?}, {:?}",
                px.len(),
                py.len(),
                pdx.len(),
                pdy.len()
            );
        }
        let mut field_values = HashMap::new();
        let default_values = vec![1.0f64; size];
        let pz = match pz {
            Some(v) => v,
            None => vec![0.0; size],
        };
        let pdz = match pdz {
            Some(v) => v,
            None => vec![1.0; size],
        };
        if !((size == pz.len()) && (size == pdz.len())) {
            // This should eventually be a Result
            panic!(
                "Size mismatch for Vector components: {:?}, {:?}, {:?}",
                size,
                pz.len(),
                pdz.len()
            );
        }
        field_values.insert(String::from("ones"), default_values);
        VariableMesh {
            px,
            py,
            pz,
            pdx,
            pdy,
            pdz,
            field_values,
        }
    }
    pub fn add_field(&mut self, name: &str, field_values: Vec<f64>) {
        self.field_values.insert(String::from(name), field_values);
    }
    pub fn has_field(&self, name: &str) -> bool {
        self.field_values.contains_key(name)
    }
}

impl VariableMesh {
    pub fn iter(&'_ self, field: &str) -> VariablePixelIterator<'_> {
        let values = self.field_values.get(field).unwrap();
        VariablePixelIterator {
            mesh: self,
            index: 0,
            values: &values,
        }
    }
}

impl<'a> Iterator for VariablePixelIterator<'_> {
    type Item = VariablePixel;

    fn next(&mut self) -> Option<VariablePixel> {
        if self.index >= self.mesh.px.len() {
            None
        } else {
            self.index += 1;
            Some(VariablePixel {
                px: self.mesh.px[self.index - 1],
                py: self.mesh.py[self.index - 1],
                pz: self.mesh.pz[self.index - 1],
                pdx: self.mesh.pdx[self.index - 1],
                pdy: self.mesh.pdy[self.index - 1],
                pdz: self.mesh.pdz[self.index - 1],
                val: self.values[self.index - 1],
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_variable_mesh() {
        // Create a new variable mesh with basic values

        let _vm_test = VariableMesh::new(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            None,
            None
        );
    }

    #[test]
    #[should_panic]
    fn create_bad_variable_mesh() {
        let _vm_test = VariableMesh::new(
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![1.0, 2.0, 3.0, 4.0],
            None,
            None
        );
    }

    #[test]
    fn test_iterator() {
        let mut px: Vec<f64> = Vec::new();
        let mut py: Vec<f64> = Vec::new();
        let mut pdx: Vec<f64> = Vec::new();
        let mut pdy: Vec<f64> = Vec::new();
        let mut val: Vec<f64> = Vec::new();
        for i in 0..1024 * 1024 {
            // Just toss some random stuff in here
            px.push((i as f64) * 1.0);
            py.push((i as f64) * 1.2);
            pdx.push((i as f64) * 0.21);
            pdy.push((i as f64) * 0.22);
            val.push((i as f64) * 4.05);
        }
        let mut vm = VariableMesh::new(px, py, pdx, pdy, None, None);
        vm.add_field("default", val);
        for (i, pixel) in vm.iter("default").enumerate() {
            assert_eq!(pixel.px, (i as f64) * 1.0);
            assert_eq!(pixel.py, (i as f64) * 1.2);
            assert_eq!(pixel.pdx, (i as f64) * 0.21);
            assert_eq!(pixel.pdy, (i as f64) * 0.22);
            assert_eq!(pixel.val, (i as f64) * 4.05);
        }
        assert_eq!(vm.iter("default").count(), 1024 * 1024);
        for (i, pixel) in vm.iter("ones").enumerate() {
            assert_eq!(pixel.px, (i as f64) * 1.0);
            assert_eq!(pixel.py, (i as f64) * 1.2);
            assert_eq!(pixel.pdx, (i as f64) * 0.21);
            assert_eq!(pixel.pdy, (i as f64) * 0.22);
            assert_eq!(pixel.val, 1.0);
        }
        assert_eq!(vm.iter("ones").count(), 1024 * 1024);
    }
}

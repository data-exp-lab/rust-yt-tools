#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use std::f64;
use std::mem::size_of;
use std::convert::{From, Into};

#[wasm_bindgen]
pub struct FixedResolutionBuffer {
  buffer: Vec<f64>,
  width: usize,
  height: usize,
  x_low: f64,
  x_high: f64,
  y_low: f64,
  y_high: f64,
  ipdx: f64,
  ipdy: f64,
}

#[wasm_bindgen]
pub struct VariableMesh {
  px: Vec<f64>,
  py: Vec<f64>,
  pdx: Vec<f64>,
  pdy: Vec<f64>,
  val: Vec<f64>,
}

#[wasm_bindgen]
impl FixedResolutionBuffer {
  #[wasm_bindgen]
  pub fn new(width: usize, height: usize,
             x_low: f64,
             x_high: f64,
             y_low: f64,
             y_high: f64) -> FixedResolutionBuffer {
    let ipdx = width as f64 / (x_high - x_low);
    let ipdy = height as f64 / (y_high - y_low);
    let mut buffer: Vec<f64> = Vec::with_capacity(width*height);

    FixedResolutionBuffer {
        buffer,
        width,
        height,
        x_low, x_high,
        y_low, y_high,
        ipdx,
        ipdy,
    }
  }

  #[wasm_bindgen]
  pub fn deposit(&mut self, vmesh: &VariableMesh, pix_i: usize) {
    // Compute our left edge pixel
    
    if vmesh.px[pix_i] + vmesh.pdx[pix_i] < self.x_low {
        return;
    } else if vmesh.py[pix_i] + vmesh.pdy[pix_i] < self.y_low {
        return;
    } else if vmesh.px[pix_i] - vmesh.pdx[pix_i] > self.x_high {
        return;
    } else if vmesh.py[pix_i] - vmesh.pdy[pix_i] > self.y_high {
        return;
    }
    let lc: usize = (((vmesh.px[pix_i] - vmesh.pdx[pix_i] - self.x_low) * self.ipdx - 1.0)
                    .floor() as usize);
    let lr: usize = (((vmesh.py[pix_i] - vmesh.pdy[pix_i] - self.y_low) * self.ipdy - 1.0)
                    .floor() as usize);
    let rc: usize = (((vmesh.px[pix_i] + vmesh.pdx[pix_i] - self.x_low) * self.ipdx + 1.0)
                    .floor() as usize);
    let rr: usize = (((vmesh.py[pix_i] + vmesh.pdy[pix_i] - self.y_low) * self.ipdy + 1.0)
                    .floor() as usize);

    for i in lc.max(0)..rc.min(self.width) {
        for j in lr.max(0)..rr.min(self.height) {
            self.buffer[i * self.width + j] = vmesh.val[pix_i];
        }
    }
  }

  #[wasm_bindgen]
  pub fn deposit_all(&mut self, vmesh: VariableMesh) {
      for i in 0..vmesh.px.len() {
          self.deposit(&vmesh, i);
      }
  }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

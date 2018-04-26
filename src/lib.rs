#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use std::f64;
use std::mem::size_of;
use std::convert::{From, Into};
use std::collections::HashMap;

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
impl VariableMesh {
  pub fn new(px: Vec<f64>, py: Vec<f64>, pdx: Vec<f64>, pdy: Vec<f64>, val: Vec<f64>) -> VariableMesh {
      VariableMesh {
          px, py, pdx, pdy, val
      }
  }
}

pub fn get_normalizer(name: String) -> (fn(f64) -> f64) {
    let f: fn(f64) -> f64 = match(name.to_lowercase().as_ref()) {
        "log" => |f| f.log10(),
        "linear" => |f| f,
        _ => |f| f,
    };
    f
}

#[wasm_bindgen]
pub struct Colormaps {
    // These colormaps are stored unrolled, such that they are RGBA RGBA RGBA
  color_maps: HashMap<String, Vec<u8>>,
}

#[wasm_bindgen]
impl Colormaps {
  pub fn new() -> Colormaps {
    let mut color_maps = HashMap::new();
    let mut default_cmap: Vec<u8> = Vec::with_capacity(4 * 256);
    let mut i: usize = 0;
    default_cmap.resize(4*256, 0);
    for i in 0..256 {
        default_cmap[i * 4 + 0] = i as u8;
        default_cmap[i * 4 + 1] = i as u8;
        default_cmap[i * 4 + 2] = i as u8;
        default_cmap[i * 4 + 3] = 255;
    }
    color_maps.insert(String::from("default"), default_cmap.clone());
    Colormaps {
        color_maps,
    }
  }

  pub fn add_colormap(&mut self, name: String, table: Vec<u8>) {
    self.color_maps.insert(name, table);
  }

  // Once we have Option support in wasm-bindgen we'll be able to get rid of these.
  pub fn normalize(&mut self, name: String, buffer: Vec<f64>, take_log: bool) -> Vec<u8> {
    self.normalize_(name, buffer, None, None, take_log)
  }

  pub fn normalize_min(&mut self, name: String, buffer: Vec<f64>, min_val: f64, take_log: bool) -> Vec<u8> {
    self.normalize_(name, buffer, Some(min_val), None, take_log)
  }

  pub fn normalize_max(&mut self, name: String, buffer: Vec<f64>, max_val: f64, take_log: bool) -> Vec<u8> {
    self.normalize_(name, buffer, None, Some(max_val), take_log)
  }

  pub fn normalize_min_max(&mut self, name: String, buffer: Vec<f64>, min_val: f64, max_val: f64, take_log: bool) -> Vec<u8> {
    self.normalize_(name, buffer, Some(min_val), Some(max_val), take_log)
  }
}

// Note that this is a separate impl block, so we do not have the wasm code generated for it; as of
// the time of writing, Option did not work.
impl Colormaps {
  pub fn normalize_(&mut self, name: String, buffer: Vec<f64>, min_val: Option<f64>, max_val: Option<f64>,
                   take_log: bool) -> Vec<u8> {
    let f = match(take_log) {
        true => get_normalizer("log".to_string()),
        false => get_normalizer("linear".to_string()),
    };
    let mut cmin_val: f64 = 0.0;
    let mut cmax_val: f64 = 0.0;
    if min_val == None || max_val == None {
        log_f64("We're looking for something ...", 0.0);
      cmin_val = f64::MAX;
      cmax_val = f64::MIN;
      for i in 0..buffer.len() {
          cmin_val = cmin_val.min(buffer[i]);
          cmax_val = cmax_val.max(buffer[i]);
      }
      log_f64("Found ", cmin_val);
      log_f64("Found ", cmax_val);
    }
    cmin_val = match(min_val) {
        Some(v) => v,
        None => cmin_val,
    };
    cmax_val = match(max_val) {
        Some(v) => v,
        None => cmax_val,
    };
    let mut image: Vec<u8> = Vec::with_capacity(buffer.len() * 4);
    if !self.color_maps.contains_key(&name) {
        let name = "default";
        log_f64("Choosing default colormap", 1.0);
    }
    let cmap = match self.color_maps.get(&name) {
        Some(cmap) => cmap,
        None => panic!("Colormap {:?} does not exist.", name)
    };
    log_f64("cmin_val", cmin_val);
    log_f64("cmax_val", cmax_val);
    cmin_val = f(cmin_val);
    cmax_val = f(cmax_val);
    log_f64("cmin_val", cmin_val);
    log_f64("cmax_val", cmax_val);
    image.resize(buffer.len() * 4, 0);
    for i in 0..buffer.len() {
        let scaled = ((f(buffer[i]) - cmin_val)/(cmax_val - cmin_val))
                     .min(1.0).max(0.0);
        let bin_id = (scaled * 255.0) as usize;
        image[i*4 + 0] = cmap[bin_id * 4 + 0];
        image[i*4 + 1] = cmap[bin_id * 4 + 1];
        image[i*4 + 2] = cmap[bin_id * 4 + 2];
        image[i*4 + 3] = cmap[bin_id * 4 + 3];
    }
    image
  }
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
    buffer.resize(width * height, 0.0);

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
  pub fn get_buffer(&mut self) -> Vec<f64> {
    self.buffer.clone()
  }

  #[wasm_bindgen]
  pub fn dump_image(&mut self) -> Vec<u8> {
      let mi = f64::MAX;
      let ma = f64::MIN;
    for i in 0..self.width {
        for j in 0..self.height {
            let mi = mi.min(self.buffer[i * self.width + j]);
            let ma = mi.max(self.buffer[i * self.width + j]);
        }
    }
    let mi = mi.log10();
    let ma = ma.log10();
    let mut image: Vec<u8> = Vec::with_capacity(self.width * self.height * 4);
    image.resize(self.width * self.height * 4, 0);
    for i in 0..self.width {
        for j in 0..self.height {
            let ind = i * self.width * 4;
            let scaled = (self.buffer[i*self.width + j].log10() - mi)/(ma - mi);
            image[ind + 0] = (scaled * 255.0) as u8;
            image[ind + 1] = (scaled * 255.0) as u8;
            image[ind + 2] = (scaled * 255.0) as u8;
            image[ind + 3] = 255;
        }
    }
    image
  }

  #[wasm_bindgen]
  pub fn deposit(&mut self, vmesh: &VariableMesh) -> u32 {
    let mut count : u32 = 0;
    for pix_i in 0..vmesh.px.len() {
        // Compute our left edge pixel
        if vmesh.px[pix_i] + vmesh.pdx[pix_i] < self.x_low {
            continue;
        } else if vmesh.py[pix_i] + vmesh.pdy[pix_i] < self.y_low {
            continue;
        } else if vmesh.px[pix_i] - vmesh.pdx[pix_i] > self.x_high {
            continue;
        } else if vmesh.py[pix_i] - vmesh.pdy[pix_i] > self.y_high {
            continue;
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
                count = count + 1;
            }
        }
    }
    count
  }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(s: &str, a: f64);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(s: &str, a: u32);
}

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::convert::From;
use std::f64;

#[wasm_bindgen]
pub struct RGBAValue {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[wasm_bindgen]
pub struct Colormap {
    name: String,
    table: Vec<RGBAValue>,
}

#[wasm_bindgen]
pub struct FixedResolutionBuffer {
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


impl RGBAValue {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> RGBAValue {
        RGBAValue { red, green, blue, alpha }
    }
}

#[wasm_bindgen]
impl Colormap {
    #[wasm_bindgen(constructor)]
    pub fn new(
        name: String,
        rgba: Vec<u8>,
    ) -> Colormap {
        if rgba.len() % 4 != 0 {
            panic!("Needs RGBA flattened.");
        }
        let mut table: Vec<RGBAValue> = Vec::new();
        for i in 0..rgba.len()/4 {
            table.push( RGBAValue::new(
                rgba[i * 4 + 0],
                rgba[i * 4 + 1],
                rgba[i * 4 + 2],
                rgba[i * 4 + 3]
            ));
        }
        Colormap {
            name: name.clone(),
            table: table,
        }
    }
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

pub fn get_normalizer(name: String) -> (fn(f64) -> f64) {
    let f: fn(f64) -> f64 = match name.to_lowercase().as_ref() {
        "log" => |f| f.log10(),
        "linear" => |f| f,
        _ => |f| f,
    };
    f
}

#[wasm_bindgen]
pub struct ColormapCollection {
    color_maps: HashMap<String, Colormap>,
}

#[wasm_bindgen]
impl ColormapCollection {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ColormapCollection {
        let mut color_maps = HashMap::new();
        let mut default_cmap: Vec<RGBAValue> = Vec::new();
        for i in 0..256 {
            default_cmap.push(RGBAValue::new(i as u8, i as u8, i as u8, 255));
        }
        color_maps.insert(String::from("default"), 
            Colormap {
                name: String::from("default"),
                table: default_cmap,
            }
        );
        ColormapCollection { color_maps }
    }

    pub fn add_colormap(&mut self, name: String, table: Vec<u8>) {
        self.color_maps.insert(name.clone(), Colormap::new(name, table));
    }

    pub fn normalize(
        &mut self,
        name: String,
        buffer: Vec<f64>,
        image: &mut [u8],
        min_val: Option<f64>,
        max_val: Option<f64>,
        take_log: bool,
    ) {
        let f = match take_log {
            true => get_normalizer("log".to_string()),
            false => get_normalizer("linear".to_string()),
        };
        let mut cmin_val: f64 = 0.0;
        let mut cmax_val: f64 = 0.0;
        if min_val == None || max_val == None {
            cmin_val = f64::MAX;
            cmax_val = f64::MIN;
            for v in &buffer {
                cmin_val = cmin_val.min(*v);
                cmax_val = cmax_val.max(*v);
            }
        }
        cmin_val = match min_val {
            Some(v) => v,
            None => cmin_val,
        };
        cmax_val = match max_val {
            Some(v) => v,
            None => cmax_val,
        };
        let cmap = match self.color_maps.get(&name) {
            Some(cmap) => cmap,
            None => panic!("Colormap {:?} does not exist.", name),
        };
        cmin_val = f(cmin_val);
        cmax_val = f(cmax_val);
        for (i, &x) in buffer.iter().enumerate() {
            let scaled = ((f(x) - cmin_val) / (cmax_val - cmin_val))
                .min(1.0)
                .max(0.0);
            let bin_id = (scaled * 255.0) as usize;
            image[i * 4 + 0] = cmap.table[bin_id].red;
            image[i * 4 + 1] = cmap.table[bin_id].green;
            image[i * 4 + 2] = cmap.table[bin_id].blue;
            image[i * 4 + 3] = cmap.table[bin_id].alpha;
        }
    }
}

#[wasm_bindgen]
impl FixedResolutionBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: usize,
        height: usize,
        x_low: f64,
        x_high: f64,
        y_low: f64,
        y_high: f64,
    ) -> FixedResolutionBuffer {
        let ipdx = width as f64 / (x_high - x_low);
        let ipdy = height as f64 / (y_high - y_low);

        FixedResolutionBuffer {
            width,
            height,
            x_low,
            x_high,
            y_low,
            y_high,
            ipdx,
            ipdy,
        }
    }

    pub fn deposit(&mut self, vmesh: &VariableMesh, buffer: &mut [f64]) -> u32 {
        let mut count: u32 = 0;

        // We do need to clear the buffer -- in cases where the buffer is completely filled this
        // will result in extra work being done, but the alternate is to allocate a bunch of memory
        // and do filling of values anyway, so it may be the best we can do.
        for val in buffer.iter_mut() {
            *val = 0.0;
        }
        let mut image_buffer: Vec<&mut [f64]> = buffer.chunks_exact_mut( self.height ).collect();

        for pix_i in 0..vmesh.px.len() {
            // Compute our left edge pixel
            if vmesh.px[pix_i] + vmesh.pdx[pix_i] < self.x_low ||
               vmesh.py[pix_i] + vmesh.pdy[pix_i] < self.y_low ||
               vmesh.px[pix_i] - vmesh.pdx[pix_i] > self.x_high ||
               vmesh.py[pix_i] - vmesh.pdy[pix_i] > self.y_high {
                continue;
            }
            let lc: usize = ((vmesh.px[pix_i] - vmesh.pdx[pix_i] - self.x_low) * self.ipdx - 1.0)
                .floor() as usize;
            let lr: usize = ((vmesh.py[pix_i] - vmesh.pdy[pix_i] - self.y_low) * self.ipdy - 1.0)
                .floor() as usize;
            let rc: usize = ((vmesh.px[pix_i] + vmesh.pdx[pix_i] - self.x_low) * self.ipdx + 1.0)
                .floor() as usize;
            let rr: usize = ((vmesh.py[pix_i] + vmesh.pdy[pix_i] - self.y_low) * self.ipdy + 1.0)
                .floor() as usize;

            for i in lc.max(0)..rc.min(self.width) {
                for j in lr.max(0)..rr.min(self.height) {
                    image_buffer[i][j] = vmesh.val[pix_i];
                    count += 1;
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(s: &str, a: f64);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(s: &str, a: u32);
}

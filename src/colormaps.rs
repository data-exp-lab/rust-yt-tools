extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::f64;

pub fn get_normalizer(name: String) -> (fn(f64) -> f64) {
    let f: fn(f64) -> f64 = match name.to_lowercase().as_ref() {
        "log" => |f| f.log10(),
        "linear" => |f| f,
        _ => |f| f,
    };
    f
}

#[wasm_bindgen]
pub struct RGBAValue {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[wasm_bindgen]
pub struct Colormap {
    table: Vec<RGBAValue>,
}

#[wasm_bindgen]
pub struct ColormapCollection {
    color_maps: HashMap<String, Colormap>,
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
            table: table,
        }
    }
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
                table: default_cmap,
            }
        );
        ColormapCollection { color_maps }
    }

    pub fn add_colormap(&mut self, name: String, table: Vec<u8>) {
        self.color_maps.insert(name.clone(), Colormap::new(table));
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
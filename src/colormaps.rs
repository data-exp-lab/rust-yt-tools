extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::f64;

pub fn get_normalizer(name: String) -> fn(f64) -> f64 {
    let f: fn(f64) -> f64 = match name.to_lowercase().as_ref() {
        "log" => |f| f.log10(),
        "linear" => |f| f,
        _ => |f| f,
    };
    f
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct RGBAValue {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct Colormap {
    table: Vec<RGBAValue>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct ColormapCollection {
    color_maps: HashMap<String, Colormap>,
}

impl RGBAValue {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> RGBAValue {
        RGBAValue {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[wasm_bindgen]
impl Colormap {
    #[wasm_bindgen(constructor)]
    pub fn new(rgba: Vec<u8>) -> Colormap {
        if rgba.len() % 4 != 0 {
            panic!("Needs RGBA flattened.");
        }
        let mut table: Vec<RGBAValue> = Vec::new();
        for i in 0..rgba.len() / 4 {
            table.push(RGBAValue::new(
                rgba[i * 4],
                rgba[i * 4 + 1],
                rgba[i * 4 + 2],
                rgba[i * 4 + 3],
            ));
        }
        Colormap { table }
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
        color_maps.insert(
            String::from("default"),
            Colormap {
                table: default_cmap,
            },
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
        let f = if take_log {
            get_normalizer("log".to_string())
        } else {
            get_normalizer("linear".to_string())
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
        let tsize = cmap.table.len();
        for (i, &x) in buffer.iter().enumerate() {
            let scaled = ((f(x) - cmin_val) / (cmax_val - cmin_val))
                .min(1.0)
                .max(0.0);
            let bin_id = ((scaled * (tsize as f64)) as usize).max(0).min(tsize - 1);
            image[i * 4] = cmap.table[bin_id].red;
            image[i * 4 + 1] = cmap.table[bin_id].green;
            image[i * 4 + 2] = cmap.table[bin_id].blue;
            image[i * 4 + 3] = cmap.table[bin_id].alpha;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn linear_ramp_cmap() -> Vec<u8> {
        let mut rgba_map: Vec<u8> = Vec::new();
        for i in 0..256 {
            rgba_map.push(i as u8);
            rgba_map.push(0);
            rgba_map.push(0);
            rgba_map.push(255);
        }
        rgba_map
    }

    #[test]
    fn create_colormap() {
        // We will make a vector. This is in R, G, B, A order.
        // We will make it with linearly ramping R, 255 A, and 0 elsewhere.
        let rgba_map = linear_ramp_cmap();
        let _cm = Colormap::new(rgba_map);
        // Test that our pixels are in the right order.
        for (i, rgba) in _cm.table.iter().enumerate() {
            assert_eq!(i as u8, rgba.red);
            assert_eq!(0, rgba.green);
            assert_eq!(0, rgba.blue);
            assert_eq!(255, rgba.alpha);
        }
    }

    #[test]
    #[should_panic]
    fn create_bad_colormap() {
        let mut rgba_map = linear_ramp_cmap().clone();
        rgba_map.pop();
        let _cm = Colormap::new(rgba_map);
    }

    #[test]
    fn create_colormap_collection() {
        let mut cmap_collection = ColormapCollection::new();
        cmap_collection.add_colormap("simple".to_string(), linear_ramp_cmap());

        // Create a normalized f64 buffer
        let mut ibuf: Vec<f64> = Vec::new();
        for i in 0..256 {
            ibuf.push((i as f64) / 256.0);
        }

        // Our output image
        let mut obuf: Vec<u8> = Vec::new();
        obuf.resize(256 * 4, 0);

        cmap_collection.normalize(
            "default".to_string(),
            ibuf.clone(),
            obuf.as_mut_slice(),
            None,
            None,
            false,
        );

        for (i, rgba) in obuf.chunks_exact(4).enumerate() {
            assert_eq!(rgba[0], i as u8);
            assert_eq!(rgba[1], i as u8);
            assert_eq!(rgba[2], i as u8);
            assert_eq!(rgba[3], 255);
        }

        cmap_collection.normalize(
            "simple".to_string(),
            ibuf.clone(),
            obuf.as_mut_slice(),
            None,
            None,
            false,
        );

        for (i, rgba) in obuf.chunks_exact(4).enumerate() {
            assert_eq!(rgba[0], i as u8);
            assert_eq!(rgba[1], 0);
            assert_eq!(rgba[2], 0);
            assert_eq!(rgba[3], 255);
        }

        // Create a normalized f64 buffer
        ibuf.resize(0, 0.0);

        for i in 0..256 {
            ibuf.push(10_f64.powf((i as f64) / 256.0));
        }

        cmap_collection.normalize(
            "simple".to_string(),
            ibuf.clone(),
            obuf.as_mut_slice(),
            None,
            None,
            true,
        );
        for (i, rgba) in obuf.chunks_exact(4).enumerate() {
            assert_eq!(rgba[0], i as u8);
            assert_eq!(rgba[1], 0);
            assert_eq!(rgba[2], 0);
            assert_eq!(rgba[3], 255);
        }
    }
}

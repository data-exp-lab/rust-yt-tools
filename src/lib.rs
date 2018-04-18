#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

use stdweb::js_export;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Serialize)]
pub struct DataPixel {
  px: f64,
  py: f64,
  pdx: f64,
  pdy: f64,
  val: f64,
}

impl DataPixel {
  pub fn new(px: f64, py: f64,
             pdx: f64, pdy: f64,
             val: f64) -> DataPixel {
    DataPixel {
      px, py, pdx, pdy, val
    }
  }
}

#[derive(Serialize, Debug)]
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

impl FixedResolutionBuffer {
  pub fn new(width: usize, height: usize,
             x_low: f64,
             x_high: f64,
             y_low: f64,
             y_high: f64) -> FixedResolutionBuffer {
    let ipdx = width as f64 / (x_high - x_low);
    let ipdy = height as f64 / (y_high - y_low);
    let mut buffer: Vec<f64> = Vec::with_capacity(width * height);
    for x in 0..width*height {
        buffer.push(0.0);
    }

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

  pub fn index(&self, xi: usize, yi: usize) -> usize {
    let index: usize = xi * self.height + yi;
    index
  }

  pub fn deposit(&self, pix: &DataPixel) {
    
  }

  pub fn deposit_all(& mut self) {
  }

}

#[js_export]
fn hello_world() {
    console!(log, "hello world");
}

#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

use std::f64;
use std::mem::size_of;
use stdweb::js_export;
use stdweb::web::{ImageData, ArrayBuffer, TypedArray};
use stdweb::unstable::{TryFrom, TryInto};
use std::convert::{From, Into};

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

js_serializable!( DataPixel );

impl DataPixel {
  pub fn new(px: f64, py: f64,
             pdx: f64, pdy: f64,
             val: f64) -> DataPixel {
    DataPixel {
      px, py, pdx, pdy, val
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

js_serializable!( FixedResolutionBuffer );

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

  pub fn deposit(&mut self, pix: &DataPixel) {
    // Compute our left edge pixel
    
    if pix.px + pix.pdx < self.x_low {
        return;
    } else if pix.py + pix.pdy < self.y_low {
        return;
    } else if pix.px - pix.pdx > self.x_high {
        return;
    } else if pix.py - pix.pdy > self.y_high {
        return;
    }
    let lc: usize = (((pix.px - pix.pdx - self.x_low) * self.ipdx - 1.0)
                    .floor() as usize);
    let lr: usize = (((pix.py - pix.pdy - self.y_low) * self.ipdy - 1.0)
                    .floor() as usize);
    let rc: usize = (((pix.px + pix.pdx - self.x_low) * self.ipdx + 1.0)
                    .floor() as usize);
    let rr: usize = (((pix.py + pix.pdy - self.y_low) * self.ipdy + 1.0)
                    .floor() as usize);

    for i in lc.max(0)..rc.min(self.width) {
        for j in lr.max(0)..rr.min(self.height) {
            let ind = self.index(i, j);
            self.buffer[ind] = pix.val;
        }
    }
  }

  pub fn deposit_all(&mut self, pixels: &Vec<DataPixel>) {
      for pix in pixels.iter() {
        self.deposit(pix);
      }
  }

}

#[js_export]
fn hello_world() {
    console!(log, "hello world");
    let frb = FixedResolutionBuffer::new(500, 500, 0.0, 1.0, 0.0, 1.0);
js! {
  var frb = @{frb};
  console.log(frb);
};
}

#[js_export]
fn put_image(buffer: &ArrayBuffer) -> TypedArray<u8> {
    let fbuffer: TypedArray<f64> = buffer.into();
    let dpv: Vec<f64> = fbuffer.into();
    let n_pix = dpv.len() / 5;
    let mut pix_count: usize = 0;
    let mut pix: Vec<DataPixel> = Vec::with_capacity(n_pix);
    let mut mi: f64 = f64::MAX;
    let mut ma: f64 = f64::MIN;
    while pix_count < n_pix {
        let val = dpv[5 * pix_count + 0];
        let pdx = dpv[5 * pix_count + 1];
        let pdy = dpv[5 * pix_count + 2];
        let px = dpv[5 * pix_count + 3];
        let py = dpv[5 * pix_count + 4];
        pix.push(DataPixel::new(px, py, pdx, pdy, val));
        pix_count += 1;
        mi = mi.min(py);
        ma = ma.max(py);
    }
    js!{console.log("bounds", @{mi}, @{ma}, @{n_pix as u32});};
    let mut frb = FixedResolutionBuffer::new(500, 500, 0.0, 1.0, 0.0, 1.0);
    frb.deposit_all(&pix);
    let mut image: Vec<u8> = Vec::with_capacity(4*frb.width*frb.height);
    let cap = image.capacity();
    for i in 0..(frb.width*frb.height) {
        if frb.buffer[i] > 0.0 {
            mi = mi.min(frb.buffer[i]);
        }
        ma = ma.max(frb.buffer[i]);
    }
    mi = mi.log10();
    ma = ma.log10();
    for i in 0..(frb.width*frb.height) {
        let mut scaled: u8 = (255.0 * (frb.buffer[i].log10() - mi)/(ma - mi)) as u8;
        if(frb.buffer[i] == 0.0) { scaled = 0; }
        image.push(scaled);
        if(frb.buffer[i] == 0.0) { scaled = 255; }
        image.push(scaled);
        image.push(scaled);
        image.push(255);
        
    }
    let rv : TypedArray<u8> = TypedArray::<u8>::from(&image[..]);
    rv
}

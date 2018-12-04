extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use variable_mesh::VariableMesh;

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

        for pixel in vmesh.iter() {
            // Compute our left edge pixel
            if pixel.px + pixel.pdx < self.x_low ||
               pixel.py + pixel.pdy < self.y_low ||
               pixel.px - pixel.pdx > self.x_high ||
               pixel.py - pixel.pdy > self.y_high {
                continue;
            }
            let lc: usize = ((pixel.px - pixel.pdx - self.x_low) * self.ipdx - 1.0)
                .floor() as usize;
            let lr: usize = ((pixel.py - pixel.pdy - self.y_low) * self.ipdy - 1.0)
                .floor() as usize;
            let rc: usize = ((pixel.px + pixel.pdx - self.x_low) * self.ipdx + 1.0)
                .floor() as usize;
            let rr: usize = ((pixel.py + pixel.pdy - self.y_low) * self.ipdy + 1.0)
                .floor() as usize;

            for i in lc.max(0)..rc.min(self.width) {
                for j in lr.max(0)..rr.min(self.height) {
                    image_buffer[i][j] = pixel.val;
                    count += 1;
                }
            }
        }
        count
    }
}

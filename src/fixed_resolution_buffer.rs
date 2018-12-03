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

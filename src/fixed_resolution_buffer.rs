extern crate wasm_bindgen;
use variable_mesh::VariableMesh;
use wasm_bindgen::prelude::*;

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

    pub fn deposit(
        &mut self,
        vmesh: &VariableMesh,
        buffer: &mut [f64],
        name: String,
        position: Option<f64>,
    ) -> u32 {
        let mut count: u32 = 0;

        // We do need to clear the buffer -- in cases where the buffer is completely filled this
        // will result in extra work being done, but the alternate is to allocate a bunch of memory
        // and do filling of values anyway, so it may be the best we can do.
        buffer.fill(0.0);
        let mut image_buffer: Vec<&mut [f64]> =
            buffer.chunks_exact_mut(self.height).rev().collect();
        for pixel in vmesh.iter(&name) {
            // Compute our left edge pixel
            if pixel.px + pixel.pdx < self.x_low
                || pixel.py + pixel.pdy < self.y_low
                || pixel.px - pixel.pdx > self.x_high
                || pixel.py - pixel.pdy > self.y_high
            {
                continue;
            }
            if match position {
                Some(v) => pixel.pz - pixel.pdz > v || pixel.pz + pixel.pdz < v,
                None => false,
            } {
                continue;
            };
            let lc: usize = ((pixel.px - pixel.pdx - self.x_low) * self.ipdx - 1.0)
                .floor()
                .max(0.0) as usize;
            let lr: usize = ((pixel.py - pixel.pdy - self.y_low) * self.ipdy - 1.0)
                .floor()
                .max(0.0) as usize;
            let rc: usize = ((pixel.px + pixel.pdx - self.x_low) * self.ipdx + 1.0)
                .floor()
                .min(self.width as f64) as usize;
            let rr: usize = ((pixel.py + pixel.pdy - self.y_low) * self.ipdy + 1.0)
                .floor()
                .min(self.height as f64) as usize;

            for row in image_buffer.iter_mut().take(rr).skip(lr) {
                for image_pix in row.iter_mut().take(rc).skip(lc) {
                    *image_pix = pixel.val;
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_fixed_res_buffer() {
        let _frb_test = FixedResolutionBuffer::new(256, 256, 0.0, 1.0, 0.0, 1.0);
        assert_eq!(_frb_test.ipdx, 256.0);
        assert_eq!(_frb_test.ipdy, 256.0);
        let _frb_test = FixedResolutionBuffer::new(256, 259, 0.0, 2.0, 0.2, 2.5);
        assert_eq!(_frb_test.ipdx, 128.0);
        assert_eq!(_frb_test.ipdy, 259.0 / 2.3);
    }
}

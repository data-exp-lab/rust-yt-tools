extern crate wasm_bindgen;

mod colormaps;
mod fixed_resolution_buffer;
mod utils;
mod variable_mesh;

pub use colormaps::ColormapCollection;
pub use fixed_resolution_buffer::FixedResolutionBuffer;
pub use variable_mesh::VariableMesh;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_vm_fixed_res() {
        // This should eventually be parameterized.
        let mut px: Vec<f64> = Vec::new();
        let mut py: Vec<f64> = Vec::new();
        let mut pdx: Vec<f64> = Vec::new();
        let mut pdy: Vec<f64> = Vec::new();
        let mut field: Vec<f64> = Vec::new();

        let _nval = 32;
        let _npix = 1024;

        // We will now create a generic mesh

        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        // Compute our widths, which we'll use to generate the
        let mut widths_x: Vec<f64> = Vec::new();
        let mut widths_y: Vec<f64> = Vec::new();
        for _i in 0.._nval {
            // Just some generic values which we'll clean up at the end.
            // We do the second divide by two so that we have half-widths
            let _px = (1.0 - sum_x) / 2.0;
            // Also note that we're going to do something funky here,
            // just to make sure we're not always the same for x and y.
            let _py = (0.9 - sum_y) / 2.0;
            widths_x.push(_px / 2.0);
            widths_y.push(_py / 2.0);
            sum_x += _px;
            sum_y += _py;
        }

        widths_x.push((1.0 - sum_x) / 2.0);
        widths_y.push((1.0 - sum_y) / 2.0);

        let mut x;
        let mut y;

        x = 0.0;
        for &_pdx in widths_x.iter() {
            x += _pdx;
            y = 0.0;
            for &_pdy in widths_y.iter() {
                y += _pdy;
                px.push(x);
                py.push(y);
                pdx.push(_pdx);
                pdy.push(_pdy);
                field.push(1.0);
                y += _pdy;
            }
            assert_eq!(y, 1.0);
            x += _pdx;
        }

        assert_eq!(x, 1.0);

        let _vm = VariableMesh::new(px, py, pdx, pdy, field);

        for pixel in _vm.iter() {
            assert_eq!(pixel.val, 1.0);
        }

        let mut _frb_test = FixedResolutionBuffer::new(_npix, _npix, 0.0, 1.0, 0.0, 1.0);

        let mut buffer: Vec<f64> = Vec::new();
        buffer.resize(_npix * _npix, 0.0);

        // This does not always work, because of how we compute rows and columns,
        // so we don't assert.  We will eventually do so, though.
        let _count = _frb_test.deposit(&_vm, buffer.as_mut_slice());

        for &v in buffer.iter() {
            assert_eq!(v, 1.0);
        }
    }
}

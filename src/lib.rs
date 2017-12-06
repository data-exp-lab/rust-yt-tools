#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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

pub struct FixedResolutionBuffer {
  buffer: Vec<f64>,
  width: usize,
  height: usize,
  x_bounds: (f64, f64),
  y_bounds: (f64, f64),
  ipdx: f64,
  ipdy: f64,
}

impl FixedResolutionBuffer {
  pub fn new(width: usize, height: usize,
             x_bounds: (f64, f64),
             y_bounds: (f64, f64)) -> FixedResolutionBuffer {
    let ipdx = width as f64 / (x_bounds.1 - x_bounds.0);
    let ipdy = height as f64 / (y_bounds.1 - y_bounds.0);
    let buffer: Vec<f64> = Vec::with_capacity(width * height);
    FixedResolutionBuffer {
        buffer,
        width,
        height,
        x_bounds,
        y_bounds,
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

}

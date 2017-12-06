#![feature(link_args)]

#[cfg_attr(target_arch="wasm32", link_args = "--embed-file binary_data.dat@binary_data.dat")]
extern {}

extern crate byteorder;
extern crate yt_tools;

use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::mem::size_of;
use byteorder::{LittleEndian, ReadBytesExt};
use yt_tools::{DataPixel, FixedResolutionBuffer};

//use std::io::BufReader;

fn main() {
    let filename = "./binary_data.dat";

    let mut f = File::open(filename);
    let mut f = match f {
      Ok(file)   => file,
      Err(error) => {
                      panic!("There was a problem opening the file: {:?}", error)
                    },
    };
    let len = match f.metadata() {
        Ok(v) => v.len() as usize,
        Err(_) => 0,
    };
    let rs = size_of::<DataPixel>();
    let n_pix = len / rs;
    println!("Reading {} bytes from {} for {} pix\n", len, filename, n_pix);
    
    let mut pix: Vec<DataPixel> = Vec::with_capacity(n_pix);

    let mut pix_count = 0;

    while pix_count < n_pix {
        let val = f.read_f64::<LittleEndian>().unwrap(); 
        let pdx = f.read_f64::<LittleEndian>().unwrap();
        let pdy = f.read_f64::<LittleEndian>().unwrap();
        let px = f.read_f64::<LittleEndian>().unwrap();
        let py = f.read_f64::<LittleEndian>().unwrap();
        pix.push(DataPixel::new(px, py, pdx, pdy, val));
        pix_count += 1;
    }
    
    let frb = FixedResolutionBuffer::new(1024, 1024, (0.0, 1.0), (0.0, 1.0));
}

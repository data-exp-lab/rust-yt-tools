#![feature(link_args)]

#[cfg_attr(target_arch="wasm32", link_args = "--embed-file binary_data.dat@binary_data.dat")]
extern {}

extern crate byteorder;
extern crate yt_tools;
extern crate stdweb;

use std::env;
use std::fs::{File, read_dir};
use std::io::Read;
use std::rc::Rc;
use std::mem::size_of;
use byteorder::{LittleEndian, ReadBytesExt};
use yt_tools::{DataPixel, FixedResolutionBuffer};
use stdweb::unstable::TryInto;
use stdweb::web::{
    INode,
    HtmlElement,
    document,
};

// use stdweb::unstable::TryInto;
//use stdweb::web::{
//    HTMLElement,
//    document,
//};


//use std::io::BufReader;

fn main() {
    stdweb::initialize();

    let output_div: HtmlElement = document().query_selector( ".output" ).unwrap().try_into().unwrap();
    let output_msg = Rc::new(move |msg: &str| {
        let elem = document().create_element("p");
        elem.set_text_content(msg);
        if let Some(child) = output_div.first_child() {
            output_div.insert_before(&elem, &child);
        } else {
            output_div.append_child(&elem);
        }
    });

    output_msg("Printing Pixel Information...");
    output_msg("Hopefully this works...");

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

    let mut max_x: f64 = 0.0;
    let mut max_y: f64 = 0.0;

    while pix_count < n_pix {
        let val = f.read_f64::<LittleEndian>().unwrap(); 
        let pdx = f.read_f64::<LittleEndian>().unwrap();
        let pdy = f.read_f64::<LittleEndian>().unwrap();
        let px = f.read_f64::<LittleEndian>().unwrap();
        let py = f.read_f64::<LittleEndian>().unwrap();

        if px > max_x {
            max_x = px;
        }
        if py > max_y {
            max_y = py;
        }

        if pix_count < 10 {
            println!("The data in pixel {:?} is {:?}\n", pix_count, val);
            println!("The pdx and pdy data in this pixel is {:?} is {:?}\n", 
                     pdx, pdy);
            println!("The px and py data in this pixel is {:?} is {:?}\n", 
                     px, py);
        }
        if pix_count > n_pix-10 {
            println!("The data in pixel {:?} is {:?}\n", pix_count, val);
            println!("The pdx and pdy data in this pixel is {:?} is {:?}\n", 
                     pdx, pdy);
            println!("The px and py data in this pixel is {:?} is {:?}\n", 
                     px, py);
        }
        // let mut specialstring = String::new();
        let mut specialstring = format!("The px and py data for pixel {:?} is {:?} is {:?}\n", 
                       pix_count, px, py);
        if pix_count > n_pix-4 {
            output_msg(&specialstring);
        }
        pix.push(DataPixel::new(px, py, pdx, pdy, val));
        pix_count += 1;
    }

    println!("The max x, y values are: {:?} {:?} \n", max_x, max_y);
    // println!("The data at pix 10 is: {:?} \n", pix[10].val);
    
    let frb = FixedResolutionBuffer::new(1024, 1024, (0.0, 1.0), (0.0, 1.0));

    println!("Index 0, 0 becomes {}; Index 512, 512 becomes {}\n",
             frb.index(0, 0), frb.index(512, 512));

    //println!("Frb: {:?}\n", frb);
}

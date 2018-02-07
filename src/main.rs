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
    IEventTarget,
    INode,
    HtmlElement,
    document,
    WebSocket,
};

use stdweb::web::event::{
    IEvent,
    IKeyboardEvent,
    IMessageEvent,
    KeypressEvent,
    SocketOpenEvent,
    SocketCloseEvent,
    SocketErrorEvent,
    SocketMessageEvent,
};

use stdweb::web::html_element::InputElement;


macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

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

    while pix_count < n_pix {
        let val = f.read_f64::<LittleEndian>().unwrap(); 
        let pdx = f.read_f64::<LittleEndian>().unwrap();
        let pdy = f.read_f64::<LittleEndian>().unwrap();
        let px = f.read_f64::<LittleEndian>().unwrap();
        let py = f.read_f64::<LittleEndian>().unwrap();

        // This next section prints the values of the last few 
        // pixels added to DataPixel. 
        let mut specialstring = format!("The px and py data for pixel {:?} is {:?} is {:?}\n", 
                       pix_count, px, py);

        if pix_count > n_pix-4 {
            output_msg(&specialstring);
        }
        
        pix.push(DataPixel::new(px, py, pdx, pdy, val));
        pix_count += 1;
    }

    output_msg("DataPixel has been filled...");
    let ws = WebSocket::new("wss://echo.websocket.org").unwrap();

    ws.add_event_listener( enclose!( (output_msg) move |_: SocketOpenEvent| {
        output_msg(&format!("> Enter a pixel value between 0 and {:?}", pix_count));
    }));

    ws.add_event_listener( enclose!( (output_msg) move |_: SocketErrorEvent| {
        output_msg("> Connection Errored");
    }));

    ws.add_event_listener( enclose!( (output_msg) move |event: SocketCloseEvent| {
        output_msg(&format!("> Connection Closed: {}", event.reason()));
    }));

    ws.add_event_listener( enclose!( (output_msg) move |event: SocketMessageEvent| {
        // let pix_str = &event.data().into_text();
        // let pix_int: u8 = pix_str.parse::<u8>().unwrap();
        // output_msg(&format!("The value you chose was {:?}", pix_int));
        output_msg(&format!("The value you chose was {:?}", &event.data().into_text().unwrap()));
    }));

    let text_entry: InputElement = document().query_selector( ".form input" ).unwrap().try_into().unwrap();
    text_entry.add_event_listener( enclose!( (text_entry) move |event: KeypressEvent| {
        if event.key() == "Enter" {
            event.prevent_default();

            let text: String = text_entry.value().try_into().unwrap();
            if text.is_empty() == false {
                text_entry.set_value("");
                ws.send_text(&text);
            }
        }
    }));

    let frb = FixedResolutionBuffer::new(1024, 1024, (0.0, 1.0), (0.0, 1.0));

    println!("Index 0, 0 becomes {}; Index 512, 512 becomes {}\n",
             frb.index(0, 0), frb.index(512, 512));

    stdweb::event_loop();
    
}

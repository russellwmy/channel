use std::{cell::RefCell, rc::Rc};

use js_sys::Error;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{AudioContext, MediaStream};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn measure(stream: &MediaStream) -> Result<(), Error> {
    const fft_size: usize = 1024;
    let context = AudioContext::new()?;
    let microphone = context.create_media_stream_source(stream)?;
    let analyser = context.create_analyser()?;

    analyser.set_fft_size(fft_size as u32);
    microphone.connect_with_audio_node(&analyser)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let pcm_data: &mut [f32]= &mut [0.0; fft_size];

        analyser.get_float_time_domain_data(pcm_data);
        let square_sum: f32 = pcm_data
            .iter()
            .fold(0.0, |acc, x| acc + x * x);
        let result = (square_sum / pcm_data.len() as f32).sqrt();

        log::info!("{:?}", result);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

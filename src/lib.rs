use std::collections::HashMap;

use image::GenericImageView;

use image::Pixel;
use js_sys::Uint8Array;
use photon_rs::helpers;
use photon_rs::native::open_image_from_bytes;
use photon_rs::transform::resize;
use photon_rs::transform::SamplingFilter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use web_sys::console;
use web_sys::window;
use web_sys::HtmlInputElement;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // TODO: Set up panic hook
    // TODO: Set up button onclick event
    // TODO: Generate all the starting HTML in here rather than using it from index.html

    // let window = window().expect("No global window exists");
    // let document = window.document().expect("No window.document exists");

    // let text = document
    //     .get_element_by_id("text")
    //     .expect("Input text not found!")
    //     .dyn_into::<HtmlInputElement>()
    //     .unwrap()
    //     .value();
    // console::log_1(&text.into());

    Ok(())
}

#[wasm_bindgen]
pub async fn submit() -> Result<(), JsValue> {
    let window = window().expect("No global window exists");
    let document = window.document().expect("No window.document exists");
    let head = document.head().expect("No head tag exists");
    let output = document
        .get_element_by_id("output")
        .expect("Expect output to exist");

    // Clear output if it has anything in it
    while let Some(child) = output.first_child() {
        output.remove_child(&child)?;
    }

    // Read text from input box
    let text = document
        .get_element_by_id("text")
        .expect("Input text not found!")
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    // Conver into char iter and cycle
    let mut chars = text.chars().cycle();

    // Read selected image into bytes
    let image = document
        .get_element_by_id("image")
        .expect("upload image not found")
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .files()
        .expect("No selected file")
        .get(0)
        .unwrap();
    let bytes = Uint8Array::new(&JsFuture::from(image.array_buffer()).await?);
    let bytes = bytes.to_vec();

    // Read image and resize
    let img = open_image_from_bytes(&bytes).expect("Image should be valid");
    let width = 128;
    let ratio = img.get_width() / width;
    let height = img.get_height() / ratio;
    let img = resize(&img, width, height, SamplingFilter::Nearest);
    let img = helpers::dyn_image_from_raw(&img);

    // Map RGB tuple to css class name
    let mut css_color_map = HashMap::new();

    for y in 0..height {
        let div = document.create_element("div")?;
        for x in 0..width {
            let span = document.create_element("span")?;
            let pixel = img.get_pixel(x, y);
            let (r, g, b, _) = pixel.channels4();
            let rgb = (r, g, b);
            if !css_color_map.contains_key(&rgb) {
                let name = format!("color-{}", css_color_map.len());
                css_color_map.insert(rgb.clone(), name);
            }
            let class_name = css_color_map.get(&rgb).unwrap();
            span.set_class_name(class_name);
            span.set_inner_html(&(chars.next().unwrap().to_string()));
            div.append_child(&span)?;
        }
        output.append_child(&div)?;
    }

    let style = document.create_element("style")?;
    let mut css = Vec::new();
    for (k, v) in css_color_map.iter() {
        css.push(format!(".{}::selection {{background: rgb{:?};}}", v, k));
    }
    style.set_inner_html(&css.join("\n"));

    head.append_child(&style)?;
    Ok(())
}

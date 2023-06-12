use js_sys::Uint8Array;
use photon_rs::native::open_image_from_bytes;
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
    let _body = document.body().expect("No winodw.document.body exists");
    let output = document
        .get_element_by_id("output")
        .expect("Expect output to exist");

    // Read text from input box
    let text = document
        .get_element_by_id("text")
        .expect("Input text not found!")
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();

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

    let p1 = document.create_element("p")?;
    p1.set_inner_html(&text);
    while let Some(child) = output.first_child() {
        output.remove_child(&child)?;
    }
    output.append_child(&p1)?;

    let img = open_image_from_bytes(&bytes).expect("Image should be valid");
    let p2 = document.create_element("p")?;
    p2.set_inner_html(&format!("{}", img.get_height()));
    output.append_child(&p2)?;

    Ok(())
}

use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

pub fn initialize_webgl_context() -> Result<WebGlRenderingContext, JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("rustCanvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;

    //attach_mouse_down_handler(&canvas)?;
    //attach_mouse_up_handler(&canvas)?;
    //attach_mouse_move_handler(&canvas)?;

    // Allow things to be transparent (for fading in and out)
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);


    // What color should it set when it needs to erase something
    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA

    // Clear everything
    gl.clear_depth(1.);

    Ok(gl)
}
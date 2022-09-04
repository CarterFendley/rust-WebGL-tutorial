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

    attach_mouse_down_handler(&canvas)?;
    attach_mouse_up_handler(&canvas)?;
    attach_mouse_move_handler(&canvas)?;

    // Allow things to be transparent (for fading in and out)
    gl.enable(GL::BLEND);
    gl.blend_func(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);


    // What color should it set when it needs to erase something
    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA

    // Clear everything
    gl.clear_depth(1.);

    Ok(gl)
}

fn attach_mouse_down_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, true)
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", handler.as_ref().unchecked_ref())?;
    /*
     * Tell rust to forget about the memory management of this function. 
     * 
     * We are sending it to JS, and will pass out of scope. If we don't tell
     * it to forget then it will be cleaned up / freed.
     * 
     * Mini / one time memory leak on purpose;
     */
    handler.forget();

    return Ok(())
}

fn attach_mouse_up_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::app_state::update_mouse_down(event.client_x() as f32, event.client_y() as f32, false)
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", handler.as_ref().unchecked_ref())?;
    /*
     * Tell rust to forget about the memory management of this function. 
     * 
     * We are sending it to JS, and will pass out of scope. If we don't tell
     * it to forget then it will be cleaned up / freed.
     * 
     * Mini / one time memory leak on purpose;
     */
    handler.forget();

    return Ok(())
}

fn attach_mouse_move_handler(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let handler = move |event: web_sys::MouseEvent| {
        super::app_state::update_mouse_position(event.client_x() as f32, event.client_y() as f32);
    };

    let handler = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", handler.as_ref().unchecked_ref())?;
    handler.forget();

    Ok(())
}
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod gl_setup;
mod common_funcs;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
  log("Howdy!... from Rust!");
}

#[wasm_bindgen]
pub struct CartersClient {
  gl: WebGlRenderingContext,
  program_color_2d: programs::Color2D,
  _program_color_2d_grad: programs::Color2DGrad,
  program_graph_3d: programs::Graph3D,
}

#[wasm_bindgen]
impl CartersClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    console_error_panic_hook::set_once();
    let gl = gl_setup::initialize_webgl_context().unwrap();
    Self {
      program_color_2d: programs::Color2D::new(&gl),
      _program_color_2d_grad: programs::Color2DGrad::new(&gl),
      program_graph_3d: programs::Graph3D::new(&gl),
      gl: gl
    }
  }

  pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
    app_state::update_dyanmic_data(_time, _height, _width);
    Ok(())
  }

  pub fn render(&self) {
    // Clear with the defined color
    self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    let curr_state = app_state::get_curr_state();

    self.program_color_2d.render(
      &self.gl,
      curr_state.control_bottom,
      curr_state.control_top,
      curr_state.control_left,
      curr_state.control_right,
      curr_state.canvas_height,
      curr_state.canvas_width
    );

    /*
    self.program_color_2d_grad.render(
      &self.gl,
      curr_state.control_bottom + 20.,
      curr_state.control_top - 20.,
      curr_state.control_left + 20.,
      curr_state.control_right - 20.,
      curr_state.canvas_height,
      curr_state.canvas_width
    ); */

    self.program_graph_3d.render(
      &self.gl,
      curr_state.control_bottom,
      curr_state.control_top,
      curr_state.control_left,
      curr_state.control_right,
      curr_state.canvas_height,
      curr_state.canvas_width,
      curr_state.rotation_x_axis,
      curr_state.rotation_y_axis,
    );
  }
}
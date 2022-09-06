use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use crate::log;

use super::super::common_funcs as cf;

#[allow(dead_code)]
pub struct Color2DGrad {
  program: WebGlProgram,
  position_index: u32,
  color_index: u32,
  color_buffer: WebGlBuffer,
  index_count: i32,
  indices_buffer: WebGlBuffer,
  rect_vertice_buffer: WebGlBuffer,
  u_opacity: WebGlUniformLocation,
  u_transform: WebGlUniformLocation,
}

#[allow(dead_code)]
impl Color2DGrad {
  pub fn new(gl: &WebGlRenderingContext) -> Self {
    let program = cf::link_program(
      &gl,
      super::super::shaders::vertex::color_2d_grad::SHADER,
      super::super::shaders::fragment::vary_color_from_vertex::SHADER,
    ).unwrap();
    
    let vertices_rect: [f32; 8] = [
      0., 1., // x, y
      0., 0.,
      1., 1.,
      1., 0.,
    ];

    // Rectangle made up of two triangles
    // Counter clock wise
    // This is less data efficient with a square but with many vertices gets faster
    let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

    
    let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
      let vertices_location = vertices_rect.as_ptr() as u32 / 4;
      // JS version of the array
      let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
        vertices_location,
        vertices_location + vertices_rect.len() as u32
      );
      let buffer_rect = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
      gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
      /*
       * Put the array close to GPU memory.
       *
       * For different efficiences
       * STATIC_DRAW = Buffer will not be updated often.
       * DYNAMIC_DRAW = Buffer will be updated
       * STREAM_DRAW = Use buffer once and only once
       */
      gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

      let indices_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
      let indices_location = indices_rect.as_ptr() as u32 / 2; // Why 2 here and 4 above?
      let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
        indices_location,
        indices_location + indices_rect.len() as u32
      );
      let buffer_indices = gl.create_buffer().unwrap();
      gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
      gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);

    Self {
      color_buffer: gl.create_buffer().ok_or("Failure to create buffer").unwrap(),
      position_index: gl.get_attrib_location(&program, "aPosition") as u32,
      color_index: gl.get_attrib_location(&program, "aColor") as u32,
      indices_buffer: buffer_indices,
      index_count: indices_rect.len() as i32,
      u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
      u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
      rect_vertice_buffer: buffer_rect,
      program: program,
    }
  }

  pub fn render(
    &self,
    gl: &WebGlRenderingContext,
    bottom: f32,
    top: f32,
    left: f32,
    right: f32,
    canvas_height: f32,
    canvas_width: f32,
  ) {
    gl.use_program(Some(&self.program));

    //log(&format!("Color: {}, Position: {}", self.color_index, self.position_index));

    // Bind is like... which buffer you want the next calls to refer to?
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));

    /*
      Assigning the data to the first (0th) attribute in the shader "aPosition"

      Size: How many data elements there are per attribute we have: x, y (2)
        - Can get as many as 4 (vec4), with 2 the second two will be left empty

      Type: What data type to expect
      Normalized: ??

      Stride & Data used for when you have a buffer that contains more information than just the one you want to pull out
     */
    gl.vertex_attrib_pointer_with_i32(self.position_index, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(self.position_index);

    /*
      Setup colors

      They are in the render function so that they can update on each call.
      Normally you would want to make them parameters of the render function.
    */
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));    
    gl.vertex_attrib_pointer_with_i32(self.color_index, 4, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(self.color_index);

    let colors: [f32; 16] = [
      // r, g, b, a (opacity) for each vertex 
      1., 0., 0., 1.,
      0., 1., 0., 1.,
      0., 0., 1., 1.,
      1., 1., 1., 1.,
    ];

    let colors_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let color_vals_location = colors.as_ptr() as u32 / 4;
    let color_vals_array = js_sys::Float32Array::new(&colors_memory_buffer)
        .subarray(
          color_vals_location,
          color_vals_location + colors.len() as u32
        );
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &color_vals_array, GL::DYNAMIC_DRAW);


    // Global opacity
    gl.uniform1f(Some(&self.u_opacity), 1.);

    // The bellow is too much for 2D but gets hard in 3d
    /*
      Range in the x, y axis is from -1 to 1 (2)

      Conversion from canvas to webGL coordinates:
      `2. * left / canvas_width - 1`
        - `left / canvas_width`: Percentage of how far over we are
        - 2. * .... : Where this falls on the 0..2 range
        - .... - 1 : Subtract 1 because we start at -1 not 0 
    */
    let t_matrix = cf::translation_matrix(
      2. * left / canvas_width - 1.,
      2. * bottom / canvas_height - 1.,
      0.
    );

    let scale_matrix = cf::scale_matrix(
      2. * (right - left) / canvas_width,
      2. * (top - bottom) / canvas_height,
      0. 
    );

    let transformation_matrix = cf::mult_matrix_4(scale_matrix, t_matrix);
    gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transformation_matrix);

    //gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_length as i32 / 2) as i32);
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));
    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0)
  }
}
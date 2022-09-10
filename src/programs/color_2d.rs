use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;
use js_sys::WebAssembly;
use super::super::common_funcs as cf;

#[allow(dead_code)]
pub struct Color2D {
  program: WebGlProgram,
  rect_vertice_ary_length: usize,
  rect_vertice_buffer: WebGlBuffer,
  u_color: WebGlUniformLocation,
  u_opacity: WebGlUniformLocation,
  u_transform: WebGlUniformLocation,
}

#[allow(dead_code)]
impl Color2D {
  pub fn new(gl: &WebGlRenderingContext) -> Self {
    let program = cf::link_program(
      &gl,
      super::super::shaders::vertex::color_2d::SHADER,
      super::super::shaders::fragment::color_2d::SHADER,
    ).unwrap();

    // Rectangle made up of two triangles
    // Counter clock wise
    let vertices_rect: [f32; 12] = [
      // Triangle one
      0., 1., // x, y
      0., 0.,
      1., 1.,
      // Triangle two
      1., 1.,
      0., 0.,
      1., 0.,
    ];

    
    let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
      let vertices_location = vertices_rect.as_ptr() as u32 / 4;
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

    Self {
      u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
      u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
      u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
      rect_vertice_ary_length: vertices_rect.len(),
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

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.rect_vertice_buffer));

    /*
      Assigning the data to the first (0th) attribute in the shader "aPosition"

      Size: How many data elements there are per attribute we have: x, y (2)
        - Can get as many as 4 (vec4), with 2 the second two will be left empty

      Type: What data type to expect
      Normalized: ??
      
      Stride & Data used for when you have a buffer that contains more information than just the one you want to pull out
     */
    // 
    gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    // r, g, b, a (opacity)
    gl.uniform4f(Some(&self.u_color), 0., 0.5, 0.5, 1.0);
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

    gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_ary_length as i32 / 2) as i32);
  }
}
use crate::constants::GRID_SIZE;

use super::super::common_funcs as cf;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Graph3D {
  pub program: WebGlProgram,
  pub indices_buffer: WebGlBuffer,
  pub index_count: i32,
  pub position_index: u32,
  pub normals_index: u32,
  pub position_buffer: WebGlBuffer,
  pub normals_buffer: WebGlBuffer,
  pub u_normals_rotation: WebGlUniformLocation,
  pub y_index: u32,
  pub y_buffer: WebGlBuffer,
  pub u_opacity: WebGlUniformLocation,
  pub u_projection: WebGlUniformLocation
}

impl Graph3D {
  pub fn new(gl: &WebGlRenderingContext) -> Self {
    let program = cf::link_program(
      &gl,
      &super::super::shaders::vertex::graph_3d::SHADER,
      &super::super::shaders::fragment::vary_color_from_vertex::SHADER,
    ).unwrap();

    let positions_and_indices = cf::get_position_grid_n_by_n(GRID_SIZE);

    // Bind data so webGL can use it
    let memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let vertices_location = positions_and_indices.0.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer).subarray(
      vertices_location,
      vertices_location + positions_and_indices.0.len() as u32
    );
    let buffer_position = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_position));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vert_array, GL::STATIC_DRAW);

    let indices_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let indices_location = positions_and_indices.1.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
      indices_location,
      indices_location + positions_and_indices.1.len() as u32
    );
    let buffer_indices = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
    gl.buffer_data_with_array_buffer_view(GL::ELEMENT_ARRAY_BUFFER, &indices_array, GL::STATIC_DRAW);

    Self {
      u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
      u_projection: gl.get_uniform_location(&program, "uProjection").unwrap(),
      u_normals_rotation: gl.get_uniform_location(&program, "uNormalsRotation").unwrap(),
      y_index: gl.get_attrib_location(&program, "aY") as u32,
      position_index: gl.get_attrib_location(&program, "aPosition") as u32,
      normals_index: gl.get_attrib_location(&program, "aVertexNormal") as u32,
      program: program,
      indices_buffer: buffer_indices,
      index_count: indices_array.length() as i32,
      position_buffer: buffer_position,
      normals_buffer: gl.create_buffer().ok_or("Failed to create buffer").unwrap(),
      y_buffer: gl.create_buffer().ok_or("Failed to create buffer").unwrap(),
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
    rotation_angle_x_axis: f32,
    rotation_angle_y_axis: f32,
    y_vals: &Vec<f32>
  ) {
    gl.use_program(Some(&self.program));

  
    // The name projection is odd to me as we have multiple matrices being returned not just the projection one.
    let my_3d_matrices = cf::get_3d_matrices(
      bottom,
      top,
      left,
      right,
      canvas_height,
      canvas_width,
      rotation_angle_x_axis,
      rotation_angle_y_axis,
    );

    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.u_projection),
      false,
      &my_3d_matrices.projection,
    );
    gl.uniform_matrix4fv_with_f32_array(
      Some(&self.u_normals_rotation),
      false,
      &my_3d_matrices.normals_rotation);
    gl.uniform1f(Some(&self.u_opacity), 1.);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.position_buffer));
    gl.vertex_attrib_pointer_with_i32(self.position_index, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(self.position_index);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.y_buffer));
    gl.vertex_attrib_pointer_with_i32(self.y_index, 1, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(self.y_index);

    // Buffer the y data
    let y_memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();
    let y_location = y_vals.as_ptr() as u32 / 4;
    let y_array = js_sys::Float32Array::new(&y_memory_buffer).subarray(
        y_location,
        y_location + y_vals.len() as u32,
    );
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &y_array, GL::DYNAMIC_DRAW);

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buffer));
    gl.vertex_attrib_pointer_with_i32(self.normals_index, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(self.normals_index);

    let normals_vals = cf::get_grid_normals(super::super::constants::GRID_SIZE, &y_vals);
    let normals_vals_memory_buffer = wasm_bindgen::memory()
      .dyn_into::<WebAssembly::Memory>()
      .unwrap()
      .buffer();
    let normals_vals_location = normals_vals.as_ptr() as u32 / 4;
    let normals_vals_array = js_sys::Float32Array::new(&normals_vals_memory_buffer).subarray(
      normals_vals_location,
      normals_vals_location + normals_vals.len() as u32
    );
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.normals_buffer));
    gl.buffer_data_with_array_buffer_view(
      GL::ARRAY_BUFFER,
      &normals_vals_array,
      GL::DYNAMIC_DRAW,
    );

    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));
    gl.draw_elements_with_i32(GL::TRIANGLES, self.index_count, GL::UNSIGNED_SHORT, 0)
  }
}
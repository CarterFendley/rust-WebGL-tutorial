use std::sync::Arc;
use std::sync::Mutex;

use crate::log;

/*
  https://youtu.be/kjYCSySObDo?t=2960
  ^ He talks about Arc<Mutex<>> vs Mutex<Arc<>>
*/
lazy_static! {
  static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

// He notes there is a place for optimization here
pub fn update_dyanmic_data(time: f32, canvas_height: f32, canvas_width: f32) {
  let min_height_width = canvas_height.min(canvas_width);
  let display_size = 0.9 * min_height_width;
  let half_display_size = display_size / 2.;
  let half_canvas_height = canvas_height / 2.;
  let half_canvas_width = canvas_width / 2.;


  let mut data = APP_STATE.lock().unwrap();

  *data = Arc::new(AppState {
    canvas_height: canvas_height,
    canvas_width: canvas_width,

    // Doing these calcs here is better than in render, only happen once
    control_bottom: half_canvas_height - half_display_size,
    control_top: half_canvas_height + half_display_size,
    control_left: half_canvas_width - half_display_size,
    control_right: half_display_size + half_canvas_width,

    time: time,
    ..*data.clone()
  });
}

pub fn get_curr_state() -> Arc<AppState> {
  APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
  pub canvas_height: f32,
  pub canvas_width: f32,
  pub control_bottom: f32,
  pub control_top: f32,
  pub control_left: f32,
  pub control_right: f32,
  pub mouse_down: bool,
  pub mouse_x: f32,
  pub mouse_y: f32,
  pub rotation_x_axis: f32,
  pub rotation_y_axis: f32,
  pub time: f32,
}

impl AppState {
  fn new() -> Self {
    Self {
      canvas_height: 0.,
      canvas_width: 0.,
      control_bottom: 0.,
      control_top: 0.,
      control_left: 0.,
      control_right: 0.,
      mouse_down: false,
      mouse_x: -1.,
      mouse_y: -1.,
      // Turned in such a way that we can see it at the start
      rotation_x_axis: -0.5,
      rotation_y_axis: 0.5,
      time: 0.,
    }
  }
}

pub fn update_mouse_down(x: f32, y: f32, is_down: bool) {
  let mut data = APP_STATE.lock().unwrap();

  *data = Arc::new(AppState {
    mouse_down: is_down,
    mouse_x: x,
    // Y value is inverted so Y increases as you go up the screen
    mouse_y: data.canvas_height - y,
    ..*data.clone()
  })
}

pub fn update_mouse_position(x: f32, y: f32) {
  let mut data = APP_STATE.lock().unwrap();

  let inverted_y = data.canvas_height - y;

  let x_delta = x - data.mouse_x;
  let y_delta = inverted_y - data.mouse_y;

  /*
   * The rotation for x depends on y.
   * 
   * It rotates around the x axis if you pull it up or down.
   */
  let rotation_x_delta = if data.mouse_down {
    std::f32::consts::PI * y_delta / data.canvas_height
  } else {
    0.
  };

  let rotation_y_delta = if data.mouse_down {
    std::f32::consts::PI * x_delta / data.canvas_width
  } else {
    0.
  };

  log(&format!("X Delta: {x_delta}, Y Delta: {y_delta}, X RDelta: {rotation_x_delta}, Y RDelta: {rotation_y_delta}"));

  *data = Arc::new(AppState {
    mouse_x: x,
    // Y value is inverted so Y increases as you go up the screen
    mouse_y: inverted_y,
    rotation_x_axis: data.rotation_x_axis + rotation_x_delta,
    rotation_y_axis: data.rotation_y_axis - rotation_y_delta,
    ..*data.clone()
  })
}
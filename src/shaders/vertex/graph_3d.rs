/**
 * Projection matrix does the heavy linear alg math
 * 
 * Doing the color calcs here is better b/c fragment (pixel) shaders are run more than vertex ones
 */
pub const SHADER: &str = r#"
  attribute vec4 aPosition;

  uniform mat4 uProjection;
  varying lowp vec4 vColor;

  void main() {
    // 1.0 Helps the linear alg math work out
    gl_Position = uProjection * vec4(aPosition.x, 0.0, aPosition.z, 1.0);

    vColor = vec4(0.5, 0.5, 0.8, 1.0);
  }
"#;
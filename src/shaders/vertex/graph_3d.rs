/**
 * Projection matrix does the heavy linear alg math
 * 
 * Doing the color calcs here is better b/c fragment (pixel) shaders are run more than vertex ones
 * 
 * For "aY" using separate float bc it will be dynamically updated. aPositon is buffered once and only once 
 * - Could rebuffer each time, but this would be inefficient
 */
pub const SHADER: &str = r#"
  attribute vec4 aPosition;
  attribute float aY;

  uniform mat4 uProjection;
  varying lowp vec4 vColor;

  void main() {
    // 1.0 Helps the linear alg math work out
    gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);

    if (aY > 0.0) {
      vColor = vec4(0.0, aY, 0.0, 1.0);
    } else {
      vColor = vec4(-aY, 0.0, 0.0, 1.0);
    }
  }
"#;
/*
  Attributes can only be used in the vertex shaders.
  They can be transferred to the fragment shader with `varying`

  - lowp -> low precision: color values don't have to be accurate
*/
pub const SHADER: &str = r#"
  attribute vec4 aPosition;
  attribute vec4 aColor;
  uniform mat4 uTransform;

  varying lowp vec4 vColor;

  void main () {
    vColor = aColor;
    gl_Position = uTransform * aPosition;
  }
"#;
// Mediump = medium precision
// gl_FragColor built in fragment color set to the final value
pub const SHADER: &str = r#"
  precision mediump float;

  uniform vec4 uColor;
  uniform float uOpacity;

  void main() {
    gl_FragColor = vec4(uColor.r, uColor.g, uColor.b, uColor.a * uOpacity);
  }
"#;
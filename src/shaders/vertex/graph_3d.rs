/**
 * Projection matrix does the heavy linear alg math
 * 
 * Doing the color calcs here is better b/c fragment (pixel) shaders are run more than vertex ones
 * 
 * For "aY" using separate float bc it will be dynamically updated. aPositon is buffered once and only once 
 * - Could rebuffer each time, but this would be inefficient
 *
 *
 *
 * For lighting code see this tutorial:
 * https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Lighting_in_WebGL
 */
pub const SHADER: &str = r#"
  attribute vec4 aPosition;
  attribute float aY;
  attribute vec3 aVertexNormal;

  uniform mat4 uNormalsRotation;
  uniform mat4 uProjection;
  varying lowp vec4 vColor;

  void main() {
    // 1.0 Helps the linear alg math work out
    gl_Position = uProjection * vec4(aPosition.x, aY, aPosition.z, 1.0);

    vec3 ambientLight = vec3(0.2, 0.2, 0.2);
    vec3 directionalLightColor = vec3(1, 1, 1);
    // Position of the light above the left shoulder
    vec3 directionalVector = normalize(vec3(-0.85, 0.8, 0.75));

    // Find if the vertex is facing towards the light or not
    vec4 transformedNormal = uNormalsRotation * vec4(aVertexNormal, 1.0);
    float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
    vec3 vLighting = ambientLight + (directionalLightColor * directional);
    vec3 baseColor = vec3(0.5, 0.5, 0.8);

    vColor = vec4(baseColor * vLighting, 1.0);
  }
"#;
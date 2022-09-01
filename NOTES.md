## WebGL

https://webglfundamentals.org/
https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API

- Three axises x, y, z. 
- All axes go from -1, 1 not matter size / scale
- Z axis is for showing which is on top of another
  - This can be used for efficiency not to render background objects
  - But with fading / transparency (which we have) it can not.
    - Must feed in rendering instructs from furthest to closest
    - What is the point in this case?
      - Sill needed for 3 calculations
- Process 
  1. Input data (xyz positon)
  2. Buffer Data for GPU
  3. Vertex Shader
    - Modify data based on 3d object / camera position
  4. Fragment Shader
    - Color the triangle's pixels
  5. Pixels put on the screen
    - As triangles

### Face culling

Counter clock wise definition of points
  - When transparency not enabled, Can figure out if something is pointed away from you
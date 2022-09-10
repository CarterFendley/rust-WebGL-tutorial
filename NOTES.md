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

## Steps in 3D video
-Create a Grid on the XZ plane
  - Can think of like a floor not a wall
- Rotate the Graph
  - Mouse events
- Add Y values
- Color Graph based on Height
- Color graph based on simulated light


## Interesting indexing error

The `.as_ptr() as u32 / 4` and `.as_ptr() as u32 / 2` things are very confusing. Initially I had nothing rendering on the screen and only when I changed the `.as_ptr() as u32 / 2` line to look like the following did I get something to show up.

```rust
let indices_location = positions_and_indices.1.as_ptr() as u32 / 4;
```

Looking at [others asking about this code](https://github.com/dmilford/rust-3d-demo/issues/2) I eventually notices the following mistake.

```rust
// I had this
let indices_array = js_sys::Float32Array::new(&indices_memory_buffer).subarray(
    indices_location,
    indices_location + positions_and_indices.1.len() as u32
  );

// Instead of this
let indices_array = js_sys::Uint16Array::new(&indices_memory_buffer).subarray(
    indices_location,
    indices_location + positions_and_indices.1.len() as u32
  );
```

When I changed to the second way the `.as_ptr() as u32 / 2` worked. This is kind of odd to me that the operation which I believe to be [resizing the buffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/TypedArray/subarray). Should care at all about the location of the rust pointer. 

TODO: Maybe there is a place for using some other way to match the size of the arrays?


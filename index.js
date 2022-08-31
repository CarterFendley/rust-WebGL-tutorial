const rust = import('./pkg/rust_web_3d_tut');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then(m => {
  if (!gl) {
    alert('Failed to init WebGL');
    return;
  }

  // Allow things to be transparent (for fading in and out)
  gl.enable(gl.BLEND);
  // Blending technique
  gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

  const FPS_THROTTLE = 1000.0 / 30.0; // Milliseconds / frames
  const carterClient = new m.CartersClient();
  const initialTime = Date.now();
  var lastDrawTime = -1; // In ms

  function render() {
    window.requestAnimationFrame(render);
    const currTime = Date.now();

    if (currTime > lastDrawTime + FPS_THROTTLE){
      lastDrawTime = currTime;

      // Check for window resize to update the canvas size
      if (window.innerHeight != canvas.height || window.innerWidth != canvas.width) {
        canvas.height = window.innerHeight;
        canvas.clientHeight = window.innerHeight;
        canvas.style.height = window.innerHeight;

        canvas.width = window.innerWidth;
        canvas.clientWidth = window.innerWidth;
        canvas.style.width = window.innerWidth;

        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
      }

      let elapsedTime = currTime - initialTime;
      carterClient.update(elapsedTime, window.innerHeight, window.innerWidth);
      carterClient.render()
    }
  }

  // Initialize the loop
  render();
});
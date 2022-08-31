const rust = import('./pkg/rust_web_3d_tut');

rust.then(m => m.say_hello_from_rust())
  .catch(console.error)
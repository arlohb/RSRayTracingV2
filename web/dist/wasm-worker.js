const main = async () => {
  console.log("Loading wasm in worker");
  // this does mean I'm doing this twice, but I'm not sure how to avoid it
  const wasm = await import("./pkg/rs_ray_tracing_v2.js");

  await wasm.default();

  console.log("Initialising thread pool in worker");
  console.log("navigator.hardwareConcurrency:", navigator.hardwareConcurrency);
  console.log(wasm);
  await wasm.initThreadPool(navigator.hardwareConcurrency);

  console.log("Thread pool initialised");

  // this may be helpful
  // https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
}

main();

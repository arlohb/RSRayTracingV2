import * as Comlink from "comlink";

let wasm;

const init = async () => {
  console.log("Loading wasm in worker");
  // this does mean I'm doing this twice, but I'm not sure how to avoid it
  wasm = await import("../dist/pkg/rs_ray_tracing_v2.js");

  await wasm.default();
}

const initThreadPool = async () => {
  console.log("Initialising thread pool in worker");
  console.log("navigator.hardwareConcurrency:", navigator.hardwareConcurrency);
  await wasm.initThreadPool(navigator.hardwareConcurrency);

  console.log("Thread pool initialised");

  // this may be helpful
  // https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html
}

const renderImage = async (options) => {
  return await wasm.render_image(options);
}

Comlink.expose({
  init,
  initThreadPool,
  renderImage,
});

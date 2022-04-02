import * as Comlink from "comlink";

let wasm;

const init = async (memory) => {
  console.log("   Loading wasm");
  wasm = await import("../dist/pkg/rs_ray_tracing_v2.js");

  await wasm.default(undefined, memory);
}

const initThreadPool = async () => {
  console.log("   Initialising thread pool");
  console.log("   navigator.hardwareConcurrency:", navigator.hardwareConcurrency);
  await wasm.initThreadPool(navigator.hardwareConcurrency);

  console.log("   Thread pool initialised");
}

const renderImage = async (options) => {
  return await wasm.render_image(options);
}

Comlink.expose({
  init,
  initThreadPool,
  renderImage,
});

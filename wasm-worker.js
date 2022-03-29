import * as Comlink from "comlink";

Comlink.expose({
  async main() {
    console.log("Loading wasm");
    const wasm = await import("./dist/pkg/rs_ray_tracing_v2.js");

    // effectively wasm.init();
    await wasm.default();

    console.log("Initialising thread pool");
    await wasm.initThreadPool(navigator.hardwareConcurrency);

    console.log("Starting app");
    wasm.start("the_canvas_id");

    console.log("App started");
    document.getElementById("center_text").remove();
  }
})
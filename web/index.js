import * as Comlink from "comlink";

let link;

const renderImage = async () => {
  window.rayTracerImage = await link.renderImage();
  setTimeout(renderImage, 10);
}

(async () => {
  console.log("Loading egui");
  const egui = await import("../dist/pkg/rs_ray_tracing_v2.js");

  // effectively wasm.init();
  await egui.default();

  console.log("Creating worker");
  const worker = new Worker(new URL("./wasm-worker.js", import.meta.url), {
    type: "module"
  });

  console.log("Exposing worker");
  link = Comlink.wrap(worker);

  console.log("Running worker");
  await link.init();
  await link.initThreadPool();

  // this will only await the first one
  await renderImage();

  console.log("Starting egui");
  egui.start("the_canvas_id");

  console.log("Egui started");
  document.getElementById("center_text").remove();
})();

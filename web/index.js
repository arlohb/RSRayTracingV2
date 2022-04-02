import * as Comlink from "comlink";

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
  const link = Comlink.wrap(worker);

  console.log("Running worker");
  await link.init();
  await link.initThreadPool();

  console.log("Starting egui");
  egui.start("the_canvas_id");

  console.log("Egui started");
  document.getElementById("center_text").remove();

  console.log(await link.threadTest());
})();

import * as Comlink from "comlink";

(async () => {
  console.log("Loading egui");
  const egui = await import("../dist/pkg/rs_ray_tracing_v2.js");

  // effectively wasm.init();
  await egui.default();

  // console.log("Initialising thread pool");
  // await egui.initThreadPool(navigator.hardwareConcurrency);

  console.log("Starting egui");
  egui.start("the_canvas_id");

  console.log("Egui started");
  document.getElementById("center_text").remove();



  // const worker = new Worker(new URL("./wasm-worker.js", import.meta.url), {
  //   type: "module"
  // });

  // const link = Comlink.wrap(worker);

  // await link.main();

  // console.log("Worker ran");
})();

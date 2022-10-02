importScripts("./bevy_worker.js");

const { start } = wasm_bindgen;

async function init_wasm() {
  await wasm_bindgen("./bevy_worker_bg.wasm");

  console.log("waiting for worker event");
  self.onmessage = (event) => {
    console.log("received worker event", event);
    start(event.data);
  };
  postMessage(null);
}

init_wasm();

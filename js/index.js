import("../pkg/index.js").then(wasm => {
  console.log("The output vector is: event.clientX, event.screenX, event.x, event.offsetX")
  window.addEventListener("pointerdown", e => {
    console.log("Log from JS: ", e.clientX, e.screenX, e.x, e.offsetX)
    wasm.get_event_coordinates(e)
  })
}).catch(console.error)

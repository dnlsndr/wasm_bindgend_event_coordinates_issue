import("../pkg/index.js").then(wasm => {
  window.addEventListener("pointerdown", e => {
    console.log(e.clientX, e.screenX, e.x, e.offsetX)
    wasm.get_event_coordinates(e)
  })
}).catch(console.error)

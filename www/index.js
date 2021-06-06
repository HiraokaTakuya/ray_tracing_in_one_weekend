'use strict'

console.log("start loading wasm");
const ray_tracing = import('../pkg')
    .catch(console.error);
Promise.all([ray_tracing]).then(async function ([{ draw_ray_tracing_set }]) {
    console.log("finished loading wasm");

    const renderBtn = document.getElementById('render');
    renderBtn.addEventListener('click', () => {
        let jsResult = null;
        let wasmResult = null;
        {
            console.log("wasm only");
            draw_ray_tracing_set();
        }
    });
});

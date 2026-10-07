import './style.css'
import init, * as wasm from './wasm/wasm.js';

console.table({
    crossOriginIsolated,
    sharedArrayBuffer: typeof SharedArrayBuffer,
    wasm: typeof WebAssembly,
});

await init();

const hc = navigator.hardwareConcurrency;
const threads = hc <= 4? 4 : hc - 1;

await wasm.initThreadPool(threads);

console.log(wasm.add(2, 2));
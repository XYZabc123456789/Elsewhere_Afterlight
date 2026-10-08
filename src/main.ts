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

await wasm.setup(threads);

let job = new wasm.Job(wasm.JobKind.MIRROR);

let input: wasm.JobValueHandle;
{
    const lock = new wasm.RegistryLock()
    input = lock.insert({String: "hello"});
    lock.release();
}

let handle = await job.executeJob(input);
let output;
{
    const lock = new wasm.RegistryLock()
    output = lock.get(handle!);
    lock.release();
}

console.log(wasm.add(2, 2), output.String);
# WASM usage (deterministic verified path)

`axiomhive` exposes a WASM-friendly entrypoint that reuses the verified pipeline:

```sh
cargo build --target wasm32-unknown-unknown --features wasm
```

From JavaScript (wasm-bindgen):

```js
import init, { wasm_generate } from "./pkg/axiomhive.js";

await init();
const axiomSet = { name: "demo", version: "1", rules: [] };
const result = await wasm_generate("hello", axiomSet, 256);
console.log(result); // returns [output, c0_signature]
```

The WASM path is deterministic for the same `(prompt, axiom_set, max_steps)` inputs.

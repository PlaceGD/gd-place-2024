import "./app.css";
import App from "./App.svelte";

import "./gd/object";
import "./firebase";

export const HAS_OPT_WASM = __HAS_OPT_WASM ?? false; // replaced by the `__HAS_OPT_WASM` define in vite config

const app = new App({
    target: document.getElementById("app") as any,
});

export default app;

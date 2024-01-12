import "./app.css";
import App from "./App.svelte";

import "./gd/object";
import "./firebase";

export const DEBUG = __DEBUG ?? false; // replaced by the `xxxxxx` define in vite config
export const HAS_OPT_WASM = __HAS_OPT_WASM ?? false;

const app = new App({
    target: document.getElementById("app") as any,
});

export default app;

import "./app.css";
import App from "./App.svelte";

export const HAS_OPT_WASM = __HAS_OPT_WASM ?? false; // replaced by the `__HAS_OPT_WASM` define in vite config

const app = new App({
    target: document.getElementById("app") as any,
});

import { helloWorld } from "./firebase/cloudFunctions";
helloWorld().then(result => {
    const data = result.data;
    console.log(data);
});

export default app;

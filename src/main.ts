import "./app.css";
import App from "./App.svelte";

import "./gd/object";
import "./firebase";

export const __DEBUG = "__DEBUG"; // replaced by the `__DEBUG` define in vite config

const app = new App({
    target: document.getElementById("app") as any,
});

export default app;

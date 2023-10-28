import "./app.css";
import App from "./App.svelte";

import "./gd/object";

const app = new App({
    target: document.getElementById("app") as any,
});

export default app;

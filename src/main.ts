import "./app.css";
import App from "./App.svelte";

import "./gd/object";
import "./firebase";
import { GDColor, GDObject } from "../wasm-lib/pkg/wasm_lib";

const app = new App({
    target: document.getElementById("app") as any,
});

export default app;

// let obj = new GDObject(
//     35,
//     0,
//     4.5,
//     69,
//     true,
//     false,
//     -2.0,
//     -420,
//     new GDColor(245, 67, 102, 11, false),
//     new GDColor(245, 67, 102, 11, false)
// );

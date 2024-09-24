// why cant this go in the d.ts file?
// who the fuck knows!!!!!!!!!!!!
interface KofiWidget {
    init: (a: string, b: string, c: string) => void;
    getHTML: () => string;
}

declare global {
    interface Window {
        kofiwidget2: KofiWidget;
        clearAllTheStuff: () => void;
        consoleErrors: string[];
    }
}

window.consoleErrors = [];

window.clearAllTheStuff = () => {
    localStorage.clear();
    indexedDB.deleteDatabase("PlaceDB");
    window.location.reload();
};

const oldError = window.console.error;
window.console.error = function (...args) {
    window.consoleErrors.push(args.join(" "));

    return oldError(...args);
};

import "./app.css";
import App from "./App.svelte";
// import "../public/Pusab.otf";

// export const HAS_OPT_WASM = __HAS_OPT_WASM ?? false; // replaced by the `__HAS_OPT_WASM` define in vite config

const app = new App({
    target: document.getElementById("app")!,
});

export default app;

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
    }
}

window.clearAllTheStuff = () => {
    localStorage.clear();
    indexedDB.deleteDatabase("PlaceDB");
    window.location.reload();
};

import "./app.css";
// import "../public/Pusab.otf";
import App from "./App.svelte";

// export const HAS_OPT_WASM = __HAS_OPT_WASM ?? false; // replaced by the `__HAS_OPT_WASM` define in vite config

const app = new App({
    target: document.getElementById("app")!,
});

export default app;

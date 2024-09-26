interface Window {
    kofiwidget2: KofiWidget;
    clearAllTheStuff: () => void;
    consoleErrors: string[];
}

declare module "*.svg" {
    import { SvelteComponent } from "svelte";
    export default SvelteComponent;
}

interface KofiWidget {
    init: (a: string, b: string, c: string) => void;
    getHTML: () => string;
}

declare const __DEBUG: boolean;
declare const __TURNSTILE_LOGIN_SITE_KEY: string;
// declare const __TURNSTILE_GENERAL_KEY: string;

/// <reference types="svelte" />
/// <reference types="vite/client" />
/// <reference types="wasm-lib" />
/// <reference types="svelte-gestures" />

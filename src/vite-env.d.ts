declare const __DEBUG: boolean;
declare const __HAS_OPT_WASM: boolean;

declare module "*.svg" {
    import { SvelteComponent } from "svelte";
    export default SvelteComponent;
}

/// <reference types="svelte" />
/// <reference types="vite/client" />
/// <reference types="wasm-lib" />

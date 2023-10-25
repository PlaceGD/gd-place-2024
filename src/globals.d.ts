/// <reference types="vite" />
/// <reference path="client.d.ts" />

declare module "*.svg" {
    import { SvelteComponent } from "svelte"
    export default SvelteComponent
}

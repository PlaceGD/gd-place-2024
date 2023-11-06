<script lang="ts">
    import Editor from "./Editor.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";

    import init from "../wasm-lib/pkg/wasm_lib.js";
    import Logo from "./components/Logo.svelte";
    import { __DEBUG } from "./main";
    import { toast } from "./utils/toast";

    let max = 0;
    let progress = 0;

    let hasLoaded = false;

    const wasm_r = new XMLHttpRequest();
    wasm_r.responseType = "arraybuffer";
    wasm_r.addEventListener("progress", p => {
        max = p.total;
        progress = p.loaded;
    });
    wasm_r.addEventListener("load", () => {
        console.log("wasm downloaded!");

        init(wasm_r.response)
            .catch(e => {
                toast.showErrorToast("Failed to initialize WASM.");
            })
            .then(() => {
                hasLoaded = true;
            });
    });
    wasm_r.open("GET", "../wasm-lib/pkg/wasm_lib_bg.wasm");

    wasm_r.send();
</script>

<SvelteToast options={{ duration: 6000, intro: { y: -64 } }} />

<div class="relative w-screen h-screen">
    <Editor bind:wasmLoaded={hasLoaded} />
    {#if !hasLoaded}
        <div class="absolute">
            <input type="range" min={0} {max} bind:value={progress} />
        </div>
    {/if}
</div>

<script lang="ts">
    import Editor from "./Editor.svelte";

    import init from "../wasm-lib/pkg/wasm_lib.js";
    import Logo from "./components/Logo.svelte";

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
                console.log("error initialising wasm:", e);
            })
            .then(() => {
                hasLoaded = true;
            });
    });
    wasm_r.open("GET", "../wasm-lib/pkg/wasm_lib_bg.wasm");

    wasm_r.send();
</script>

<div class="relative w-screen h-screen">
    {#if hasLoaded}
        <Editor />
    {:else}
        <div class="absolute">
            <!-- <Logo /> -->
            <input type="range" min={0} {max} bind:value={progress} />
        </div>
    {/if}
</div>

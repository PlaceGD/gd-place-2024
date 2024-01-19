<script lang="ts">
    import Editor from "./Editor.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";

    import initWasm from "wasm-lib";
    import Logo from "./components/Logo.svelte";
    import { HAS_OPT_WASM } from "./main";
    import Toast from "./utils/Toast";
    import { alertHasDarkReader } from "./utils/Document";

    alertHasDarkReader();

    if (localStorage.getItem("didErrorOccur") == "1") {
        localStorage.setItem("didErrorOccur", "0");
        Toast.showErrorToast(
            "An error occured and the page had to be refreshed."
        );
    }

    const WASM_URL = `../wasm-lib/pkg/wasm_lib_bg.wasm${
        HAS_OPT_WASM ? "-opt.wasm" : ""
    }`;

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
        initWasm(wasm_r.response)
            .catch(e => {
                Toast.showErrorToast(`Failed to initialize WASM. (${e})`);
            })
            .then(() => {
                hasLoaded = true;
            });
    });
    wasm_r.addEventListener("error", () => {
        Toast.showErrorToast("Failed to download WASM.");
    });
    wasm_r.addEventListener("abort", () => {
        Toast.showErrorToast("Failed to download WASM.");
    });

    wasm_r.open("GET", WASM_URL);
    wasm_r.send();
</script>

<SvelteToast options={{ duration: 6000, intro: { y: -64 } }} />

<div class="relative w-screen h-screen overflow-hidden">
    <Editor bind:wasmLoaded={hasLoaded} />
    {#if !hasLoaded}
        <div class="absolute">
            <input type="range" min={0} {max} bind:value={progress} />
        </div>
    {/if}
</div>

<style lang="postcss">
    :global(._toastContainer) {
        font-family: Saira;
        color: white;
        border-radius: 8px;
    }

    @media screen(lg) {
        :global(._toastContainer) {
            font-size: 14px;
        }
    }

    @media screen(sm) {
        :global(._toastContainer) {
            font-size: 12px;
        }
    }
</style>

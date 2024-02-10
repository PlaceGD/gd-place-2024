<script lang="ts">
    import Editor from "./Editor.svelte";
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import Toast from "./utils/toast";
    import { alertHasDarkReader } from "./utils/document";
    import Login from "./login/Login.svelte";
    import LoginButton from "./login/LoginButton.svelte";
    import DataPopup from "./DataPopup.svelte";
    import ModButton from "./moderator/ModButton.svelte";

    import {
        wasmProgress,
        initWasm,
        loadSpritesheet,
        spritesheetProgress,
    } from "./load_wasm";
    import { writable } from "svelte/store";
    import ReportedUserList from "./moderator/ReportedUserList.svelte";

    alertHasDarkReader();

    if (localStorage.getItem("didErrorOccur") == "1") {
        localStorage.setItem("didErrorOccur", "0");
        Toast.showErrorToast(
            "An error occured and the page had to be refreshed."
        );
    }

    // import { onMount } from "svelte";
    // import { SITE_KEY } from "./grecaptcha";
    // onMount(() => {
    //     grecaptcha.execute(SITE_KEY, { action: "page_load" }).then(t => {
    //         console.log(t);
    //     });
    // });

    initWasm();
    loadSpritesheet();

    $: loaded =
        $wasmProgress.hasLoaded && $spritesheetProgress.arrayBuffer != null;

    $: max = $wasmProgress.max + $spritesheetProgress.max;
    $: progress = $wasmProgress.progress + $spritesheetProgress.progress;
</script>

<SvelteToast options={{ duration: 6000, intro: { y: -64 } }} />

<DataPopup />

<div class="relative w-screen h-screen overflow-hidden">
    <Editor bind:wasmLoaded={loaded} />
    {#if !loaded}
        <div class="absolute">
            <input
                type="range"
                min={0}
                {max}
                value={progress}
                aria-label="Download progress of data required for GD Place"
                aria-valuetext={`${(progress / max) * 100}%`}
                tabindex="-1"
            />
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

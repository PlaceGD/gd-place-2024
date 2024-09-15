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
        fetchAndParseSpritesheet,
        spritesheetProgress,
    } from "./load_wasm";
    import { writable } from "svelte/store";
    import ReportedUserList from "./moderator/ReportedUserList.svelte";
    import ToastContainer from "./components/ToastContainer.svelte";
    import Turnstiles from "./Turnstiles.svelte";
    import { rawSpritesheetData } from "./stores";
    // import { getTurnstileToken, resetTurnstile } from "./utils/turnstile";

    alertHasDarkReader();

    // import { onMount } from "svelte";
    // import { SITE_KEY } from "./grecaptcha";
    // onMount(() => {
    //     grecaptcha.execute(SITE_KEY, { action: "page_load" }).then(t => {
    //         console.log(t);
    //     });
    // });

    initWasm();
    fetchAndParseSpritesheet().then(data => {
        $rawSpritesheetData = data;
    });

    $: loaded =
        $wasmProgress.hasLoaded &&
        $spritesheetProgress.arrayBuffer != null &&
        $rawSpritesheetData != null;

    $: max = $wasmProgress.max + $spritesheetProgress.max;
    $: progress = $wasmProgress.progress + $spritesheetProgress.progress;
</script>

<Turnstiles />

<ToastContainer />

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

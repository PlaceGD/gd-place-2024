<script lang="ts">
    import Image from "../components/Image.svelte";
    import {
        ExclusiveMenus,
        openMenu,
        viewingLevelAfterEvent,
    } from "../stores";
    import * as wasm from "wasm-lib";
    import viewLevelButton from "./assets/view_level.png?url";
    import { scale } from "svelte/transition";

    export let state: wasm.State;
</script>

<button
    class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
    data-guide="test"
    on:click={() => {
        $openMenu = ExclusiveMenus.Editor;
        $viewingLevelAfterEvent = true;
        state.transition_to_ending_spectate(3.0);
    }}
    transition:scale={{
        duration: 200,
        delay: 0,
        start: 0.5,
    }}
>
    <Image src={viewLevelButton} class="object-contain aspect-square"></Image>
</button>

<script lang="ts">
    import Image from "../components/Image.svelte";
    import { ExclusiveMenus, openMenu } from "../stores";
    import * as wasm from "wasm-lib";
    import viewLevelButton from "./assets/view_level.png?url";

    export let state: wasm.State;
</script>

<button
    class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12"
    data-guide="test"
    on:click={() => {
        state.transition_to_ending_spectate(3.0);
        $openMenu =
            $openMenu == ExclusiveMenus.Editor ? null : ExclusiveMenus.Editor;
    }}
>
    <Image src={viewLevelButton} class="object-contain aspect-square"></Image>
</button>

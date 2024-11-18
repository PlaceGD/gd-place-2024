<script lang="ts">
    import LevelView from "./level_view/LevelView.svelte";
    import * as wasm from "wasm-lib";
    import { eventStatus, openMenu } from "./stores";

    // const dick = (v: wasm.Gliberal) => {
    //     v.doink
    // }

    export let wasmLoaded: boolean;

    let editorFocused: boolean = false;

    $: {
        if (editorFocused) {
            $openMenu = null;
        }
    }

    let state: wasm.State | null = null;

    let canvas: HTMLCanvasElement;
</script>

<!-- <button
    class="absolute z-50 text-xl text-white bg-black"
    on:click={() => ($DEBUG_ENDING_VISIBILITY = !$DEBUG_ENDING_VISIBILITY)}
    >SHOW/HIDE ENDING</button
> -->

{#if wasmLoaded}
    {#if $eventStatus != "loading"}
        <LevelView bind:state bind:canvas />
    {/if}
{/if}

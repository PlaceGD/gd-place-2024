<script lang="ts">
    import AnimatedEnding from "./AnimatedEnding.svelte";
    import { DEBUG_ENDING_VISIBILITY } from "./ending";

    import * as wasm from "wasm-lib";
    import LiveNameInput from "./LiveNameInput.svelte";
    import StatsDisplay from "./StatsDisplay.svelte";
    import { eventStatus } from "../stores";
    import { onDestroy } from "svelte";

    export let state: wasm.State;
</script>

{#if $eventStatus == "name set"}
    <LiveNameInput />
{:else if $eventStatus == "fully done"}
    <AnimatedEnding />
    <StatsDisplay bind:state />
{/if}

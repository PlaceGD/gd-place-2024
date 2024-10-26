<script lang="ts">
    import * as wasm from "wasm-lib";
    import { beforeUpdate, onMount } from "svelte";
    import type { GuideAction } from "./guideActions";
    import { get } from "svelte/store";

    export let step: GuideAction;
    export let state: wasm.State;
    export let tooltipSize: { width: number; height: number };

    export let tooltipTop: number = 0;
    export let tooltipLeft: number = 0;

    export let canInteract: boolean;

    const setTooltipPos = () => {
        [tooltipLeft, tooltipTop] = step?.getTooltipPos?.({
            state,
            tooltipSize,
        }) ?? [0, 0];
    };

    onMount(async () => {
        setTooltipPos();
    });

    beforeUpdate(() => {
        setTooltipPos();
    });
</script>

<svelte:window
    on:resize={() => {
        setTooltipPos();
    }}
/>

{#if !canInteract}
    <!-- disable clicking of the element -->
    <div class="absolute w-screen h-screen z-[52] pointer-events-auto"></div>
{/if}

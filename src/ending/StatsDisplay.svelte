<script lang="ts">
    import { expoOut } from "svelte/easing";
    import { fade, fly } from "svelte/transition";
    import * as wasm from "wasm-lib";
    import { STATS } from "./ending";
    import { onDestroy } from "svelte";

    const stat_names = [
        "Number of creators",
        "Objects placed",
        "Objects deleted",
    ];

    export let state: wasm.State;
    let current_stat = -1;

    let interval: NodeJS.Timeout;
    let timeout: NodeJS.Timeout;

    STATS.then((stats: number[]) => {
        interval = setInterval(() => {
            timeout = setTimeout(() => {
                state.set_stats(stats[current_stat]);
            }, 2000);
            current_stat++;
            current_stat = current_stat % stats.length;
        }, 8000);
    });

    const fltoatInOut = (_: HTMLElement) => {
        return {
            duration: 5000,
            easing: expoOut,
            css: (t: number) => {
                return `
                    opacity: ${Math.max(t - 0.75, 0.0) * 4};
                    transform: translateY(-${(0.25 - Math.max(t - 0.75, 0.0)) * 60}px);
                `;
            },
        };
    };

    onDestroy(() => {
        state.hide_stats();
        clearInterval(interval);
        clearTimeout(timeout);
    });
</script>

{#if current_stat != -1}
    {#key stat_names[current_stat]}
        <div
            class="absolute z-30 flex justify-center w-full h-full text-3xl text-white bg-transparent pointer-events-none xs:text-xl flex-center"
            transition:fltoatInOut|global
        >
            {stat_names[current_stat]}:
        </div>
    {/key}
{/if}

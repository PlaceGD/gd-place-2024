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
            }, 1000);
            current_stat++;
            current_stat = current_stat % stats.length;
        }, 8000);
    });

    const fltoatInOutTest = (node: HTMLElement) => {
        node.animate(
            [
                {
                    easing: "cubic-bezier(0.16, 1, 0.3, 1)",
                    offset: 0,
                    transform: `translateY(-30px)`,
                },
                {
                    transform: `translateY(0px)`,
                    offset: 0.2,
                },
                {
                    transform: `translateY(0px)`,
                    easing: "cubic-bezier(0.7, 0, 0.84, 0)",
                    offset: 0.8,
                },
                {
                    transform: `translateY(-30px)`,
                    offset: 1,
                },
            ],
            {
                duration: 5000,
                fill: "forwards",
            }
        );

        node.animate(
            [
                {
                    offset: 0,
                    opacity: 0,
                },
                {
                    opacity: 1,
                    offset: 0.2,
                },
                {
                    opacity: 1,
                    offset: 0.8,
                },
                {
                    opacity: 0,
                    offset: 1,
                },
            ],
            {
                duration: 5000,
                fill: "forwards",
            }
        );
    };

    onDestroy(() => {
        state.hide_stats();
        clearInterval(interval);
        clearTimeout(timeout);
    });

    // const disappear = (_: HTMLElement) => {
    //     console.log("HERE");
    //     return {
    //         duration: 0,
    //         css: () => {
    //             return "";
    //         },
    //     };
    // };
</script>

<!-- transition:fltoatInOut|global -->
{#if current_stat != -1}
    {#key stat_names[current_stat]}
        <div
            class="absolute z-30 flex justify-center w-full h-full text-3xl text-white bg-transparent pointer-events-none xs:text-xl flex-center"
            use:fltoatInOutTest
        >
            {stat_names[current_stat]}:
        </div>
    {/key}
{/if}

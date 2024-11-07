<script lang="ts">
    import { expoOut } from "svelte/easing";
    import { fade, fly } from "svelte/transition";
    import * as wasm from "wasm-lib";
    import { onDestroy } from "svelte";
    import { db } from "../firebase/firebase";
    import { customFadeOut } from "./animations";
    import { disappear } from "../utils/transitions";

    let userCount = 0;
    let unsubUserCountUnsub = db
        .ref("userCount")
        .on("value", snapshot => (userCount = snapshot.val() ?? 0));

    let totalPlaced = 0;
    let totalPlacedUnsub = db
        .ref("totalObjectsPlaced")
        .on("value", snapshot => (totalPlaced = snapshot.val() ?? 0));

    let totalDeleted = 0;
    let totalDeletedUnsub = db
        .ref("totalObjectsDeleted")
        .on("value", snapshot => (totalDeleted = snapshot.val() ?? 0));

    $: STATS = {
        "Number of creators": userCount,
        "Objects placed": totalPlaced,
        "Objects deleted": totalDeleted,
    } as Record<string, number>;

    $: STAT_NAMES = Object.keys(STATS);

    export let state: wasm.State;
    let currentStat = -1;

    let interval: NodeJS.Timeout;
    let timeout: NodeJS.Timeout;

    interval = setInterval(() => {
        timeout = setTimeout(() => {
            state.set_stats(STATS[STAT_NAMES[currentStat]]);
        }, 1000);
        currentStat++;
        currentStat = currentStat % STAT_NAMES.length;
    }, 8000);

    onDestroy(() => {
        // if (statText != null) {
        //     statText.animate([{ opacity: 1 }, { opacity: 0 }], {
        //         duration: 500,
        //         fill: "forwards",
        //     });
        // }

        totalPlacedUnsub();
        totalDeletedUnsub();
        unsubUserCountUnsub();

        state.hide_stats();
        clearInterval(interval);
        clearTimeout(timeout);
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
</script>

<!-- transition:fltoatInOut|global -->
{#if currentStat != -1}
    {#key STAT_NAMES[currentStat]}
        <div
            class="absolute z-30 flex justify-center w-full h-full text-3xl text-white bg-transparent pointer-events-none xs:text-xl flex-center"
            use:fltoatInOutTest
        >
            <span out:fade|global={{ duration: 500 }}>
                {STAT_NAMES[currentStat]}:
            </span>
        </div>
    {/key}
{/if}

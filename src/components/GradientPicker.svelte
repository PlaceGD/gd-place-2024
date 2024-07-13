<script lang="ts">
    import RangeSlider from "svelte-range-slider-pips";

    function random(min: number, max: number) {
        return Math.floor(Math.random() * (max - min + 1)) + min;
    }

    export let maxStops: number = 10;

    export let gradientString = "";

    let rawValues: number[] = new Array(maxStops)
        .fill(0)
        .map((_, i) => (i / (maxStops - 1)) * 100);

    let currentStops = rawValues.map(v => ({
        position: v,
        r: random(0, 255),
        g: random(0, 255),
        b: random(0, 255),
    }));

    $: {
        currentStops = currentStops
            .map((c, i) => ({
                ...c,
                position: rawValues[i],
            }))
            .sort((a, b) => a.position - b.position);

        gradientString = `linear-gradient(90deg, ${currentStops.map(c => `rgba(${c.r},${c.g},${c.b},1) ${c.position}%`)})`;
    }
</script>

<div class="w-full">
    <div class="h-16" style={`--bg: ${gradientString}`}>
        <RangeSlider
            bind:values={rawValues}
            id="gradient-slider"
            springValues={{ stiffness: 1, damping: 1 }}
            step={1}
            min={0}
            max={100}
            float
            hoverable={false}
            suffix="%"
        ></RangeSlider>
    </div>
    <table class="mt-16 w-full">
        <tr class="flex">
            <th class="flex-auto">+</th>
            <th class="flex-auto">Stop</th>
            <th class="flex-auto">⊕</th>
        </tr>
        {#each currentStops as stop}
            <tr class="flex">
                <td class="flex-auto">TODO</td>
                <td class="flex-auto">{stop.position}%</td>
                <td class="flex-auto">X</td>
            </tr>
        {/each}
    </table>
</div>

<style lang="postcss">
    :global(#gradient-slider) {
        --range-handle: white;
        --range-handle-border: transparent;
        --range-handle-focus: white;
        --range-handle-inactive: white;
        @apply m-0 h-full rounded-md border-2 border-white/70;
        background: var(--bg);
    }

    :global(#gradient-slider .rangeHandle) {
        @apply bottom-0 top-0 h-full w-2 -translate-x-1/2 translate-y-0 rounded-md;
    }

    :global(#gradient-slider .rangeNub) {
        @apply rounded-md bg-transparent outline outline-4 outline-white ring-2 ring-white/70 ring-offset-4;
    }

    :global(#gradient-slider .rangeFloat) {
        @apply pointer-events-auto top-full mt-4 h-8 w-12 -translate-x-1/2 translate-y-0 rounded-md border border-disabled-white text-black opacity-100 transition-none;
    }
</style>

<script lang="ts">
    import { onDestroy } from "svelte";
    import { useIsOverflowing } from "../utils/document";

    export let reachedBottom = false;

    let elem: HTMLDivElement | null = null;

    const { isOverflowing, element: overflowElem } = useIsOverflowing();

    let threshold = 20;
    let scrollTop = 0;
    let elemBottom = threshold + 1;

    const onScrollElem = () => {
        if (elem == null) return;

        elemBottom = elem.scrollHeight - elem.offsetHeight;
        scrollTop = elem.scrollTop;
        if (scrollTop >= elemBottom - threshold) {
            reachedBottom = true;
        }
    };

    $: {
        if (elem != null) {
            elem.addEventListener("scroll", onScrollElem, { passive: true });
        }
    }

    onDestroy(() => {
        elem?.removeEventListener("scroll", onScrollElem);
    });

    $: topThreshold = scrollTop >= threshold ? 10 : 0;
    $: bottomThreshold = scrollTop >= elemBottom - threshold ? 100 : 90;
</script>

<div
    class="w-full h-full overflow-auto fadeout xs:text-sm"
    bind:this={elem}
    style={`--gradient: ${
        $isOverflowing
            ? `linear-gradient(
            transparent 0%,
            black ${topThreshold}%,
            black ${bottomThreshold}%,
            transparent 100%
        )`
            : "transparent"
    }`}
    use:overflowElem
>
    <slot />
</div>

<style>
    .fadeout {
        mask-image: var(--gradient);
        -webkit-mask-image: var(--gradient);
    }
</style>

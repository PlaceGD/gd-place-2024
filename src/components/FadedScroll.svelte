<script lang="ts">
    import { onDestroy } from "svelte";

    export let reachedBottom = false;

    let elem: HTMLDivElement | null = null;

    let threshold = 20;
    let scrollTop = 0;
    let elemBottom = threshold + 1;

    const onScrollNotice = () => {
        if (elem == null) return;

        elemBottom = elem.scrollHeight - elem.offsetHeight;
        scrollTop = elem.scrollTop;
        if (scrollTop === elem.scrollHeight - elem.offsetHeight) {
            reachedBottom = true;
        }
    };

    $: {
        if (elem != null) {
            elem.addEventListener("scroll", onScrollNotice);
        }
    }

    onDestroy(() => {
        elem?.removeEventListener("scroll", onScrollNotice);
    });

    $: topThreshold = scrollTop >= threshold ? 10 : 0;
    $: bottomThreshold = scrollTop >= elemBottom - threshold ? 100 : 90;
</script>

<div
    class="w-full h-full overflow-auto fadeout xs:text-sm"
    bind:this={elem}
    style="--gradient: linear-gradient(
            transparent 0%,
            black {topThreshold}%,
            black {bottomThreshold}%,
            transparent 100%
        )"
>
    <slot />
</div>

<style>
    .fadeout {
        mask-image: var(--gradient);
        -webkit-mask-image: var(--gradient);
    }
</style>

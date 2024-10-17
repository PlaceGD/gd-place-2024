<script lang="ts">
    import { afterUpdate, onDestroy, onMount } from "svelte";
    import { VisualObserver } from "viz-observer";

    export let target: string;
    export let tooltipSize: { width: number; height: number };

    export let tooltipLeft: number;
    export let tooltipTop: number;

    const getGuideElem = (id: string): HTMLElement | null => {
        if (typeof window == "undefined") return null;

        return document.querySelector(
            `[data-guide="${id}"]`
        ) as HTMLElement | null;
    };

    let targetElemRect: DOMRect | null;

    let observer: VisualObserver | null;
    onMount(() => {
        observer = new VisualObserver(entries => {
            targetElemRect = entries[0].contentRect;
        });

        const elem = getGuideElem(target);

        if (elem) {
            observer.observe(elem);
            targetElemRect = elem.getBoundingClientRect();
        } else {
            console.warn("no target elem found for guide:", target);
        }
    });

    onDestroy(() => {
        observer?.disconnect();
    });

    let windowHeight: number = 0;
    let windowWidth: number = 0;

    let left: number = 0;
    let right: number = 0;
    let top: number = 0;
    let bottom: number = 0;

    afterUpdate(() => {
        if (targetElemRect) {
            left = targetElemRect.left;
            right = left + targetElemRect.width;
            top = targetElemRect.top;
            bottom = top + targetElemRect.height;
            if (left > windowWidth / 2) {
                tooltipLeft = right - tooltipSize.width;
            } else {
                tooltipLeft = left;
            }

            if (top > windowHeight / 2) {
                tooltipTop = top - tooltipSize.height;
            } else {
                tooltipTop = bottom;
            }
        }
    });
</script>

<svelte:window bind:innerHeight={windowHeight} bind:innerWidth={windowWidth} />

<div
    class="fixed top-0 left-0 z-[51] w-screen h-screen bg-black/40"
    style={`
        clip-path: polygon(
            0px 0px,
            0px ${windowHeight}px,
            ${left}px ${windowHeight}px,
            ${left}px ${top}px,
            ${right}px ${top}px,
            ${right}px ${bottom}px,
            ${left}px ${bottom}px,
            ${left}px ${windowHeight}px,
            ${windowWidth}px ${windowHeight}px,
            ${windowWidth}px 0px
        )
    `}
></div>

<!-- disable clicking of the element -->
<div class="absolute w-screen h-screen z-[52] pointer-events-auto"></div>

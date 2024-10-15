<script lang="ts">
    import { VisualObserver } from "viz-observer";
    import {
        GUIDE_ELEM_IDS,
        GUIDE_STEPS,
        // guideElems,
        isGuideActive,
    } from "./guide";
    import { onDestroy, onMount } from "svelte";
    import { clamp } from "shared-lib/util";
    import ClosableWindow from "../components/ClosableWindow.svelte";
    import IconButton from "../components/Buttons/IconButton.svelte";
    import ArrowLeft from "../icons/ArrowLeft.svelte";
    import ArrowRight from "../icons/ArrowRight.svelte";

    const guideElems: Record<string, HTMLElement> = {};
    const getGuideElem = (id: string): HTMLElement | null => {
        if (typeof window == "undefined") return null;

        if (guideElems[id] != null) {
            return guideElems[id];
        }
        const elem = document.querySelector(
            `[data-guide="${id}"]`
        ) as HTMLElement | null;
        if (elem) {
            guideElems[id] = elem;
            return elem;
        }

        return null;
    };

    let innerHeight: number = 0;
    let innerWidth: number = 0;

    let tooltip: HTMLElement;

    let boundingRect: DOMRect | null;

    let observer: VisualObserver | null;
    onMount(() => {
        observer = new VisualObserver(entries => {
            if (entries[0].target.isSameNode(element)) {
                boundingRect = entries[0].contentRect;
            }
        });
    });

    let step = 0;

    let prevElement: HTMLElement | null;
    let element: HTMLElement | null;

    $: currentStep = GUIDE_STEPS[step];

    $: {
        if ($isGuideActive) {
            prevElement = getGuideElem(
                GUIDE_STEPS[clamp(step - 1, 0, GUIDE_STEPS.length - 1)].id
            );
            element = getGuideElem(currentStep.id);
        }
    }

    $: {
        if (observer && element && prevElement) {
            observer.unobserve(prevElement);
            observer.observe(element);

            // doesnt always call the callback for observing if the element has previously been observed
            boundingRect = element.getBoundingClientRect();
        }
    }

    let left: number = 0;
    let right: number = 0;
    let top: number = 0;
    let bottom: number = 0;

    let tooltipLeft: number = 0;
    let tooltipTop: number = 0;

    $: {
        if (boundingRect && tooltip) {
            left = boundingRect.left;
            right = left + boundingRect.width;
            top = boundingRect.top;
            bottom = top + boundingRect.height;

            const tooltipBox = tooltip.getBoundingClientRect();

            // on the right side
            if (left > innerWidth / 2) {
                tooltipLeft = right - tooltipBox.width;
            } else {
                tooltipLeft = left;
            }

            if (top > innerHeight / 2) {
                tooltipTop = top - tooltipBox.height;
            } else {
                tooltipTop = bottom;
            }
        }
    }

    const nextStep = () => {
        step++;
    };
    const prevStep = () => {
        step--;
    };
</script>

<svelte:window bind:innerWidth bind:innerHeight />

{#if $isGuideActive}
    <div
        class="fixed top-0 left-0 z-[51] w-screen h-screen bg-black/40"
        style={`
            clip-path: polygon(
                0px 0px,
                0px ${innerHeight}px,
                ${left}px ${innerHeight}px,
                ${left}px ${top}px,
                ${right}px ${top}px,
                ${right}px ${bottom}px,
                ${left}px ${bottom}px,
                ${left}px ${innerHeight}px,
                ${innerWidth}px ${innerHeight}px,
                ${innerWidth}px 0px
            )
        `}
    ></div>

    <!-- disable clicking of the element -->
    <div class="absolute w-screen h-screen z-[52] pointer-events-auto"></div>

    <div
        class="absolute flex flex-col z-[53]"
        style={`
            left: ${tooltipLeft}px;
            top: ${tooltipTop}px;
            
        `}
        bind:this={tooltip}
    >
        <div
            class="flex flex-col gap-2 p-4 text-white rounded-lg xs:p-2 menu-panel flex-center w-max-[400px] mx-2 mt-2"
        >
            <span class="text-base xs:text-sm">
                {currentStep.text}
            </span>
            <div class="flex w-full gap-4 xs:gap-2">
                <IconButton
                    class="w-24"
                    disabled={step == 0}
                    on:click={prevStep}
                >
                    <span slot="left">
                        <ArrowLeft
                            stroke-width={1}
                            class="w-8 h-8 xs:h-7 xs:w-7"
                        />
                    </span>
                    <span slot="children"></span>
                </IconButton>

                <div class="text-center tabular-nums">
                    {step + 1}/{GUIDE_STEPS.length}
                </div>

                <IconButton
                    class="w-24"
                    disabled={step == GUIDE_STEPS.length - 1}
                    on:click={nextStep}
                >
                    <span slot="children">
                        <ArrowRight
                            stroke-width={1}
                            class="w-8 h-8 xs:h-7 xs:w-7"
                        />
                    </span>
                    <span slot="right"></span>
                </IconButton>
            </div>
        </div>
        <!-- margin on the bottom doesnt work without this for some reason -->
        <div class="mb-2"></div>
    </div>
{/if}

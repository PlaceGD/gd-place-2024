<script lang="ts">
    import { VisualObserver } from "viz-observer";
    import {
        GUIDE_ELEM_IDS,
        GUIDE_STEPS,
        // guideElems,
        isGuideActive,
        type Step,
    } from "./guide";
    import { afterUpdate, onDestroy, onMount } from "svelte";
    import { clamp } from "shared-lib/util";
    import ClosableWindow from "../components/ClosableWindow.svelte";
    import IconButton from "../components/Buttons/IconButton.svelte";
    import ArrowLeft from "../icons/ArrowLeft.svelte";
    import ArrowRight from "../icons/ArrowRight.svelte";
    import Cross from "../icons/Cross.svelte";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";

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

    let targetElemRect: DOMRect | null;

    let observer: VisualObserver | null;
    onMount(() => {
        observer = new VisualObserver(entries => {
            if (entries[0].target.isSameNode(element)) {
                targetElemRect = entries[0].contentRect;
            }
        });
    });

    let step = 0;

    let prevElement: HTMLElement | null;
    let element: HTMLElement | null;

    let currentStep: Step;

    $: currentStep = GUIDE_STEPS[step];

    $: {
        if ($isGuideActive) {
            prevElement = getGuideElem(
                GUIDE_STEPS[clamp(step - 1, 0, GUIDE_STEPS.length - 1)].id
            );
            element = getGuideElem(currentStep.id);
        }
    }

    const cleanup = () => {
        step = 0;
        currentStep = GUIDE_STEPS[0];
        tooltipLeft = 0;
        tooltipTop = 0;
        if (observer && element) {
            observer.unobserve(element);
        }
    };

    $: if (!$isGuideActive) cleanup();

    $: {
        if (observer && element && prevElement) {
            observer.unobserve(prevElement);
            observer.observe(element);

            // doesnt always call the callback for observing if the element has previously been observed
            targetElemRect = element.getBoundingClientRect();
        }
    }

    let left: number = 0;
    let right: number = 0;
    let top: number = 0;
    let bottom: number = 0;

    let tooltipLeft: number = 0;
    let tooltipTop: number = 0;

    // we want to handle this logic after first update so the tooltip bounding box
    // has deifinitely changed to be the size of the new content
    // we could use another observer but its a bit overkill
    // im surprised this doesnt cause an infinite loop tbh
    afterUpdate(() => {
        if (targetElemRect && tooltip) {
            left = targetElemRect.left;
            right = left + targetElemRect.width;
            top = targetElemRect.top;
            bottom = top + targetElemRect.height;

            const tooltipBox = tooltip.getBoundingClientRect();

            // // some of these calculations may be wrong but its hard to know
            // // until it targets an element that satifies one of them which may not happen
            // if (targetElemRect.width > tooltipBox.width) {
            //     if (left > innerWidth / 2) {
            //         tooltipLeft =
            //             right - tooltipBox.width / 2 - targetElemRect.width / 2;
            //     } else {
            //         tooltipLeft =
            //             left + targetElemRect.width / 2 - tooltipBox.width / 2;
            //     }
            // } else {
            // }
            if (left > innerWidth / 2) {
                tooltipLeft = right - tooltipBox.width;
            } else {
                tooltipLeft = left;
            }

            // if (targetElemRect.height > tooltipBox.height) {
            //     if (top > innerHeight / 2) {
            //         tooltipTop =
            //             top - tooltipBox.height / 2 - targetElemRect.height / 2;
            //     } else {
            //         tooltipTop =
            //             bottom +
            //             targetElemRect.height / 2 -
            //             tooltipBox.height / 2;
            //     }
            // } else {
            // }
            if (top > innerHeight / 2) {
                tooltipTop = top - tooltipBox.height;
            } else {
                tooltipTop = bottom;
            }
        }
    });

    // $: {
    //     if (targetElemRect && tooltip) {
    //         left = targetElemRect.left;
    //         right = left + targetElemRect.width;
    //         top = targetElemRect.top;
    //         bottom = top + targetElemRect.height;

    //         const tooltipBox = tooltip.getBoundingClientRect();

    //         console.log(tooltipBox);

    //         // some of these calculations will be wrong but its hard to know
    //         // until it targets an element that satifies one of them which may not happen
    //         if (targetElemRect.width > tooltipBox.width) {
    //             if (left > innerWidth / 2) {
    //                 tooltipLeft =
    //                     right - tooltipBox.width / 2 - targetElemRect.width / 2;
    //             } else {
    //                 tooltipLeft =
    //                     left + targetElemRect.width / 2 - tooltipBox.width / 2;
    //             }
    //         } else {
    //             if (left > innerWidth / 2) {
    //                 tooltipLeft = right - tooltipBox.width;
    //             } else {
    //                 tooltipLeft = left;
    //             }
    //         }

    //         if (top > innerHeight / 2) {
    //             tooltipTop = top - tooltipBox.height;
    //         } else {
    //             tooltipTop = bottom;
    //         }

    //         if (targetElemRect.height > tooltipBox.height) {
    //         }
    //     }
    // }

    const nextStep = async () => {
        let s = GUIDE_STEPS[step + 1];
        await s?.beforeCb?.();

        step++;
    };
    const prevStep = async () => {
        let s = GUIDE_STEPS[step - 1];
        await s?.beforeCb?.();

        step--;
    };
</script>

<svelte:window
    bind:innerWidth
    bind:innerHeight
    on:keyup={e => {
        if (e.key == "Escape") {
            $isGuideActive = false;
        }
    }}
/>

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
    <dialog
        class="absolute w-screen h-screen z-[52] pointer-events-auto"
    ></dialog>

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
                    <span slot="children">
                        <ArrowLeft
                            stroke-width={1}
                            class="w-8 h-8 xs:h-7 xs:w-7"
                        />
                    </span>
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
                </IconButton>
            </div>
        </div>
        <!-- margin on the bottom doesnt work without this for some reason -->
        <div class="mb-2"></div>
    </div>

    <div
        class="absolute text-white left-1/2 -translate-x-1/2 top-6 xs:top-3 z-[53]"
    >
        <WhiteButton
            aria-label="Exit guide"
            on:click={() => {
                $isGuideActive = false;
            }}
        >
            <Cross aria-label="Exit" class="stroke-1 w-7 h-7" />
        </WhiteButton>
    </div>
{/if}

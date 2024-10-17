<script lang="ts">
    import {
        GUIDE_STEPS,
        // guideElems,
        isGuideActive,
    } from "./guide";
    import IconButton from "../components/Buttons/IconButton.svelte";
    import ArrowLeft from "../icons/ArrowLeft.svelte";
    import ArrowRight from "../icons/ArrowRight.svelte";
    import Cross from "../icons/Cross.svelte";
    import WhiteButton from "../components/Buttons/WhiteButton.svelte";
    import type { GuideAction } from "./guideActions";

    let tooltipLeft: number = 0;
    let tooltipTop: number = 0;

    let step = 0;
    let currentStep: GuideAction;

    let tooltipWidth: number = 0;
    let tooltipHeight: number = 0;

    $: {
        currentStep = GUIDE_STEPS[step];
    }

    const nextStep = async () => {
        step++;
    };
    const prevStep = async () => {
        step--;
    };
</script>

<svelte:window
    on:keyup={e => {
        if (e.key == "Escape") {
            $isGuideActive = false;
        }
    }}
/>

{#if $isGuideActive}
    {#if currentStep.component}
        {@const props = currentStep?.props?.() ?? {}}
        <svelte:component
            this={currentStep.component}
            {...props}
            tooltipSize={{ width: tooltipWidth, height: tooltipHeight }}
            bind:tooltipTop
            bind:tooltipLeft
        />
    {/if}

    <div
        class="absolute flex flex-col z-[53]"
        style={`
            left: ${tooltipLeft}px;
            top: ${tooltipTop}px;
            
        `}
        bind:clientWidth={tooltipWidth}
        bind:clientHeight={tooltipHeight}
    >
        <div
            class="flex flex-col gap-2 p-4 text-white rounded-lg xs:p-2 menu-panel flex-center w-max-[400px] mx-2 mt-2"
        >
            <span class="text-base xs:text-sm">
                {currentStep.description}
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

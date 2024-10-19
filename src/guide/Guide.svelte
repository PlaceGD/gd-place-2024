<script lang="ts">
    import * as wasm from "wasm-lib";
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
    import GenericAction from "./GenericAction.svelte";
    import Check from "../icons/Check.svelte";

    export let state: wasm.State;

    let tooltipLeft: number = 0;
    let tooltipTop: number = 0;

    let step = -1;
    let prevStep = 0;
    // only updated once the async actions have been awaited
    let delayedStep = 0;

    $: canChangeStep = delayedStep == step;

    let currentStep: GuideAction | null = null;

    let tooltipSize = { width: 0, height: 0 };

    $: {
        if ($isGuideActive) {
            goNextStep();
        }
    }

    $: {
        if (!$isGuideActive) {
            step = -1;
            delayedStep = 0;
            currentStep = null;
        }
    }

    const goNextStep = async () => {
        if (step + 1 >= GUIDE_STEPS.length) {
            await GUIDE_STEPS[step]?.onEndAction?.({
                state,
                tooltipSize,
                direction: 1,
                nextStep: goNextStep,
                prevStep: goPrevStep,
            });

            $isGuideActive = false;
        } else {
            await GUIDE_STEPS[step]?.onEndAction?.({
                state,
                tooltipSize,
                direction: 1,
                nextStep: goNextStep,
                prevStep: goPrevStep,
            });
            await GUIDE_STEPS[step + 1]?.onBeginAction?.({
                state,
                tooltipSize,
                direction: 1,
                nextStep: goNextStep,
                prevStep: goPrevStep,
            });

            prevStep = step;
            step = step + 1;

            delayedStep = step;
            currentStep = GUIDE_STEPS[step];
        }
    };
    const goPrevStep = async () => {
        await GUIDE_STEPS[step]?.onEndAction?.({
            state,
            tooltipSize,
            direction: -1,
            nextStep: goNextStep,
            prevStep: goPrevStep,
        });
        await GUIDE_STEPS[step - 1]?.onBeginAction?.({
            state,
            tooltipSize,
            direction: -1,
            nextStep: goNextStep,
            prevStep: goPrevStep,
        });

        prevStep = step;
        step = step - 1;

        delayedStep = step;
        currentStep = GUIDE_STEPS[step];
    };

    $: console.log(currentStep);
</script>

<svelte:window
    on:keyup={e => {
        if (e.key == "Escape") {
            $isGuideActive = false;
        }
    }}
/>

{#if currentStep != null}
    {#if currentStep?.getComponent?.() != undefined}
        {@const props = currentStep?.getProps?.() ?? {}}
        <svelte:component
            this={currentStep.getComponent()}
            {...props}
            {state}
            {tooltipSize}
            bind:tooltipTop
            bind:tooltipLeft
        />
    {:else}
        <GenericAction
            step={currentStep}
            {state}
            {tooltipSize}
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
        bind:clientWidth={tooltipSize.width}
        bind:clientHeight={tooltipSize.height}
    >
        <div
            class="flex flex-col gap-2 p-4 text-white rounded-lg xs:p-2 menu-panel flex-center max-w-[400px] outline-2 outline outline-white/20 m-4"
        >
            <span class="text-base sm:text-sm xs:text-xs">
                {currentStep.description}
            </span>
            <div class="flex w-full gap-4 xs:gap-2">
                {#if !currentStep.getRequiresInteraction()}
                    <IconButton
                        class="w-24"
                        disabled={step == 0 || !canChangeStep}
                        on:click={goPrevStep}
                    >
                        <span slot="children">
                            <ArrowLeft
                                stroke-width={1}
                                class="w-8 h-8 xs:h-7 xs:w-7"
                            />
                        </span>
                    </IconButton>
                {/if}
                <div class="text-center tabular-nums hover-text-transition">
                    {delayedStep + 1}/{GUIDE_STEPS.length}
                </div>
                {#if !currentStep.getRequiresInteraction()}
                    <IconButton
                        class="w-24"
                        disabled={!canChangeStep}
                        on:click={goNextStep}
                    >
                        <span slot="children">
                            {#if delayedStep < GUIDE_STEPS.length - 1}
                                <ArrowRight
                                    stroke-width={1}
                                    class="w-8 h-8 xs:h-7 xs:w-7"
                                />
                            {:else}
                                <Check
                                    stroke-width={1}
                                    class="w-8 h-8 xs:h-7 xs:w-7"
                                />
                            {/if}
                        </span>
                    </IconButton>
                {/if}
            </div>
        </div>
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

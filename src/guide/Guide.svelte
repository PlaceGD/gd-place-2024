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
    import { get, type Unsubscriber } from "svelte/store";

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

    let requiresInteractionUnsub: Unsubscriber | undefined;
    let requiresInteraction = false;
    $: requiresInteractionUnsub = currentStep?.requiresInteraction.subscribe(
        v => (requiresInteraction = v)
    );

    let canInteractUnsub: Unsubscriber | undefined;
    let canInteract = false;
    $: canInteractUnsub = currentStep?.canInteract.subscribe(
        v => (canInteract = v)
    );

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
        requiresInteractionUnsub?.();
        canInteractUnsub?.();

        if (step + 1 >= GUIDE_STEPS.length) {
            await GUIDE_STEPS[step]?.onEndAction?.({
                state,
                tooltipSize,
                nextStep: goNextStep,
            });

            $isGuideActive = false;
        } else {
            await GUIDE_STEPS[step]?.onEndAction?.({
                state,
                tooltipSize,
                nextStep: goNextStep,
            });
            await GUIDE_STEPS[step + 1]?.onBeginAction?.({
                state,
                tooltipSize,
                nextStep: goNextStep,
            });

            prevStep = step;
            step = step + 1;

            delayedStep = step;
            currentStep = GUIDE_STEPS[step];
        }
    };
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
            {canInteract}
            bind:tooltipTop
            bind:tooltipLeft
        />
    {/if}

    <div
        class="absolute z-[53]"
        style={`
            left: ${tooltipLeft}px;
            top: ${tooltipTop}px;    
        `}
        bind:clientWidth={tooltipSize.width}
        bind:clientHeight={tooltipSize.height}
    >
        <div
            class="z-[53] flex flex-col gap-2 p-4 text-white rounded-lg xs:p-2 menu-panel flex-center max-w-[400px] xs:max-w-[350px] outline-2 outline pulsing-outline m-4 xs:m-2"
        >
            <span class="text-base sm:text-sm xs:text-xs">
                {@html currentStep.description}
            </span>
            <div class="flex items-center justify-between w-full gap-2">
                <div class="flex gap-4 xs:gap-2 flex-center">
                    <div
                        class="text-center tabular-nums hover-text-transition xs:text-sm"
                    >
                        {delayedStep + 1}/{GUIDE_STEPS.length}
                    </div>
                    {#if true}
                        {#if !requiresInteraction}
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
                    {/if}
                </div>
                <IconButton
                    aria-label="Exit guide"
                    on:click={() => {
                        $isGuideActive = false;
                    }}
                >
                    <span slot="children">
                        <Cross
                            aria-label="Exit"
                            class="w-8 h-8 stroke-1 xs:h-7 xs:w-7"
                        />
                    </span>
                </IconButton>
            </div>
        </div>
    </div>
{/if}

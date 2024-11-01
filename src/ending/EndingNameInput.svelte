<script lang="ts">
    import { fade } from "svelte/transition";
    import Input from "../components/Input.svelte";
    import {
        CHARACTER_COOLDOWN_SECONDS,
        TOTAL_ENDING_INPUTS,
        VALID_LEVEL_NAME,
    } from "shared-lib/ending";
    import { setLevelNameLetter } from "../firebase/cloud_functions";
    import { onDestroy, onMount } from "svelte";
    import { db } from "../firebase/firebase";
    import type { Unsubscribe } from "firebase/database";
    import { SyncedCooldown } from "../utils/cooldown";
    import { loginData } from "../stores";
    import { cubicInOut } from "svelte/easing";
    import { isGuideActive } from "../guide/guide";
    import { toast } from "@zerodevx/svelte-toast";

    const characterCooldown = SyncedCooldown.new(
        `userDetails/${$loginData.currentUserData!.user.uid}/lastCharacterTimestamp`,
        CHARACTER_COOLDOWN_SECONDS
    );
    let {
        display: characterCooldownDisplay,
        finished: characterCooldownFinished,
    } = characterCooldown;

    let letters: string[] = Array(TOTAL_ENDING_INPUTS).fill(" ");

    let unsub: Unsubscribe | null;
    onMount(async () => {
        document
            .getElementById("level-canvas")
            ?.style.setProperty("--canvas-fade", `transparent 10%`);

        $isGuideActive = false;
        toast.pop();
        toast.pop({ target: "announcement" });

        unsub = db.ref("/levelName/inputs").on("value", v => {
            Object.entries(v?.val() ?? {}).forEach(([key, value]) => {
                let index = parseInt(key);
                letters[index] = value ?? " ";
            });
        });

        let interval = setInterval(() => {
            lettersVisible += 1;

            if (lettersVisible > titleText.length) {
                clearInterval(interval);
            }
        }, 100);
    });

    onDestroy(() => {
        document
            .getElementById("level-canvas")
            ?.style.setProperty("--canvas-fade", `white`);

        unsub?.();
    });

    const titleText = "ENTER LEVEL NAME:";
    let lettersVisible = 0;

    let inputsDisabled = false;

    $: {
        let _ = $characterCooldownFinished;
        inputsDisabled = false;
    }
</script>

<div
    class="absolute z-50 flex flex-col w-full h-full gap-32 pointer-events-none flex-center ending-container"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
>
    <h1
        class="text-center text-white font-exo text-7xl md:text-5xl sm:text-4xl xs:text-3xl enter-level-name-text"
    >
        {titleText.slice(0, lettersVisible)}
    </h1>

    <div class="flex flex-col gap-2 flex-center">
        <div class="content-center justify-center ending-grid">
            {#each Array(TOTAL_ENDING_INPUTS) as _, i (i)}
                <div
                    class="relative flex w-auto h-full character-input text-stroke backdrop-blur-sm flex-center"
                    transition:fade|global={{
                        delay: titleText.length * 100 + 1000 + i * 125,
                        duration: 200,
                        easing: cubicInOut,
                    }}
                >
                    <span
                        class="absolute flex w-full h-full text-center flex-center font-pusab text-stroke"
                    >
                        {letters[i]}
                    </span>

                    <input
                        class="absolute w-full h-full text-transparent bg-transparent pointer-events-auto select-none outline-2 outline-transparent focus:outline-red caret-transparent"
                        on:keydown={async e => {
                            if (!$characterCooldownFinished) return;
                            let key = null;
                            if (e.key === "Backspace") {
                                key = " ";
                            } else {
                                if (VALID_LEVEL_NAME.test(e.key)) {
                                    key = e.key;
                                }
                            }

                            if (key != null) {
                                letters[i] = key;

                                inputsDisabled = true;

                                setLevelNameLetter({
                                    index: i,
                                    letter: key,
                                });
                            }
                        }}
                        disabled={!$characterCooldownFinished || inputsDisabled}
                    />
                </div>
            {/each}
        </div>
        <div
            class="p-4 text-5xl text-white rounded-sm"
            transition:fade|global={{
                delay:
                    titleText.length * 100 +
                    1000 +
                    TOTAL_ENDING_INPUTS * 125 +
                    300,
                duration: 200,
                easing: cubicInOut,
            }}
        >
            {$characterCooldownDisplay}
        </div>
    </div>
</div>

<style lang="postcss">
    .ending-grid {
        --input-width: 85px;
        --input-height: calc(var(--input-width) * 1.5);

        @apply grid gap-4;
        grid-template-columns: repeat(var(--count), var(--input-width));
        grid-template-rows: var(--input-height);
    }

    .character-input {
        --box-shadow-thickness: 3px;

        @apply text-center font-pusab text-7xl text-white !outline-none;

        box-shadow:
            0px 0px 0px var(--box-shadow-thickness) #989696,
            /* 0px 0px 0px calc(var(--box-shadow-thickness) * 2) #363535, */ 0px
                0px 70px 0px #ffffff28;
        background: linear-gradient(0deg, #18181833 0%, #03030333 100%);
    }

    @media only screen and (max-width: 2045px) {
        .ending-grid {
            grid-template-columns: repeat(
                calc(var(--count) / 2),
                var(--input-width)
            );
            grid-template-rows: repeat(2, var(--input-height));
        }
    }

    @media only screen and (max-width: 1025px) {
        .ending-grid {
            --input-width: 65px;
        }

        .ending-grid {
            grid-template-columns: repeat(
                calc(var(--count) / 4),
                var(--input-width)
            );
            grid-template-rows: repeat(4, var(--input-height));
        }

        .character-input {
            @apply text-5xl;
        }
    }

    @media only screen and (max-width: 525px) {
        .ending-grid {
            --input-width: 45px;
            @apply gap-2;
        }

        .character-input {
            --box-shadow-thickness: 2px;
            @apply text-4xl;
        }
    }

    :global(#level-canvas) {
        mask: linear-gradient(0deg, var(--canvas-fade, white), white);
    }

    .enter-level-name-text {
        text-shadow: 0px 0px 70px #ffffff69;
    }

    .ending-container::before {
        filter: url(#filter);
        --size: 45px;
        --line: #ffffff1a;
        content: "";
        height: 100vh;
        width: 100vw;
        position: fixed;
        background:
            linear-gradient(90deg, var(--line) 2px, transparent 2px var(--size))
                50% 50% / var(--size) var(--size),
            linear-gradient(var(--line) 2px, transparent 2px var(--size)) 50%
                50% / var(--size) var(--size);
        mask: linear-gradient(0deg, transparent 300px, white),
            radial-gradient(
                circle,
                transparent 0%,
                transparent 300px,
                white 480px,
                white
            );
        mask-composite: intersect;
        top: 0;
        transform-style: flat;
        pointer-events: none;
        z-index: -1;
    }
</style>

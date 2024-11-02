<script lang="ts">
    import { default as cx } from "classnames";
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
    import {
        eventElapsed,
        eventElapsedContinuous,
        eventEndTime,
        loginData,
    } from "../stores";
    import { cubicInOut } from "svelte/easing";
    import { isGuideActive } from "../guide/guide";
    import { toast } from "@zerodevx/svelte-toast";
    import { readable } from "svelte/store";
    import Loading from "../components/Loading.svelte";
    import { clamp } from "shared-lib/util";
    import { LEVEL_NAME_DELAY } from "./ending";

    const DURATION = 10000;

    // let percentage = 0;
    // const updateVignettes = () => {
    //     percentage += 0.1;

    //     raf = requestAnimationFrame(updateVignettes);
    // };

    const characterCooldown = $loginData.currentUserData
        ? SyncedCooldown.new(
              `userDetails/${$loginData.currentUserData!.user.uid}/lastCharacterTimestamp`,
              CHARACTER_COOLDOWN_SECONDS
          )
        : null;
    let {
        display: characterCooldownDisplay,
        finished: characterCooldownFinished,
    } = characterCooldown ?? {
        display: readable(""),
        finished: readable(false),
    };

    let letters: string[] = Array(TOTAL_ENDING_INPUTS).fill(" ");

    let unsub: Unsubscribe | null;

    onMount(async () => {
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
        unsub?.();
    });

    const titleText = "ENTER LEVEL NAME:";
    let lettersVisible = 0;

    let inputsDisabled = false;

    $: {
        $characterCooldownFinished;
        inputsDisabled = false;
    }
    $: vignetteProgress = clamp(
        ($eventElapsedContinuous - ($eventEndTime + LEVEL_NAME_DELAY + 3000)) /
            DURATION,
        0,
        1
    );
</script>

<div class="contents" style={`--vignette-progress: ${vignetteProgress}`}>
    <span
        class="absolute z-[70] pointer-events-none size-full vignette-2"
        style={``}
    ></span>
    <span class="absolute z-[60] pointer-events-none size-full vignette"></span>
    <span class="absolute z-30 pointer-events-none size-full dagrid"></span>
</div>

<div
    class="absolute z-40 flex flex-col w-full h-full gap-32 pointer-events-none sm:gap-28 xs:gap-24 flex-center ending-container"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
>
    <h1
        class="text-center text-white font-share text-7xl md:text-5xl sm:text-4xl xs:text-3xl enter-level-name-text"
    >
        {titleText.slice(0, lettersVisible)}
    </h1>

    <div class="relative flex flex-col gap-2 flex-center">
        <div class="relative w-full h-full">
            <div class="content-center justify-center ending-grid">
                {#each Array(TOTAL_ENDING_INPUTS) as _, i (i)}
                    <div
                        class="relative flex w-auto h-full flex-center"
                        in:fade|global={{
                            delay: titleText.length * 100 + 1000 + i * 125,
                            duration: 200,
                            easing: cubicInOut,
                        }}
                    >
                        <input
                            class="character-input-input backdrop-blur-md"
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
                            disabled={!$characterCooldownFinished ||
                                inputsDisabled ||
                                $loginData.currentUserData?.userDetails == null}
                        />

                        <span
                            class="absolute z-20 flex w-full h-full text-center pointer-events-none font-pusab text-stroke flex-center character-input"
                        >
                            {letters[i]}
                        </span>
                    </div>
                {/each}
            </div>
        </div>
        <div class="h-12 p-4 text-5xl text-white sm:text-4xl xs:text-3xl">
            {#if inputsDisabled}
                <div class="relative z-40 w-11 h-11 xs:h-10 xs:w-10">
                    <Loading darken={false} />
                </div>
            {:else if !$characterCooldownFinished}
                {$characterCooldownDisplay}
            {/if}
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
    .ending-container {
        animation: rotateAnimation 1s linear infinite;
    }

    .character-input {
        @apply text-center font-pusab text-7xl text-white;
    }

    .character-input-input {
        --box-shadow-thickness: 3px;
        @apply pointer-events-auto relative z-10 h-full w-full cursor-text select-none rounded-md text-transparent caret-transparent !outline-none;

        background: linear-gradient(0deg, #18181833 0%, #03030333 100%);
        border: var(--box-shadow-thickness) solid #747272;
        box-shadow: 0px 0px 70px 0px #ffffff28;
    }

    .character-input-input:not(:disabled):hover {
        background: linear-gradient(0deg, #181818 0%, #03030333 100%);
    }

    @keyframes pulse-opacity {
        0% {
            opacity: 70%;
        }
        50% {
            opacity: 20%;
        }
        100% {
            opacity: 70%;
        }
    }

    .character-input-input:focus {
        border: var(--box-shadow-thickness) solid #fff;
        box-shadow: 0px 0px 70px 0px #ffffff28;
    }

    .character-input-input:focus ~ .character-input {
        animation: pulse-opacity 1s ease-in-out forwards infinite;
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

        .character-input-input {
            --box-shadow-thickness: 2px;
        }

        .character-input {
            @apply text-4xl;
        }
    }

    .enter-level-name-text {
        text-shadow: 0px 0px 70px #ffffff69;
    }

    .dagrid::before {
        --size: 45px;
        --line: #ffffff08;
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
        mix-blend-mode: plus-lighter;
        opacity: 1;
        transition: 4s ease-in-out opacity;
    }

    @property --vignette-color {
        syntax: "<color>";
        initial-value: #c51111;
        inherits: false;
    }

    @keyframes pulse-vignette {
        0% {
            --vignette-color: #ff0000;
        }
        100% {
            --vignette-color: #c51111;
        }
    }

    .vignette {
        background: radial-gradient(
            circle,
            transparent 0%,
            transparent calc(100% - var(--vignette-progress) * 100%),
            var(--vignette-color) calc(170% - var(--vignette-progress) * 120%)
        );
        mix-blend-mode: multiply;
        opacity: 100%;
        animation: pulse-vignette 1s cubic-bezier(0.33, 1, 0.68, 1) forwards
            infinite;
    }

    .vignette-2 {
        background: radial-gradient(
            circle,
            transparent 0%,
            transparent calc(130% - var(--vignette-progress) * 70%),
            var(--vignette-color) calc(470% - var(--vignette-progress) * 70%)
        );
        /* background-image: url("https://i.pinimg.com/originals/f7/71/e9/f771e93c26b66a17eaa3495813fab033.gif"); */
        mix-blend-mode: plus-lighter;
        opacity: 100%;
        animation: pulse-vignette 1s cubic-bezier(0.33, 1, 0.68, 1) forwards
            infinite;
    }
</style>

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
        unsub = db.ref("/levelName/inputs").on("value", v => {
            Object.entries(v.val()).forEach(([key, value]) => {
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
</script>

<div
    class="absolute z-50 flex flex-col w-full h-full pointer-events-none flex-center gap-32"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
>
    <h1
        class="font-exo text-7xl md:text-5xl sm:text-4xl xs:text-3xl text-white text-center enter-level-name-text"
    >
        {titleText.slice(0, lettersVisible)}
    </h1>
    <div class="flex flex-col gap-2 flex-center">
        <div class="content-center justify-center ending-grid">
            {#each Array(TOTAL_ENDING_INPUTS) as _, i (i)}
                <div
                    class="relative w-auto h-full character-input text-stroke backdrop-blur-sm flex flex-center"
                    transition:fade|global={{
                        delay: titleText.length * 100 + 1000 + i * 125,
                        duration: 200,
                        easing: cubicInOut,
                    }}
                >
                    <span
                        class="absolute w-full h-full flex flex-center text-center font-pusab text-stroke"
                    >
                        {letters[i]}
                    </span>

                    <input
                        class="w-full h-full absolute pointer-events-auto bg-transparent outline-2 outline-transparent focus:outline-red caret-transparent text-transparent"
                        on:keydown={e => {
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

                                setLevelNameLetter({
                                    index: i,
                                    letter: key,
                                });
                            }
                        }}
                        disabled={!$characterCooldownFinished}
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

    :global(.character-input) {
        --box-shadow-thickness: 3px;

        @apply text-center font-pusab text-7xl text-white !outline-none;

        box-shadow:
            0px 0px 0px var(--box-shadow-thickness) #989696,
            0px 0px 0px calc(var(--box-shadow-thickness) * 2) #363535;
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

        :global(.character-input) {
            @apply text-5xl;
        }
    }

    @media only screen and (max-width: 525px) {
        .ending-grid {
            --input-width: 45px;
            @apply gap-2;
        }

        :global(.character-input) {
            --box-shadow-thickness: 2px;
            @apply text-4xl;
        }
    }

    .enter-level-name-text {
        text-shadow:
            0 0 2px #92ffe9,
            0 0 8px #affbf1,
            0 0 16px #63a7a7,
            0 0 24px #397d7d,
            0 0 4px #2d5959,
            0 0 40px #264646;
    }
</style>

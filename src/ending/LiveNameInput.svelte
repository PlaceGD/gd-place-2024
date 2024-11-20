<script lang="ts">
    import { default as cx } from "classnames";
    import { fade } from "svelte/transition";
    import Input from "../components/Input.svelte";
    import {
        CHARACTER_COOLDOWN_SECONDS,
        LEVEL_NAME_DELAY,
        TOTAL_ENDING_INPUTS,
        VALID_LEVEL_NAME,
    } from "shared-lib/ending";
    import {
        getCharacterCooldown,
        setLevelNameLetter,
    } from "../firebase/cloud_functions";
    import { onDestroy, onMount } from "svelte";
    import { db } from "../firebase/firebase";
    import type { Unsubscribe } from "firebase/database";
    import { Cooldown } from "../utils/cooldown";
    import {
        DEFAULT_SETTINGS,
        editorSettings,
        eventElapsedContinuous,
        eventEndTime,
        eventStartTime,
        loginData,
        setNameSeconds,
    } from "../stores";
    import enterLevelNameSoundUrl from "../assets/enter_level_name_sound.mp3?url";
    import { cubicInOut } from "svelte/easing";
    import { isGuideActive } from "../guide/guide";
    import { toast } from "@zerodevx/svelte-toast";
    import { readable } from "svelte/store";
    import Loading from "../components/Loading.svelte";
    import { clamp } from "shared-lib/util";
    import { CROSSFADE_DURATION } from "./ending";
    import "./ending_styles.css";
    import { disappear } from "../utils/transitions";
    import { playSound } from "../utils/audio";
    import heartBeatUrl from "../assets/heartbeat.mp3?url";
    import { Howler } from "howler";
    import Toast from "../utils/toast";

    const VIGNETTE_DELAY = LEVEL_NAME_DELAY + 3;

    const characterCooldown = new Cooldown(
        getCharacterCooldown,
        loginData,
        "lastCharacterTimestamp"
    );
    let {
        display: characterCooldownDisplay,
        finished: characterCooldownFinished,
    } = characterCooldown;

    let letters: string[] = Array(TOTAL_ENDING_INPUTS).fill(" ");

    let unsub: Unsubscribe | null;
    onMount(async () => {
        playSound({
            url: enterLevelNameSoundUrl,
            volume: 2.0,
            exclusiveChannel: "enter-level-name",
        });

        loopHeartbeatSound();

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
        characterCooldown.cleanup();
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
        ($eventElapsedContinuous -
            ($eventEndTime / 1000 - $eventStartTime / 1000 + VIGNETTE_DELAY)) /
            ($setNameSeconds - VIGNETTE_DELAY),
        0,
        1
    );

    const loopHeartbeatSound = () => {
        let interval = setInterval(() => {
            if (vignetteProgress >= 0.99999) {
                clearInterval(interval);
            } else {
                playSound({
                    url: heartBeatUrl,
                    volume: 2 * Math.pow(vignetteProgress, 3),
                });
            }
        }, 1000);
    };
</script>

<div class="contents" style={`--vignette-progress: ${vignetteProgress}`}>
    <span
        class="absolute z-[70] pointer-events-none size-full vignette-2"
        out:fade={{ duration: CROSSFADE_DURATION }}
    />
    <span
        class="absolute z-[60] pointer-events-none size-full vignette"
        out:fade={{ duration: CROSSFADE_DURATION }}
    />
    <span
        class="absolute z-30 pointer-events-none size-full dagrid"
        out:fade={{ duration: CROSSFADE_DURATION }}
    />
</div>

<div
    class="absolute z-40 w-full h-full pointer-events-none page-grid"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
>
    <h1
        class="text-center text-white font-share text-7xl md:text-5xl sm:text-4xl xs:text-3xl enter-level-name-text"
        out:fade={{ duration: CROSSFADE_DURATION }}
    >
        {titleText.slice(0, lettersVisible)}
    </h1>

    <div
        class="relative flex flex-col gap-2 flex-center self-baseline"
        out:fade={{ duration: CROSSFADE_DURATION }}
    >
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
                            out:disappear
                            class="pointer-events-auto character-input-input backdrop-blur-md"
                            unselectable="on"
                            maxlength={1}
                            on:keydown={async e => {
                                e.preventDefault();

                                const prevLetter = letters[i];

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
                                    })
                                        .then(c => {
                                            characterCooldown.start(
                                                c.data.cooldown
                                            );
                                        })
                                        .catch(() => {
                                            letters[i] = prevLetter;
                                            inputsDisabled = false;
                                            Toast.showErrorToast(
                                                "Failed to set letter!"
                                            );
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
    .enter-level-name-text {
        text-shadow: 0px 0px 70px #ffffff69;
    }

    @property --fade-pos {
        syntax: "<number>";
        initial-value: 0;
        inherits: false;
    }

    @keyframes fade-in {
        0% {
            opacity: 0;
            --fade-pos: 0;
        }
        100% {
            opacity: 1;
            --fade-pos: 300;
        }
    }

    .dagrid::before {
        --grid-size: 45px;
        --line: #ffffff08;
        content: "";
        height: 100vh;
        width: 100vw;
        position: fixed;
        background:
            linear-gradient(
                    90deg,
                    var(--line) 2px,
                    transparent 2px var(--grid-size)
                )
                50% 50% / var(--grid-size) var(--grid-size),
            linear-gradient(var(--line) 2px, transparent 2px var(--grid-size))
                50% 50% / var(--grid-size) var(--grid-size);
        mask: linear-gradient(
                0deg,
                transparent calc(var(--fade-pos) * 1px),
                white
            ),
            radial-gradient(
                circle,
                transparent 0%,
                transparent calc(var(--fade-pos) * 1px),
                white 480px,
                white
            );
        mask-composite: intersect;
        top: 0;
        transform-style: flat;
        pointer-events: none;
        z-index: -1;
        mix-blend-mode: plus-lighter;
        animation: fade-in 5s forwards;
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

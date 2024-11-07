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
    import { Cooldown } from "../utils/cooldown";
    import { loginData, viewingLevelAfterEvent } from "../stores";
    import {
        cubicInOut,
        expoIn,
        quadIn,
        quartIn,
        sineIn,
        sineInOut,
    } from "svelte/easing";

    import "./ending_styles.css";
    import { CROSSFADE_DURATION } from "./ending";
    import { clamp } from "shared-lib/util";
    import KofiButton from "../components/KofiButton.svelte";
    import { disappear } from "../utils/transitions";
    // import { typewriter } from "../utils/transitions";

    const GLOBAL_DELAY = 1000;
    const CHARACTER_DELAY = 125;
    const CHARACTER_DURATION = 3000;

    let letters: string[] = Array(TOTAL_ENDING_INPUTS).fill(" ");

    let unsub: Unsubscribe | null;

    onMount(async () => {
        unsub = db.ref("/levelName/inputs").on("value", v => {
            Object.entries(v?.val() ?? {}).forEach(([key, value]) => {
                let index = parseInt(key);
                letters[index] = value ?? " ";
            });
        });

        setTimeout(() => {
            let interval = setInterval(() => {
                lettersVisible += 1;

                if (lettersVisible > letters.length) {
                    clearInterval(interval);
                }
            }, CHARACTER_DELAY);
        }, 3700);
    });

    onDestroy(() => {
        unsub?.();
    });

    let lettersVisible = 0;

    let target: HTMLElement;

    export const fadeOut = (
        _: HTMLElement,
        { duration }: { duration: number }
    ) => {
        return {
            duration,
            css: (t: number) => `opacity: ${1 - t}`,
        };
    };

    export const moveToTitle = (
        node: HTMLElement,
        {
            duration,
            delay,
            offcenter,
        }: { duration: number; delay: number; offcenter: number }
    ) => {
        const screenCenter = window.innerWidth / 2;
        const nodePos = node.getBoundingClientRect();
        const nodeX = nodePos.left + nodePos.width / 2;
        const targetPos = target.getBoundingClientRect();

        return {
            delay,
            duration,
            easing: expoIn,
            css: (t: number) => {
                return `
                    opacity: ${1 - t};
                    transform: translate(${(screenCenter - nodeX) * 0.8 * t}px, ${(targetPos.top - nodePos.bottom + nodePos.height / 2) * t}px);
                `;
            },
        };
    };
</script>

<div
    class="absolute z-40 w-full h-full pointer-events-none page-grid"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
    out:fade|global={{ duration: 1000 }}
>
    <span class="flex flex-col gap-2 p-4 text-white flex-center xs:p-2">
        <p
            class="text-4xl text-center sm:text-3xl xs:text-2xl font-share text-stroke"
            in:fade|global={{
                duration: CROSSFADE_DURATION,
                delay: GLOBAL_DELAY + 1000,
            }}
            out:disappear
        >
            LEVEL NAME:
        </p>
        <h1
            class="text-center font-pusab text-stroke text-7xl md:text-5xl sm:text-3xl xs:text-xl enter-level-name-texttext-center min-h-[72px] md:min-h-[48px] sm:min-h-[40px] xs:min-h-[36px]"
            bind:this={target}
        >
            {letters.slice(0, lettersVisible).join("")}
        </h1>
        <span
            class="flex flex-col gap-1 pointer-events-auto flex-center"
            in:fade|global={{
                duration: CROSSFADE_DURATION,
                delay:
                    CHARACTER_DURATION +
                    GLOBAL_DELAY +
                    TOTAL_ENDING_INPUTS * CHARACTER_DELAY +
                    1000,
            }}
            out:disappear
        >
            <p
                class="text-2xl italic text-center pointer-events-auto xs:text-base hover-text-transition"
            >
                THANK YOU FOR PARTICIPATING! ❤
            </p>
            <p
                class="p-4 text-center xs:p-2 menu-panel max-w-[450px] sm:max-w-[350px] flex flex-col gap-1 flex-center"
            >
                <span class="text-base xs:text-sm">
                    This website took a long time to make. If you enjoyed it,
                    we'd really appreciate if you left a small donation. Thank
                    you :)
                </span>
                <KofiButton />
            </p>
        </span>
    </span>

    <div class="content-center justify-center row-start-2 ending-grid">
        {#each Array(TOTAL_ENDING_INPUTS) as _, i (i)}
            <div class="relative flex w-auto h-full flex-center">
                <div
                    in:fadeOut|global={{ duration: CROSSFADE_DURATION }}
                    class="opacity-0 pointer-events-none character-input-input backdrop-blur-md"
                />
                <span
                    class="absolute z-20 flex w-full h-full text-center opacity-0 pointer-events-none font-pusab text-stroke flex-center character-input"
                    in:moveToTitle|global={{
                        duration: CHARACTER_DURATION,
                        delay: GLOBAL_DELAY + i * CHARACTER_DELAY,
                        offcenter: i / TOTAL_ENDING_INPUTS,
                    }}
                    out:disappear
                >
                    {letters[i]}
                </span>
            </div>
        {/each}
    </div>
</div>

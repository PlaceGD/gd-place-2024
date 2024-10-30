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

    const characterCooldown = SyncedCooldown.new(
        `userDetails/${$loginData.currentUserData!.user.uid}/lastCharacterTimestamp`,
        CHARACTER_COOLDOWN_SECONDS
    );
    let {
        display: characterCooldownDisplay,
        finished: characterCooldownFinished,
    } = characterCooldown;

    let letters: (HTMLInputElement | null)[] =
        Array(TOTAL_ENDING_INPUTS).fill(null);

    let unsub: Unsubscribe | null;
    onMount(async () => {
        unsub = db.ref("/levelName/inputs").on("value", v => {});
    });

    onDestroy(() => {
        unsub?.();
    });
</script>

<div
    transition:fade
    class="absolute z-50 flex flex-col w-full h-full gap-4 pointer-events-none flex-center"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
>
    <div class="content-center justify-center ending-grid">
        {#each Array(TOTAL_ENDING_INPUTS) as _, i (i)}
            <Input
                maxLength={1}
                class="w-auto pointer-events-auto h- character-input text-stroke"
                hardValidInput={VALID_LEVEL_NAME}
                on:change={e => {
                    setLevelNameLetter({
                        index: i,
                        letter: e.detail,
                    });
                }}
                disabled={!$characterCooldownFinished}
            />
        {/each}
    </div>
    <div class="p-4 text-5xl text-white bg-black rounded-sm">
        {$characterCooldownDisplay}
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
        @apply text-center font-pusab text-7xl text-white !outline-none;

        box-shadow:
            0px 0px 0px 3px #989696,
            0px 0px 0px 6px #363535;
        background: linear-gradient(0deg, #181818 0%, #030303 100%);
    }

    @media only screen and (max-width: 1630px) {
        .ending-grid {
            grid-template-columns: repeat(
                calc(var(--count) / 2),
                var(--input-width)
            );
            grid-template-rows: repeat(2, var(--input-height));
        }
    }

    @media only screen and (max-width: 820px) {
        .ending-grid {
            grid-template-columns: repeat(
                calc(var(--count) / 4),
                var(--input-width)
            );
            grid-template-rows: repeat(4, var(--input-height));
        }
    }

    @media only screen and (max-width: 415px) {
        .ending-grid {
            --input-width: 75px;
        }

        :global(.character-input) {
            @apply text-6xl;
        }
    }
</style>

<script lang="ts">
    import { fade } from "svelte/transition";
    import Input from "../components/Input.svelte";
    import { TOTAL_ENDING_INPUTS, VALID_LEVEL_NAME } from "shared-lib/ending";
    import { setLevelNameLetter } from "../firebase/cloud_functions";
</script>

<div
    transition:fade
    class="absolute z-50 flex content-center justify-center w-full h-full pointer-events-none ending-grid"
    style={`--count: ${TOTAL_ENDING_INPUTS}`}
>
    {#each Array(TOTAL_ENDING_INPUTS) as _, i}
        <Input
            maxLength={1}
            class="w-auto h-full pointer-events-auto character-input text-stroke"
            hardValidInput={VALID_LEVEL_NAME}
            on:change={e => {
                setLevelNameLetter({
                    index: i,
                    letter: e.detail,
                });
            }}
        />
    {/each}
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

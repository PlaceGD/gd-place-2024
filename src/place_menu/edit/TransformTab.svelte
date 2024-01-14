<script lang="ts">
    import { TRANSFORM_BUTTONS } from "./edit_tab";
    import Image from "../../components/Image.svelte";
    import * as wasm from "wasm-lib";

    export let state: wasm.StateWrapper | null;
</script>

<ul
    class="w-full h-full gap-4 xs:gap-2 overflow-x-hidden overflow-y-scroll rounded-lg thin-scrollbar transform-grid-container"
>
    {#each TRANSFORM_BUTTONS as button, i (i)}
        <li class="w-16 h-16 md:w-14 md:h-14 xs:w-10 xs:h-10">
            <button
                class={"flex-center w-full h-full p-2 md:p-1.5 xs:p-1 z-20 rounded-md bg-button-green bounce-active"}
                on:click={() => {
                    if (state == null) return;
                    let obj = state.get_preview_object();
                    button.cb(obj);
                    state.set_preview_object(obj);
                }}
            >
                <Image
                    class="object-contain max-w-full max-h-full"
                    src={`/assets/ui/edit/${button.image}.svg`}
                    style={`transform: rotate(${button.angle}deg)${
                        button.flipped ? " scaleX(-1)" : ""
                    }`}
                    lazyLoad
                    skeleton
                />
            </button>
        </li>
    {/each}
</ul>

<style lang="postcss">
    .transform-grid-container {
        @apply grid justify-between p-4 md:p-3 xs:p-2;
        grid-template-columns: repeat(auto-fill, 64px);
    }

    @media screen(sm) {
        .transform-grid-container {
            grid-template-columns: repeat(auto-fill, 56px);
        }
    }

    @media screen(xs) {
        .transform-grid-container {
            grid-template-columns: repeat(auto-fill, 48px);
        }
    }
</style>

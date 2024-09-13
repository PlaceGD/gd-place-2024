<script lang="ts">
    import { SFX_TRIGGER_SOUNDS } from "shared-lib/nexusgen";
    import { menuMinimized, menuSelectedSFX } from "../../stores";
    import Image from "../../components/Image.svelte";
    import { playSound } from "../../utils/misc";
</script>

<div class="w-full h-full">
    <ul
        class="w-full h-full overflow-x-hidden overflow-y-scroll rounded-lg thin-scrollbar sfx-grid-container"
    >
        {#each SFX_TRIGGER_SOUNDS as sfx_name, id}
            <li class="relative w-16 h-16 md:w-12 md:h-12 xs:w-10 xs:h-10">
                <button
                    class={"absolute w-full h-full p-3 md:p-2 xs:p-1 z-20"}
                    tabindex={$menuMinimized ? -1 : 0}
                    on:click={() => {
                        // if (id == 3854) {
                        //     var audio = new Audio(fireMp3Url);
                        //     audio.volume = 0.02;
                        //     audio.play();
                        // }
                        playSound(
                            `/assets/audio/sfx/${SFX_TRIGGER_SOUNDS[id]}.ogg`,
                            0.5
                        );
                        $menuSelectedSFX = id;
                    }}
                >
                    <Image
                        src={`/assets/objects/sfx_icons/${sfx_name}.png`}
                        lazyLoad
                        class="object-contain w-full h-full"
                    />
                </button>
                {#if $menuSelectedSFX == id}
                    <span class="absolute w-full h-full sliding-selector"
                    ></span>
                {/if}
            </li>
        {/each}
    </ul>
</div>

<style lang="postcss">
    .sfx-grid-container {
        @apply grid justify-between p-4 md:p-3 xs:p-2;
        grid-template-columns: repeat(auto-fill, 64px);
    }

    @media screen(sm) {
        .sfx-grid-container {
            grid-template-columns: repeat(auto-fill, 56px);
        }
    }

    @media screen(xs) {
        .sfx-grid-container {
            grid-template-columns: repeat(auto-fill, 48px);
        }
    }
</style>

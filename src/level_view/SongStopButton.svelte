<script lang="ts">
    import Image from "../components/Image.svelte";
    import { songPlaying } from "../stores";
    import { stopSound } from "../utils/audio";

    import musicStop from "./assets/music_stop.png?url";

    import upArrowUrl from "../login/assets/arrow_up.svg?url";

    let hasStoppedMusicBefore = !!localStorage.getItem("hasStoppedMusicBefore");
</script>

{#if $songPlaying}
    <div class="flex align-center">
        <button
            class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12 stop-music-anim"
            on:click={() => {
                songPlaying.set(false);
                stopSound("preview song");
                stopSound("song");
                localStorage.setItem("hasStoppedMusicBefore", "true");
                hasStoppedMusicBefore = true;
            }}
        >
            <Image src={musicStop} class="object-contain aspect-square"></Image>
        </button>

        {#if !hasStoppedMusicBefore}
            <div
                class="absolute left-20 sm:left-16 xs:left:12 z-20 flex flex-row flex-left h-hover-anim opacity-70 self-center"
            >
                <Image src={upArrowUrl} class="w-12 h-12 rotate-[-90deg]"
                ></Image>
                <h1
                    class="top-0 text-2xl font-bold text-center self-center text-white pointer-events-none sm:text-xl xs:text-lg"
                >
                    Press to stop music
                </h1>
            </div>
        {/if}
    </div>
{/if}

<style lang="postcss">
    /* scale in and out ina sine wave */
    .stop-music-anim {
        animation: stop-music 0.7s infinite ease-in-out;
    }

    @keyframes stop-music {
        0% {
            transform: scale(1);
        }
        50% {
            transform: scale(1.13);
        }
        100% {
            transform: scale(1);
        }
    }

    @keyframes h-hover-anim {
        0% {
            transform: translateX(0);
        }
        50% {
            transform: translateX(0.5rem);
        }
        100% {
            transform: translateX(0);
        }
    }
    .h-hover-anim {
        animation: h-hover-anim 1.4s infinite ease-in-out;
    }
</style>

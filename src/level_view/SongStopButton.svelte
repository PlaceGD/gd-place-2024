<script lang="ts">
    import Image from "../components/Image.svelte";
    import { songPlaying } from "../stores";
    import { stopSound } from "../utils/audio";

    import musicStop from "./assets/music_stop.png?url";

    import upArrowUrl from "../login/assets/arrow_up.svg?url";

    let hasStoppedMusicBefore =
        localStorage.getItem("hasStoppedMusicBefore") === "true";
</script>

{#if $songPlaying}
    <div class="flex align-center">
        <div class="relative flex">
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
                <Image src={musicStop} class="object-contain aspect-square" />
            </button>

            {#if !hasStoppedMusicBefore}
                <div
                    class="absolute right-0 z-20 flex w-max flex-center h-hover-anim top-1/2 opacity-70"
                >
                    <Image
                        src={upArrowUrl}
                        class="w-10 h-10 rotate-[-90deg]"
                        style="filter: drop-shadow(0px 0px 5px rgba(0, 0, 0, 0.7))"
                    />
                    <h1
                        class="top-0 self-center text-2xl font-bold text-center text-white pointer-events-none sm:text-xl xs:text-lg"
                        style="text-shadow: 0px 0px 5px rgba(0, 0, 0, 0.7);"
                    >
                        Press <wbr />to<br />stop <wbr />music
                    </h1>
                </div>
            {/if}
        </div>
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
            transform: translate(100%, -50%);
        }
        50% {
            transform: translate(110%, -50%);
        }
        100% {
            transform: translate(100%, -50%);
        }
    }

    .h-hover-anim {
        animation: h-hover-anim 1.4s infinite ease-in-out;
    }
</style>

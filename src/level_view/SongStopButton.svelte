<script lang="ts">
    import Image from "../components/Image.svelte";
    import { songPlaying } from "../stores";
    import { stopSound } from "../utils/audio";

    import musicStop from "./assets/music_stop.png?url";
</script>

{#if $songPlaying}
    <button
        class="top-0 right-0 z-30 w-16 aspect-square sm:w-14 xs:w-12 stop-music-anim"
        on:click={() => {
            songPlaying.set(false);
            stopSound("preview song");
        }}
    >
        <Image
            src={musicStop}
            class="object-contain aspect-square"
        ></Image>
    </button>
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

</style>
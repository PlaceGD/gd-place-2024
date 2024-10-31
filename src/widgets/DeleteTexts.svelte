<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import { deleteTexts } from "../stores";
    import LevelWidget from "./LevelWidget.svelte";

    export let state;
</script>

{#each Object.entries($deleteTexts) as [k, { name, x, y }] (k)}
    <LevelWidget {state} {x} {y} scale={0.7}>
        <span
            class="absolute text-anim text-black font-bold text-saira text-stroke text-3xl text-nowrap w-[500px] text-center"
        >
            Deleted by <p>{name}</p>
        </span>
        <span
            class="absolute text-anim text-saira font-bold text-3xl text-nowrap w-[500px] text-center text-white"
        >
            Deleted by <ColoredName username={name}></ColoredName>
        </span>
    </LevelWidget>
{/each}

<style>
    @keyframes move-up {
        0% {
            transform: translateY(calc(0px - 50%)) scale(0.5);
            opacity: 100%;
        }
        80% {
            transform: translateY(calc(-30px - 50%)) scale(0.5);
            opacity: 0%;
        }
        100% {
            transform: translateY(calc(-30px - 50%)) scale(0.5);
            opacity: 0%;
        }
    }
    .text-anim {
        animation-name: move-up;
        animation-duration: 1.875s;
        animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
    }
</style>

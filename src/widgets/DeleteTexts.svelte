<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import { deleteTexts } from "../stores";
    import LevelWidget from "./LevelWidget.svelte";

    export let state;
</script>

{#each Object.entries($deleteTexts) as [k, { name, x, y }] (k)}
    <LevelWidget {state} {x} {y} scale={0.7}>
        <span
            class="absolute text-black font-bold text-saira text-stroke text-3xl text-nowrap w-[500px] text-center"
        >
            Deleted by <p>{name}</p>
        </span>
        <span
            class="absolute text-saira font-bold text-3xl text-nowrap w-[500px] text-center text-white"
        >
            Deleted by <ColoredName username={name}></ColoredName>
        </span>
    </LevelWidget>
{/each}

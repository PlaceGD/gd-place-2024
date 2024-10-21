<script lang="ts">
    import { triggerRuns } from "../stores";
    import LevelWidget from "./LevelWidget.svelte";
    export let state;
</script>

{#each Object.entries($triggerRuns) as [k, { x, y }] (k)}
    <LevelWidget {state} {x} {y}>
        <div class="absolute border-white rounded-full w-32 h-32 dog"></div>
    </LevelWidget>
{/each}

<style>
    @keyframes example {
        0% {
            transform: scale(0);
            border-width: 32px;
            opacity: 100%;
        }
        100% {
            transform: scale(1);
            border-width: 0px;
            opacity: 0%;
        }
    }
    .dog {
        animation: example 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        mix-blend-mode: plus-lighter;
    }
</style>

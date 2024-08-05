<script lang="ts">
    import { triggerRuns } from "../stores";
</script>

<div class="absolute text-white">
    {#each Object.entries($triggerRuns) as [k, { x, y }] (k)}
        <div
            class="text-nowrap"
            style={`
                position: absolute;
                transform: translate(${x}px, ${-y}px);
            `}
        >
            <!-- <span
                class="absolute letext text-black text-saira text-stroke text-3xl text-nowrap w-[500px] text-center"
                >Deleted by {name}</span
            >
            <span
                class="absolute letext text-saira text-3xl text-nowrap w-[500px] text-center"
                >Deleted by <ColoredName username={name}></ColoredName></span
            > -->
            <div class="absolute border-white rounded-full w-32 h-32 dog"></div>
        </div>
    {/each}
</div>

<style>
    @keyframes example {
        0% {
            transform: translate(-50%, -50%) scale(0);
            border-width: 32px;
            opacity: 100%;
        }
        100% {
            transform: translate(-50%, -50%) scale(1);
            border-width: 0px;
            opacity: 0%;
        }
    }
    .dog {
        animation: example 1s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        mix-blend-mode: plus-lighter;
    }
</style>

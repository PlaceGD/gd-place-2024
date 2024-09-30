<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";
    import { deleteTexts } from "../stores";
</script>

<div class="absolute text-white">
    {#each Object.entries($deleteTexts) as [k, { name, x, y }] (k)}
        <div
            class="text-nowrap"
            style={`
                position: absolute;
                transform: translate(${x / window.devicePixelRatio}px, ${-y / window.devicePixelRatio}px);
            `}
        >
            <span
                class="absolute letext text-black font-bold text-saira text-stroke text-3xl text-nowrap w-[500px] text-center"
                >Deleted by {name}</span
            >
            <span
                class="absolute letext text-saira font-bold text-3xl text-nowrap w-[500px] text-center"
                >Deleted by <ColoredName username={name}></ColoredName></span
            >
        </div>
    {/each}
</div>

<style>
    @keyframes example {
        0% {
            transform: translate(-50%, calc(0px - 50%)) scale(0.5);
            opacity: 100%;
        }
        80% {
            transform: translate(-50%, calc(-30px - 50%)) scale(0.5);
            opacity: 0%;
        }
        100% {
            transform: translate(-50%, calc(-30px - 50%)) scale(0.5);
            opacity: 0%;
        }
    }
    .letext {
        animation-name: example;
        animation-duration: 1.875s;
        animation-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
    }
</style>

<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";

    export let username: string;
</script>

<div
    class="px-4 py-2 text-xl text-center text-white -translate-y-8 rounded-md bg-menu-gray/60 backdrop-blur-sm"
>
    Placed by
    <span class="inline-block font-bold"><ColoredName {username} /></span>
</div>

<style>
    /* @keyframes example {
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
    } */
</style>

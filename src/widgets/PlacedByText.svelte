<script lang="ts">
    import ColoredName from "../components/ColoredName.svelte";

    export let username: string;
</script>

<div
    class="text-white bg-menu-gray/60 rounded-md text-xl -translate-y-16 backdrop-blur-sm px-4 py-2"
>
    Placed by <span class="font-bold"><ColoredName {username} /></span>
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

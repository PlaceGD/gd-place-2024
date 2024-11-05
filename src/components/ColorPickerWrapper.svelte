<script>
    import { portal } from "svelte-portal";

    export let wrapper;
    export let isOpen;
    export let isDialog;
</script>

<div
    id="gradient-color-picker"
    bind:this={wrapper}
    class="wrapper"
    class:is-open={isOpen}
    role={isDialog ? "dialog" : undefined}
    aria-label="Pick Color"
>
    <slot />
</div>

<style>
    .wrapper {
        padding: 8px;
        background-color: black;
        margin: 0 10px 10px;
        border-radius: 12px;
        display: none;
        width: max-content;
    }
    .is-open {
        display: inline-block;
    }
    [role="dialog"] {
        position: absolute;
        z-index: 100;
    }
</style>

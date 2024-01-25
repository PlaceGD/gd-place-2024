<script lang="ts">
    import type { SliderMethods } from "../login";

    export let slider: SliderMethods;
</script>

<div class="w-full h-full text-center flex-center">
    <p>HERE IS THE TOS PLEASE READ</p>
    <button on:click={() => slider.previous()}>Ok</button>
</div>

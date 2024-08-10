<script lang="ts">
    export let max: number;
    export let remaining: number;

    const gradientFunc = (t: number) =>
        `conic-gradient(white ${t * 360}deg, black ${t * 360}deg 360deg)`;

    $: gradientString = gradientFunc(1 - remaining / max);
</script>

<div class="relative w-full h-full">
    <slot />
    {#if remaining > 0}
        <div
            class="absolute bottom-0 right-0 w-4 h-4 border-2 border-white rounded-full translate-x-1/4 translate-y-1/4"
            style:background={gradientString}
        ></div>
    {/if}
</div>

<style>
</style>

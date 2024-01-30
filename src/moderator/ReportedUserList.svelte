<script lang="ts">
    import { onMount } from "svelte";
    import { showModeratorOptions } from "../stores";

    let container: HTMLDivElement | null = null;

    $: {
        if ($showModeratorOptions && container != null) {
            container.focus();

            container.addEventListener("focusout", () => {
                console.log("here");
            });
        }
    }
</script>

{#if $showModeratorOptions}
    <div bind:this={container}></div>
{/if}

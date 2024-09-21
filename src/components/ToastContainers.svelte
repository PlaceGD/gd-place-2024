<script lang="ts">
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import Portal from "svelte-portal";
    import { toastPortals } from "../utils/toast";
</script>

<div bind:this={$toastPortals[$toastPortals.length]}></div>

<Portal target={$toastPortals.at(-1) ?? undefined}>
    <SvelteToast
        options={{
            intro: { y: -64 },
        }}
    />
</Portal>

<div class="announcement-wrapper">
    <SvelteToast target="announcement" options={{ intro: { y: -50 } }} />
</div>

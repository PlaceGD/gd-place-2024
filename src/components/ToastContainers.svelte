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
            classes: ["log"],
        }}
    />
</Portal>

<div class="announcement-wrapper">
    <SvelteToast target="announcement" options={{ intro: { y: -50 } }} />
</div>

<style lang="postcss">
    :global(._toastContainer) {
        font-family: Saira;
        color: white;
        border-radius: 8px;
    }

    @media screen(lg) {
        :global(._toastContainer) {
            font-size: 14px;
        }
    }

    @media screen(sm) {
        :global(._toastContainer) {
            font-size: 12px;
        }
    }
    :global(._toastItem) {
        backdrop-filter: blur(10px);
    }
</style>

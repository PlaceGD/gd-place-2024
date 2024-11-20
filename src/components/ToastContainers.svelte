<script lang="ts">
    import { SvelteToast } from "@zerodevx/svelte-toast";
    import Portal from "svelte-portal";
    import { toastPortals } from "../utils/toast";
</script>

<div bind:this={$toastPortals[$toastPortals.length]}></div>

<Portal target={$toastPortals.at(-1) ?? undefined}>
    <div class="toast-wrapper">
        <SvelteToast
            options={{
                intro: { y: -64 },
                classes: ["log"],
            }}
        />
    </div>
</Portal>

<div class="toast-wrapper">
    <SvelteToast
        target="announcement"
        options={{ intro: { y: -50 }, classes: ["announcement"] }}
    />
</div>

<style lang="postcss">
    :global(.log.info) {
        --toastBackground: #396196b2;
        --toastBarBackground: #5b82b5;
    }
    :global(.log.success) {
        --toastBackground: #449639b2;
        --toastBarBackground: #6ab55b;
    }
    :global(.log.warning) {
        --toastBackground: #966e39b2;
        --toastBarBackground: #b58d5b;
    }
    :global(.log.error) {
        --toastBackground: #963939b2;
        --toastBarBackground: #b55f5b;
    }

    .toast-wrapper {
        --toastContainerTop: 2rem;
        --toastContainerLeft: 0;
        --toastWidth: min(max-content, 50%);
    }

    .toast-wrapper > :global(._toastContainer) {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    :global(.announcement) {
        --toastBackground: #2d597bb2;
        --toastBarBackground: #234863;
        padding: 8px;
    }

    :global(._toastContainer) {
        padding: 8px;
        --toastColor: white;
        --toastBorderRadius: 0.5rem;
        --toastPadding: 8px;
    }

    :global(._toastItem) {
        backdrop-filter: blur(10px);
        pointer-events: all;
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

    @media screen(xs) {
        :global(._toastContainer) {
            font-size: 10px;
        }
    }
</style>

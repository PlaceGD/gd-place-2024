<script lang="ts">
    import { default as cx } from "classnames";
    import ToastContainers from "./ToastContainers.svelte";
    import Cross from "../icons/Cross.svelte";
    import Loading from "../components/Loading.svelte";
    import { createEventDispatcher, onDestroy, onMount } from "svelte";
    import { toastPortals } from "../utils/toast";
    import { get } from "svelte/store";

    let modal: HTMLDialogElement | null = null;

    // let prevToastPortal: HTMLElement | null = null;
    let toastContainer: HTMLElement;

    export let hasCloseButton = false;

    export let canClose = true;
    export let isOpen = false;
    export let state: "default" | "loading" = "default";
    export let label: string = "";
    export let size: string = "max-w-[500px] max-h-[500px]";

    let dispatcher = createEventDispatcher();

    onMount(() => {
        modal?.close();
    });

    // used so we can listen on the intro/outro events
    const fakeTransition = (_: HTMLElement) => {
        return {
            duration: 0,
            css: () => {
                return "";
            },
        };
    };

    const handleClose = () => {
        dispatcher("close");
        modal?.close();

        $toastPortals.pop();
        $toastPortals = $toastPortals;
    };
</script>

{#if isOpen}
    <dialog
        aria-label={label}
        class="flex flex-col w-full h-full gap-4 p-6 overflow-visible text-white bg-transparent pointer-events-auto xs:gap-2 aspect-square open:flex xs:p-4 flex-center modal-dialog"
        bind:this={modal}
        on:cancel={e => {
            if (canClose) {
                handleClose();
            } else {
                e.preventDefault();
            }
        }}
        transition:fakeTransition
        on:introstart={() => {
            modal?.showModal();
            $toastPortals = [...$toastPortals, toastContainer];
        }}
        on:outrostart={handleClose}
    >
        <div bind:this={toastContainer}></div>

        <div class={`flex-auto menu-panel w-full h-full relative ${size}`}>
            {#if state === "loading"}
                <Loading class="rounded-xl" />
            {/if}
            <slot />
        </div>

        {#if hasCloseButton}
            <div class="flex items-center h-12 text-white xs:h-10 flex-center">
                <div class="h-full">
                    <button
                        disabled={!canClose}
                        class={cx({
                            "flex-col h-full p-1 rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200": true,
                            "text-disabled-white pointer-events-none":
                                !canClose,
                        })}
                        aria-label="Close"
                        on:click={() => {
                            isOpen = false;
                        }}
                    >
                        <Cross
                            aria-label="Close"
                            class="w-full h-full stroke-1"
                        />
                    </button>
                </div>
            </div>
        {/if}
    </dialog>
{/if}

<style lang="postcss">
    .modal-dialog::backdrop {
        @apply brightness-[65%] backdrop-blur-lg;
    }
</style>

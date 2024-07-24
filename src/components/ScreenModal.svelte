<script lang="ts">
    import { default as cx } from "classnames";
    import ToastContainer from "./ToastContainer.svelte";
    import Cross from "../icons/cross.svg";
    import Loading from "../components/Loading.svelte";
    import { createEventDispatcher } from "svelte";

    let modal: HTMLDialogElement | null = null;

    export let hasCloseButton = false;

    export let canClose = true;
    export let isOpen = false;
    export let state: "default" | "loading" = "default";
    export let label: string = "";

    let dispatcher = createEventDispatcher();

    $: {
        if (isOpen) {
            modal?.showModal();
        } else {
            modal?.close();
        }
    }
</script>

<!-- <svelte:window
    on:keyup={e => {
        console.log("HERE");
        if (canClose && e.key === "Escape") {
            modal?.close();
        } else {
            e.preventDefault();
        }
    }}
/> -->

<dialog
    aria-label={label}
    class="flex w-full h-full p-0 overflow-visible pointer-events-auto dialog-panel flex-center"
    bind:this={modal}
    on:close={() => dispatcher("close")}
    on:cancel={e => {
        if (canClose) {
            modal?.close();
        } else {
            e.preventDefault();
        }
    }}
>
    <ToastContainer />

    <div class="flex-auto menu-panel w-full max-w-[600px] max-h-[600px]">
        <slot />
        {#if state === "loading"}
            <Loading class="rounded-xl" />
        {/if}
    </div>

    {#if hasCloseButton}
        <div class="flex items-center h-12 text-white xs:h-10 flex-center">
            <div class="h-full">
                <button
                    disabled={!canClose}
                    class={cx({
                        "flex-col h-full p-1 rounded-lg flex-center menu-panel hover:brightness-150 active:brightness-200": true,
                        "text-disabled-white pointer-events-none": !canClose,
                    })}
                    aria-label="Close"
                    on:click={() => {
                        isOpen = false;
                    }}
                >
                    <Cross alt="Close" class="w-full h-full"></Cross>
                </button>
            </div>
        </div>
    {/if}
</dialog>

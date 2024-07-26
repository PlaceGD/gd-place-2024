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

<dialog
    aria-label={label}
    class="flex flex-col w-full h-full gap-4 p-6 overflow-visible text-white bg-transparent pointer-events-auto xs:gap-2 aspect-square open:flex xs:p-4 flex-center"
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

    <div class="flex-auto menu-panel w-full max-w-[500px] max-h-[500px] h-full">
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

<script lang="ts">
    import { default as cx } from "classnames";
    import ToastContainer from "./ToastContainer.svelte";
    import { IconX as Cross } from "@tabler/icons-svelte";
    import Loading from "../components/Loading.svelte";
    import { createEventDispatcher, onMount } from "svelte";

    let modal: HTMLDialogElement | null = null;

    export let hasCloseButton = false;

    export let canClose = true;
    export let isOpen = false;
    export let state: "default" | "loading" = "default";
    export let label: string = "";
    export let size: string = "max-w-[500px] max-h-[500px]";

    let dispatcher = createEventDispatcher();

    onMount(() => {
        console.log(modal);
        modal?.close();
    });

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
    class={cx({
        "flex-col w-full h-full gap-4 p-6 overflow-visible text-white bg-transparent pointer-events-auto xs:gap-2 aspect-square open:flex xs:p-4 flex-center": true,
        flex: isOpen,
        hidden: !isOpen,
    })}
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


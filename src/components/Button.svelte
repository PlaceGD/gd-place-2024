<script lang="ts">
    import { default as cx } from "classnames";
    import { createEventDispatcher } from "svelte";
    import Check from "../icons/Check.svelte";
    import Cross from "../icons/Cross.svelte";

    export let type: "accept" | "decline" | "white" | "plain" = "white";
    export let iconClass = "";
    export let disabled = false;

    const dispatcher = createEventDispatcher();

    const { class: _class, disabled: _dis, ...restProps } = $$restProps;
</script>

<button
    class={cx({
        [`rounded-lg relative flex flex-center ${$$restProps["class"]}`]: true,
        "white-button outline-2 outline outline-white/20 -outline-offset-2 gap-1 px-2":
            type !== "plain",
        "grid-cols-[min-content_1fr]": type === "accept" || type === "decline",
        "grid-cols-[1fr]": type === "plain" || type === "white",
    })}
    on:click={() => dispatcher("click")}
    {disabled}
    {...restProps}
>
    {#if type == "accept"}
        <Check
            class="stroke-[1.5] text-[#47ff47] xs:w-9 xs:h-9 w-10 h-10 p-1 {iconClass}"
        />
    {:else if type == "decline"}
        <Cross
            class="stroke-[1.5] text-[#ff4747] xs:w-9 xs:h-9 w-10 h-10 p-1 {iconClass}"
        />
    {/if}

    <slot />
</button>
